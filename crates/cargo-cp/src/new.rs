use anyhow::{Context, Result, bail};
use argh::FromArgs;
use cargo_metadata::MetadataCommand;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

static SOLUTION_TEMPLATE: &str = r#"use cp_library::prelude::*;

fn solve() {
    
}

fn run(_: usize, cin: &mut Cin, cout: &mut Cout) {
    
}

pub fn main() {
    driver(run, TestKind::Many);
}

#[cfg(test)]
mod tests {
    use insta::assert_snapshot;

    use super::*;

    #[test]
    fn smoke() {
        assert_snapshot!(test_driver(run, TestKind::Many, "
"),
        @"
")
    }
}

"#;

/// Create a solution from the embedded template.
#[derive(Debug, FromArgs)]
#[argh(subcommand, name = "new")]
pub(crate) struct New {
    /// solution module name (the .rs extension is appended automatically)
    #[argh(positional, arg_name = "FILE")]
    pub(crate) file: String,
}

impl New {
    pub(crate) fn run(self) -> Result<()> {
        let workspace_root = workspace_root(Path::new("."))?;
        create_solution(&workspace_root, &self.file)
    }
}

fn workspace_root(current_dir: &Path) -> Result<PathBuf> {
    let metadata = MetadataCommand::new()
        .current_dir(current_dir)
        .no_deps()
        .exec()
        .context("failed to run `cargo metadata`; are you inside the workspace?")?;
    Ok(metadata.workspace_root.into_std_path_buf())
}

fn create_solution(workspace_root: &Path, file: &str) -> Result<()> {
    let module = module_name(file)?;
    let solutions_dir = workspace_root.join("crates/solutions/src");
    let output_path = solutions_dir.join(format!("{module}.rs"));
    let lib_path = solutions_dir.join("lib.rs");

    let lib = fs::read_to_string(&lib_path)
        .with_context(|| format!("failed to read `{}`", lib_path.display()))?;
    let updated_lib = add_module(&lib, module)?;

    let mut output =
        OpenOptions::new().write(true).create_new(true).open(&output_path).with_context(|| {
            format!("failed to create `{}`; does it already exist?", output_path.display())
        })?;
    if let Err(error) = output.write_all(SOLUTION_TEMPLATE.as_bytes()) {
        drop(output);
        let _ = fs::remove_file(&output_path);
        return Err(error).with_context(|| format!("failed to write `{}`", output_path.display()));
    }

    if let Err(error) = fs::write(&lib_path, updated_lib) {
        drop(output);
        let _ = fs::remove_file(&output_path);
        return Err(error).with_context(|| format!("failed to update `{}`", lib_path.display()));
    }

    eprintln!("created {} and added `pub mod {module};`", output_path.display());
    Ok(())
}

fn module_name(file: &str) -> Result<&str> {
    if file.is_empty() || file.contains(['/', '\\']) || syn::parse_str::<syn::Ident>(file).is_err()
    {
        bail!("`{file}` is not a valid Rust module name");
    }
    Ok(file)
}

fn add_module(lib: &str, module: &str) -> Result<String> {
    let declaration = format!("pub mod {module};");
    if lib.lines().any(|line| line.trim() == declaration) {
        bail!("module `{module}` is already declared in lib.rs");
    }

    let mut lines: Vec<_> = lib.lines().collect();
    let insertion = lines
        .iter()
        .position(|line| {
            line.trim_start().starts_with("pub mod ") && line.trim() > declaration.as_str()
        })
        .unwrap_or(lines.len());
    lines.insert(insertion, &declaration);

    let mut updated = lines.join("\n");
    if lib.ends_with('\n') {
        updated.push('\n');
    }
    Ok(updated)
}

#[cfg(test)]
mod tests {
    use super::{SOLUTION_TEMPLATE, add_module, create_solution, module_name};
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn accepts_a_filename_stem() {
        assert_eq!(module_name("new_problem").unwrap(), "new_problem");
    }

    #[test]
    fn rejects_paths_and_invalid_identifiers() {
        assert!(module_name("nested/problem").is_err());
        assert!(module_name("new_problem.rs").is_err());
        assert!(module_name("two-strings").is_err());
        assert!(module_name("mod").is_err());
    }

    #[test]
    fn inserts_module_in_sorted_order() {
        let lib = "pub mod alpha;\npub mod gamma;\n";
        assert_eq!(
            add_module(lib, "beta").unwrap(),
            "pub mod alpha;\npub mod beta;\npub mod gamma;\n"
        );
    }

    #[test]
    fn creates_the_embedded_template_and_updates_lib() {
        let directory = tempdir().unwrap();
        let solutions = directory.path().join("crates/solutions/src");
        fs::create_dir_all(&solutions).unwrap();
        fs::write(solutions.join("lib.rs"), "pub mod alpha;\n").unwrap();

        create_solution(directory.path(), "beta").unwrap();

        assert_eq!(fs::read_to_string(solutions.join("beta.rs")).unwrap(), SOLUTION_TEMPLATE);
        assert_eq!(
            fs::read_to_string(solutions.join("lib.rs")).unwrap(),
            "pub mod alpha;\npub mod beta;\n"
        );
    }

    #[test]
    fn does_not_overwrite_an_existing_solution() {
        let directory = tempdir().unwrap();
        let solutions = directory.path().join("crates/solutions/src");
        fs::create_dir_all(&solutions).unwrap();
        fs::write(solutions.join("existing.rs"), "keep me\n").unwrap();
        fs::write(solutions.join("lib.rs"), "pub mod alpha;\n").unwrap();

        assert!(create_solution(directory.path(), "existing").is_err());
        assert_eq!(fs::read_to_string(solutions.join("existing.rs")).unwrap(), "keep me\n");
        assert_eq!(fs::read_to_string(solutions.join("lib.rs")).unwrap(), "pub mod alpha;\n");
    }
}
