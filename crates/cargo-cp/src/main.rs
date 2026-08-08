mod bundle_command;
mod fuzz;

use anyhow::Result;
use argh::{EarlyExit, FromArgs};
use bundle_command::Bundle;
use fuzz::Fuzz;
use std::env;
use std::process;

/// Competitive programming helpers.
#[derive(Debug, FromArgs)]
struct Cli {
    #[argh(subcommand)]
    command: Command,
}

#[derive(Debug, FromArgs)]
#[argh(subcommand)]
enum Command {
    Bundle(Bundle),
    Fuzz(Fuzz),
}

fn main() -> Result<()> {
    let cli = parse_env_args();

    match cli.command {
        Command::Bundle(command) => command.run(),
        Command::Fuzz(command) => command.run(),
    }
}

fn parse_env_args() -> Cli {
    let args: Vec<_> = env::args().skip(1).collect();
    match parse_args(&args) {
        Ok(cli) => cli,
        Err(early_exit) => exit_after_parse_error(early_exit),
    }
}

fn parse_args(args: &[String]) -> Result<Cli, EarlyExit> {
    // Cargo invokes installed external subcommands as `cargo-cp cp ...`, while
    // the workspace alias and direct binary invocation omit the `cp` prefix.
    let args = if args.first().map(String::as_str) == Some("cp") {
        &args[1..]
    } else {
        args
    };

    // Keep the previous no-argument and `help` behavior while letting argh
    // generate and format the actual help text.
    let help = [String::from("--help")];
    let args = if args.is_empty() || matches!(args, [argument] if argument == "help") {
        &help[..]
    } else {
        args
    };
    let args: Vec<_> = args.iter().map(String::as_str).collect();

    Cli::from_args(&["cargo", "cp"], &args)
}

fn exit_after_parse_error(early_exit: EarlyExit) -> ! {
    if early_exit.status.is_ok() {
        print!("{}", early_exit.output);
        process::exit(0);
    }

    eprint!("{}", early_exit.output);
    process::exit(1);
}

#[cfg(test)]
mod tests {
    use super::{Command, parse_args};
    use std::path::PathBuf;

    #[test]
    fn parses_bundle_arguments() {
        let args = ["bundle", "solution.rs", "-o", "submission.rs"].map(String::from);
        let cli = parse_args(&args).unwrap();
        let Command::Bundle(command) = cli.command else {
            panic!("expected bundle command");
        };

        assert_eq!(command.input, PathBuf::from("solution.rs"));
        assert_eq!(command.output, Some(PathBuf::from("submission.rs")));
    }

    #[test]
    fn accepts_cargo_external_subcommand_prefix() {
        let args = ["cp", "bundle", "solution.rs"].map(String::from);
        let cli = parse_args(&args).unwrap();
        let Command::Bundle(command) = cli.command else {
            panic!("expected bundle command");
        };

        assert_eq!(command.input, PathBuf::from("solution.rs"));
        assert_eq!(command.output, None);
    }

    #[test]
    fn parses_fuzz_target() {
        let args = ["fuzz", "fuzz_target_1"].map(String::from);
        let cli = parse_args(&args).unwrap();
        let Command::Fuzz(command) = cli.command else {
            panic!("expected fuzz command");
        };

        assert_eq!(command.target, "fuzz_target_1");
    }
}
