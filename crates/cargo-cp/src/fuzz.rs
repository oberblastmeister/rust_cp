use anyhow::{Context, Result, bail};
use argh::FromArgs;
use std::process::{Command, Stdio};

/// Run a fuzz target with the nightly Rust toolchain.
#[derive(Debug, FromArgs)]
#[argh(subcommand, name = "fuzz")]
pub(crate) struct Fuzz {
    /// cargo-fuzz target to run
    #[argh(positional, arg_name = "TARGET")]
    pub(crate) target: String,
}

impl Fuzz {
    pub(crate) fn run(self) -> Result<()> {
        let status = cargo_fuzz_command(&self.target)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
            .context(
                "failed to run `cargo +nightly fuzz`; are Rust nightly and cargo-fuzz installed?",
            )?;

        if !status.success() {
            bail!(
                "`cargo +nightly fuzz run {}` failed with {status}",
                self.target
            );
        }

        Ok(())
    }
}

fn cargo_fuzz_command(target: &str) -> Command {
    let mut command = Command::new("cargo");
    command.args(["+nightly", "fuzz", "run", target]);
    command
}

#[cfg(test)]
mod tests {
    use super::cargo_fuzz_command;
    use std::ffi::OsStr;

    #[test]
    fn runs_cargo_fuzz_with_nightly() {
        let command = cargo_fuzz_command("parser");

        assert_eq!(command.get_program(), OsStr::new("cargo"));
        assert_eq!(
            command.get_args().collect::<Vec<_>>(),
            ["+nightly", "fuzz", "run", "parser"]
                .map(OsStr::new)
                .as_slice()
        );
    }
}
