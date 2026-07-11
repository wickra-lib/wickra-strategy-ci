//! WebAssembly bindings for `wickra-strategy-ci` (wasm-bindgen).
//!
//! Golden-pin and regression-test trading strategies, compiled to WebAssembly
//! for the browser: create a `Session`, drive it with a command JSON
//! (`run_test`, `bless`, `run_suite`, `list`, `version`) and read back the
//! response JSON. The same command protocol crosses every binding, so a browser
//! front-end runs against the exact same core as the native CLI.
//!
//! The backtest engine runs sequentially here (no rayon thread pool in a
//! browser sandbox), which is byte-identical to the native run — the exact
//! cross-language golden check.

use wasm_bindgen::prelude::*;

use strategy_ci_core::Session as CoreSession;

/// A stateless strategy-test session driven by JSON commands.
#[wasm_bindgen]
pub struct Session {
    inner: CoreSession,
}

#[wasm_bindgen]
impl Session {
    /// Create a session. Tests and data are passed with each command.
    #[wasm_bindgen(constructor)]
    #[allow(clippy::new_without_default)]
    pub fn new() -> Session {
        Self {
            inner: CoreSession::new(),
        }
    }

    /// Apply a command JSON (`{"cmd":"...", ...}`) and return the response JSON.
    pub fn command(&mut self, cmd_json: &str) -> Result<String, JsError> {
        self.inner
            .command_json(cmd_json)
            .map_err(|e| JsError::new(&e.to_string()))
    }

    /// The library version.
    #[wasm_bindgen(js_name = version)]
    pub fn instance_version(&self) -> String {
        strategy_ci_core::VERSION.to_string()
    }
}

/// The library version.
#[wasm_bindgen]
pub fn version() -> String {
    strategy_ci_core::VERSION.to_string()
}
