use anyhow::{Context, Result, bail};
use argh::FromArgs;
use cargo_metadata::MetadataCommand;
use std::env;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// Minimize a failing fuzz input.
#[derive(Debug, FromArgs)]
#[argh(subcommand, name = "tmin")]
pub(crate) struct Tmin {
    /// cargo-fuzz target to run
    #[argh(positional, arg_name = "TARGET")]
    pub(crate) target: String,

    /// failing input to minimize
    #[argh(positional, arg_name = "INPUT")]
    pub(crate) input: PathBuf,
}

impl Tmin {
    pub(crate) fn run(self) -> Result<()> {
        let fuzz_dir = fuzz_directory(Path::new("."))?;
        let input = resolve_input(self.input)?;
        let status = cargo_tmin_command(&self.target, &input, &fuzz_dir)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
            .context(
                "failed to run `cargo +nightly fuzz tmin`; are Rust nightly and cargo-fuzz installed?",
            )?;

        if !status.success() {
            bail!(
                "`cargo +nightly fuzz tmin {} {}` failed with {status}",
                self.target,
                input.display()
            );
        }

        Ok(())
    }
}

fn fuzz_directory(current_dir: &Path) -> Result<PathBuf> {
    let metadata = MetadataCommand::new()
        .current_dir(current_dir)
        .no_deps()
        .exec()
        .context("failed to run `cargo metadata`; are you inside the workspace?")?;
    Ok(metadata
        .workspace_root
        .join("crates/fuzz")
        .into_std_path_buf())
}

fn resolve_input(input: PathBuf) -> Result<PathBuf> {
    let input = if input.is_absolute() {
        input
    } else {
        env::current_dir()
            .context("failed to determine the current directory")?
            .join(input)
    };
    input
        .canonicalize()
        .with_context(|| format!("failed to find fuzz input `{}`", input.display()))
}

fn cargo_tmin_command(target: &str, input: &Path, fuzz_dir: &Path) -> Command {
    let mut command = Command::new("cargo");
    command
        .args(["+nightly", "fuzz", "tmin", target])
        .arg(input)
        .arg("--fuzz-dir")
        .arg(fuzz_dir)
        .current_dir(fuzz_dir);
    command
}

#[cfg(test)]
mod tests {
    use super::cargo_tmin_command;
    use std::ffi::OsStr;
    use std::path::Path;

    #[test]
    fn minimizes_with_nightly_and_the_relocated_fuzz_directory() {
        let command = cargo_tmin_command(
            "parser",
            Path::new("/tmp/crash-input"),
            Path::new("crates/fuzz"),
        );

        assert_eq!(command.get_program(), OsStr::new("cargo"));
        assert_eq!(
            command.get_args().collect::<Vec<_>>(),
            [
                OsStr::new("+nightly"),
                OsStr::new("fuzz"),
                OsStr::new("tmin"),
                OsStr::new("parser"),
                OsStr::new("/tmp/crash-input"),
                OsStr::new("--fuzz-dir"),
                OsStr::new("crates/fuzz"),
            ]
        );
        assert_eq!(command.get_current_dir(), Some(Path::new("crates/fuzz")));
    }
}
