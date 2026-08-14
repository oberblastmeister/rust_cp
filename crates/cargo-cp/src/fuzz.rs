use anyhow::{Context, Result, bail};
use argh::FromArgs;
use cargo_metadata::MetadataCommand;
use std::env;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// Run a fuzz target with the nightly Rust toolchain.
#[derive(Debug, FromArgs)]
#[argh(subcommand, name = "fuzz")]
pub(crate) struct Fuzz {
    /// cargo-fuzz target to run
    #[argh(positional, arg_name = "TARGET")]
    pub(crate) target: String,

    /// failing input to reproduce
    #[argh(positional, arg_name = "INPUT")]
    pub(crate) input: Option<PathBuf>,
}

impl Fuzz {
    pub(crate) fn run(self) -> Result<()> {
        let fuzz_dir = fuzz_directory(Path::new("."))?;
        let input = self.input.map(resolve_input).transpose()?;
        let status = cargo_fuzz_command(&self.target, input.as_deref(), &fuzz_dir)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
            .context(
                "failed to run `cargo +nightly fuzz`; are Rust nightly and cargo-fuzz installed?",
            )?;

        if !status.success() {
            bail!("`cargo +nightly fuzz run {}` failed with {status}", self.target);
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
    Ok(metadata.workspace_root.join("crates/fuzz").into_std_path_buf())
}

fn resolve_input(input: PathBuf) -> Result<PathBuf> {
    let input = if input.is_absolute() {
        input
    } else {
        env::current_dir().context("failed to determine the current directory")?.join(input)
    };
    input.canonicalize().with_context(|| format!("failed to find fuzz input `{}`", input.display()))
}

fn cargo_fuzz_command(target: &str, input: Option<&Path>, fuzz_dir: &Path) -> Command {
    let mut command = Command::new("cargo");
    command.args(["+nightly", "fuzz", "run", target]);
    if let Some(input) = input {
        command.arg(input);
    }
    command.arg("--fuzz-dir").arg(fuzz_dir).current_dir(fuzz_dir);
    command
}

#[cfg(test)]
mod tests {
    use super::{cargo_fuzz_command, fuzz_directory};
    use std::ffi::OsStr;
    use std::path::{Path, PathBuf};

    #[test]
    fn runs_cargo_fuzz_with_nightly() {
        let command = cargo_fuzz_command("parser", None, Path::new("crates/fuzz"));

        assert_eq!(command.get_program(), OsStr::new("cargo"));
        assert_eq!(
            command.get_args().collect::<Vec<_>>(),
            [
                OsStr::new("+nightly"),
                OsStr::new("fuzz"),
                OsStr::new("run"),
                OsStr::new("parser"),
                OsStr::new("--fuzz-dir"),
                OsStr::new("crates/fuzz"),
            ]
        );
        assert_eq!(command.get_current_dir(), Some(Path::new("crates/fuzz")));
    }

    #[test]
    fn reproduces_a_specific_failing_input() {
        let command = cargo_fuzz_command(
            "parser",
            Some(Path::new("/tmp/crash-input")),
            Path::new("crates/fuzz"),
        );

        assert_eq!(
            command.get_args().collect::<Vec<_>>(),
            [
                OsStr::new("+nightly"),
                OsStr::new("fuzz"),
                OsStr::new("run"),
                OsStr::new("parser"),
                OsStr::new("/tmp/crash-input"),
                OsStr::new("--fuzz-dir"),
                OsStr::new("crates/fuzz"),
            ]
        );
    }

    #[test]
    fn finds_the_fuzz_crate_in_the_workspace() {
        assert_eq!(
            fuzz_directory(Path::new(env!("CARGO_MANIFEST_DIR"))).unwrap(),
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../fuzz").canonicalize().unwrap()
        );
    }
}
