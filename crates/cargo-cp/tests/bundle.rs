use cargo_cp::bundle;
use std::path::PathBuf;
use std::process::Command;

fn manifest_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

#[test]
fn preserves_the_solution_source_exactly() {
    let path = manifest_dir().join("tests/fixtures/source/main.rs");
    let original = std::fs::read_to_string(&path).unwrap();
    let source = bundle(&path).unwrap();

    assert_eq!(source, original);
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
    let solution = manifest_dir().join("../solutions/src/chicken_jockey.rs");
    let original = std::fs::read_to_string(&solution).unwrap();
    let source = bundle(&solution).unwrap();
    assert!(source.contains("# [allow (warnings)] mod cp_library {"));
    assert!(source.ends_with(&original));
    assert!(source.contains("mod cio {"));
    assert!(source.contains("pub struct Cin"));
    assert!(!source.contains("mod dsu {"));
    assert!(!source.contains("mod itertools {"));
    assert!(!source.contains("mod kruskal {"));
    assert!(!source.contains("mod seg_tree {"));

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
fn prelude_glob_only_bundles_prelude_dependencies() {
    let solution = manifest_dir().join("tests/fixtures/source/prelude.rs");
    let source = bundle(&solution).unwrap();

    assert!(source.contains("pub mod prelude {"));
    assert!(source.contains("mod cio {"));
    assert!(source.contains("mod driver {"));
    assert!(source.contains("mod itertools {"));
    assert!(!source.contains("mod algebra {"));
    assert!(!source.contains("mod dsu {"));
    assert!(!source.contains("mod prefix_sum {"));
    assert!(!source.contains("mod segtree {"));

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
fn rejects_non_prelude_glob_imports() {
    let solution = manifest_dir().join("tests/fixtures/source/unsupported_glob.rs");
    let error = bundle(&solution).unwrap_err();

    assert!(
        format!("{error:#}").contains("only imports ending in `prelude::*` can be bundled"),
        "unexpected error: {error:#}"
    );
    assert!(
        format!("{error:#}").contains("unsupported glob import `std::collections::*`"),
        "unexpected error: {error:#}"
    );
}

#[test]
fn follows_transitive_dependencies_between_library_modules() {
    let solution = manifest_dir().join("tests/fixtures/source/kruskal.rs");
    let source = bundle(&solution).unwrap();

    assert!(source.contains("mod prefix_sum {"));
    assert!(source.contains("mod algebra {"));
    assert!(source.contains("use crate :: cp_library :: algebra :: Group ;"));
    assert!(!source.contains("mod cio {"));
    assert!(!source.contains("mod dsu {"));
    assert!(!source.contains("mod itertools {"));
    assert!(!source.contains("mod seg_tree {"));

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
