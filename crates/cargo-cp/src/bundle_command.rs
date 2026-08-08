use anyhow::{Context, Result};
use argh::FromArgs;
use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};

/// Write a single-file submission.
#[derive(Debug, FromArgs)]
#[argh(subcommand, name = "bundle")]
pub(crate) struct Bundle {
    /// rust source file to bundle
    #[argh(positional, arg_name = "FILE")]
    pub(crate) input: PathBuf,

    /// output file (default: <FILE>_bundled.rs)
    #[argh(option, short = 'o', arg_name = "FILE")]
    pub(crate) output: Option<PathBuf>,
}

impl Bundle {
    pub(crate) fn run(self) -> Result<()> {
        let Self { input, output } = self;
        let output = output.unwrap_or_else(|| default_output_path(&input));
        let bundled = cargo_cp::bundle(&input)?;
        fs::write(&output, bundled)
            .with_context(|| format!("failed to write `{}`", output.display()))?;
        eprintln!("bundled {} -> {}", input.display(), output.display());
        Ok(())
    }
}

fn default_output_path(input: &Path) -> PathBuf {
    let stem = input.file_stem().unwrap_or(input.as_os_str());
    let mut filename = OsString::from(stem);
    filename.push("_bundled.rs");
    input.with_file_name(filename)
}

#[cfg(test)]
mod tests {
    use super::default_output_path;
    use std::path::PathBuf;

    #[test]
    fn appends_bundled_to_the_file_stem() {
        assert_eq!(
            default_output_path(&PathBuf::from("solutions/example.rs")),
            PathBuf::from("solutions/example_bundled.rs")
        );
    }
}
