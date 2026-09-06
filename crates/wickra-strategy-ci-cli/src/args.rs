//! Command-line argument model.

use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};

/// Run golden, property and fuzz tests for trading strategies against the
/// deterministic wickra-backtest engine, and gate CI on the result.
#[derive(Parser, Debug)]
#[command(name = "wickra-strategy-ci", version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Run a test or a directory of tests; exits non-zero if any test fails.
    Run {
        /// A test JSON file, or a directory searched recursively for `*.json`.
        path: PathBuf,
        /// Directory of `<SYMBOL>.csv` candle files.
        #[arg(long)]
        data: PathBuf,
        /// Output format.
        #[arg(long, value_enum, default_value_t = Format::Text)]
        format: Format,
        /// Stop rendering after the first failing test (text output only).
        #[arg(long)]
        fail_fast: bool,
    },
    /// Re-run tests and write their fresh reports back as the `expected` golden.
    Bless {
        /// A test JSON file, or a directory searched recursively for `*.json`.
        path: PathBuf,
        /// Directory of `<SYMBOL>.csv` candle files.
        #[arg(long)]
        data: PathBuf,
    },
    /// List the test ids found under a path.
    List {
        /// A test JSON file, or a directory searched recursively for `*.json`.
        path: PathBuf,
    },
    /// Print the version.
    Version,
}

#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
pub enum Format {
    Text,
    Json,
}
