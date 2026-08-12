use anyhow::{Context, Result};
use argh::FromArgs;
use cargo_metadata::MetadataCommand;
use std::fs;
use std::path::{Path, PathBuf};

/// Write a single-file submission.
#[derive(Debug, FromArgs)]
#[argh(subcommand, name = "bundle")]
pub(crate) struct Bundle {
    /// rust source file to bundle
    #[argh(positional, arg_name = "FILE")]
    pub(crate) input: PathBuf,

    /// output file (default: crates/bundled/src/bin/<FILE>)
    #[argh(option, short = 'o', arg_name = "FILE")]
    pub(crate) output: Option<PathBuf>,
}

impl Bundle {
    pub(crate) fn run(self) -> Result<()> {
        let Self { input, output } = self;
        let output = match output {
            Some(output) => output,
            None => default_output_path(&input)?,
        };
        let bundled = cargo_cp::bundle(&input)?;
        if let Some(parent) = output.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("failed to create `{}`", parent.display()))?;
        }
        fs::write(&output, bundled)
            .with_context(|| format!("failed to write `{}`", output.display()))?;
        eprintln!("bundled {} -> {}", input.display(), output.display());
        Ok(())
    }
}

fn default_output_path(input: &Path) -> Result<PathBuf> {
    let filename = input
        .file_name()
        .with_context(|| format!("input `{}` has no file name", input.display()))?;
    let metadata = MetadataCommand::new()
        .current_dir(input.parent().unwrap_or_else(|| Path::new(".")))
        .no_deps()
        .exec()
        .context("failed to run `cargo metadata`; is the solution inside a Cargo workspace?")?;
    Ok(metadata
        .workspace_root
        .join("crates/bundled/src/bin")
        .into_std_path_buf()
        .join(filename))
}

#[cfg(test)]
mod tests {
    use super::default_output_path;
    use std::path::PathBuf;

    #[test]
    fn places_output_in_the_bundled_crate() {
        let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let input = manifest_dir.join("../solutions/src/chicken_jockey.rs");
        let bundled_dir = manifest_dir
            .join("../bundled/src/bin")
            .canonicalize()
            .unwrap();

        assert_eq!(
            default_output_path(&input).unwrap(),
            bundled_dir.join("chicken_jockey.rs")
        );
    }
}
