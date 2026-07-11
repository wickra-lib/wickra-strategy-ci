//! The `command_json` boundary: one JSON entry point every language binding
//! drives. The session is stateless with respect to tests and data (both arrive
//! in each command), so bindings stay trivial and the output is byte-identical.

use std::collections::BTreeMap;

use serde::Deserialize;
use serde_json::{json, Value};
use wickra_backtest_core::Candle;

use crate::config::{Config, VERSION};
use crate::error::{Error, Result};
use crate::model::StrategyTest;
use crate::runner;

type DataMap = BTreeMap<String, Vec<Candle>>;

#[derive(Deserialize)]
struct TestEnvelope {
    test: StrategyTest,
    #[serde(default)]
    data: DataMap,
}

#[derive(Deserialize)]
struct SuiteEnvelope {
    tests: Vec<StrategyTest>,
    #[serde(default)]
    data: DataMap,
}

#[derive(Deserialize)]
struct ListEnvelope {
    tests: Vec<StrategyTest>,
}

/// A stateless handle over the test runner. It holds only configuration; tests
/// and data are passed in with each command.
#[derive(Debug, Clone, Copy, Default)]
pub struct Session {
    #[allow(dead_code)]
    config: Config,
}

impl Session {
    /// Create a session with the default configuration.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// The crate version.
    #[must_use]
    pub fn version() -> &'static str {
        VERSION
    }

    /// Dispatch one command envelope (`{"cmd":"...", ...}`) to a JSON response.
    /// Internal failures are returned as `{"ok":false,"error":"..."}` in the
    /// response string rather than as an `Err`, so every binding parses uniformly.
    pub fn command_json(&mut self, cmd_json: &str) -> Result<String> {
        Ok(dispatch(cmd_json).unwrap_or_else(|e| error_json(&e.to_string())))
    }
}

fn error_json(message: &str) -> String {
    json!({ "ok": false, "error": message }).to_string()
}

fn dispatch(cmd_json: &str) -> Result<String> {
    let envelope: Value = serde_json::from_str(cmd_json)?;
    let cmd = envelope
        .get("cmd")
        .and_then(Value::as_str)
        .ok_or_else(|| Error::BadSpec("missing cmd".into()))?;

    match cmd {
        "run_test" => {
            let env: TestEnvelope = serde_json::from_value(envelope)?;
            let result = runner::run_test(&env.test, &env.data)?;
            Ok(serde_json::to_string(&result)?)
        }
        "bless" => {
            let env: TestEnvelope = serde_json::from_value(envelope)?;
            let blessed = runner::bless(&env.test, &env.data)?;
            Ok(serde_json::to_string(&blessed)?)
        }
        "run_suite" => {
            let env: SuiteEnvelope = serde_json::from_value(envelope)?;
            let result = runner::run_suite(&env.tests, &env.data)?;
            Ok(serde_json::to_string(&result)?)
        }
        "list" => {
            let env: ListEnvelope = serde_json::from_value(envelope)?;
            let mut ids: Vec<String> = env.tests.into_iter().map(|t| t.id).collect();
            ids.sort();
            Ok(json!({ "ids": ids }).to_string())
        }
        "version" => Ok(json!({ "version": VERSION }).to_string()),
        other => Err(Error::BadSpec(format!("unknown cmd: {other}"))),
    }
}
