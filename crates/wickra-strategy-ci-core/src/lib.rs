//! Data-driven core of Wickra Strategy-CI.
//!
//! A [`StrategyTest`] carries an opaque `StrategySpec` sub-JSON that is forwarded
//! verbatim to the [`wickra-backtest`](https://github.com/wickra-lib/wickra-backtest)
//! engine. The resulting `BacktestReport` is golden-pinned and asserted against
//! per-field [`Tolerance`]s, checked for invariant [`Property`]s, and fuzzed with
//! seeded data [`Perturbation`]s. The [`Session`] `command_json` boundary is the
//! single JSON entry point every language binding drives, so results are
//! byte-identical across languages and between the parallel and sequential paths.

#![cfg_attr(docsrs, feature(doc_cfg))]

mod config;
mod error;
mod fuzz;
mod model;
mod property;
mod runner;
mod session;
mod tolerance;

pub use config::{Config, VERSION};
pub use error::{Error, Result};
pub use model::{
    flatten_report, round8, DiffKind, FieldDiff, FuzzFailure, FuzzSpec, Perturbation, Property,
    PropertyResult, ReportJson, StrategyJson, StrategyTest, SuiteResult, TestResult, Tolerance,
};
pub use property::{check, check_all};
pub use runner::{bless, run_suite, run_test};
pub use session::Session;
pub use tolerance::diff_reports;

/// The OHLCV candle type, re-exported from the backtest engine so tests, data and
/// bindings all speak the exact same JSON.
pub use wickra_backtest_core::Candle;

#[cfg(test)]
mod tests;
