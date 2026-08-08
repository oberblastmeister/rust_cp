use cargo_cp::bundle;
use std::path::PathBuf;
use std::process::Command;

fn manifest_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

#[test]
fn recursively_expands_file_modules() {
    let source = bundle(&manifest_dir().join("tests/fixtures/source/main.rs")).unwrap();
    assert!(source.contains("mod nested {"));
    assert!(source.contains("mod value {"));
    assert!(!source.contains("mod nested;"));
    assert!(!source.contains("mod value;"));
    assert!(!source.contains("mod unused"));
    assert!(!source.contains("unused modules must not be bundled"));
    syn::parse_file(&source).unwrap();
}

#[test]
fn rejects_referenced_crates_io_dependencies() {
    let error = bundle(&manifest_dir().join("tests/fixtures/source/external.rs")).unwrap_err();
    assert!(
        error
            .to_string()
            .contains("cannot bundle external dependency `anyhow`"),
        "unexpected error: {error:#}"
    );
}

#[test]
fn embeds_a_referenced_workspace_library_and_compiles() {
    let solution = manifest_dir().join("../../solutions/src/bin/range_sum.rs");
    let source = bundle(&solution).unwrap();
    assert!(source.contains("#[allow(warnings)]\nmod cp_library {"));
    assert!(source.contains("mod fenwick {"));
    assert!(source.contains("mod io {"));
    assert!(source.contains("pub struct Fenwick"));
    assert!(!source.contains("mod dsu {"));
    assert!(!source.contains("mod itertools {"));
    assert!(!source.contains("mod kruskal {"));
    assert!(!source.contains("mod segment_tree {"));

    let directory = tempfile::tempdir().unwrap();
    let submission = directory.path().join("submission.rs");
    std::fs::write(&submission, source).unwrap();
    let result = Command::new("rustc")
        .args(["--edition=2024", "-Dwarnings", "-o"])
        .arg(directory.path().join("submission"))
        .arg(&submission)
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "rustc failed:\n{}",
        String::from_utf8_lossy(&result.stderr)
    );
}

#[test]
fn follows_transitive_dependencies_between_library_modules() {
    let solution = manifest_dir().join("tests/fixtures/source/kruskal.rs");
    let source = bundle(&solution).unwrap();

    assert!(source.contains("mod kruskal {"));
    assert!(source.contains("mod dsu {"));
    assert!(source.contains("use crate::cp_library::Dsu;"));
    assert!(!source.contains("mod fenwick {"));
    assert!(!source.contains("mod io {"));
    assert!(!source.contains("mod itertools {"));
    assert!(!source.contains("mod segment_tree {"));

    let directory = tempfile::tempdir().unwrap();
    let submission = directory.path().join("submission.rs");
    std::fs::write(&submission, source).unwrap();
    let result = Command::new("rustc")
        .args(["--edition=2024", "-Dwarnings", "-o"])
        .arg(directory.path().join("submission"))
        .arg(&submission)
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "rustc failed:\n{}",
        String::from_utf8_lossy(&result.stderr)
    );
}
