//! `wickra-strategy-ci` — the reference CLI and the CI gate.
//!
//! `run` exits non-zero when any test fails, so it drops straight into a CI
//! pipeline (or the composite GitHub Action) as a pass/fail step.

mod args;
mod load;
mod run;

use std::process::ExitCode;

use clap::Parser;

use args::{Cli, Command};

fn main() -> ExitCode {
    let cli = Cli::parse();
    let result = match cli.command {
        Command::Run {
            path,
            data,
            format,
            fail_fast,
        } => run::run(&path, &data, format, fail_fast).map(|failed| {
            // The CI gate: any failing test exits non-zero.
            if failed == 0 {
                ExitCode::SUCCESS
            } else {
                ExitCode::FAILURE
            }
        }),
        Command::Bless { path, data } => run::bless(&path, &data).map(|()| ExitCode::SUCCESS),
        Command::List { path } => run::list(&path).map(|()| ExitCode::SUCCESS),
        Command::Version => {
            println!("{}", strategy_ci_core::VERSION);
            Ok(ExitCode::SUCCESS)
        }
    };

    match result {
        Ok(code) => code,
        Err(message) => {
            eprintln!("error: {message}");
            ExitCode::FAILURE
        }
    }
}
