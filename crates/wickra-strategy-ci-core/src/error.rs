//! Error type for the strategy-ci core.

use thiserror::Error;

/// A `Result` specialized to [`Error`].
pub type Result<T> = std::result::Result<T, Error>;

/// Everything that can go wrong while running a strategy test.
///
/// Internal failures (bad spec, missing data, a backtest error) are surfaced to
/// callers as a value: the [`crate::Session`] boundary serializes them to
/// `{"ok":false,"error":"..."}` rather than crossing the FFI as an error code.
#[derive(Debug, Error)]
pub enum Error {
    /// A JSON document could not be parsed or serialized.
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),

    /// A malformed request: an unknown command, an unknown property kind, or a
    /// structurally invalid test.
    #[error("bad spec: {0}")]
    BadSpec(String),

    /// The dataset a test references was not provided in the `data` map.
    #[error("data error: {0}")]
    Data(String),

    /// The backtest engine rejected the strategy or the candles.
    #[error("backtest error: {0}")]
    Backtest(String),
}
