//! Node.js bindings for `wickra-strategy-ci` via napi-rs.
//!
//! A `Session` wraps the core runner; `command` takes a request JSON and returns
//! the response JSON, so Node drives the exact same byte-identical surface as
//! every other binding.

use napi_derive::napi;

/// A stateless strategy-test session. Tests and data are passed with each command.
#[napi]
pub struct Session(strategy_ci_core::Session);

#[napi]
impl Session {
    #[napi(constructor)]
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Session(strategy_ci_core::Session::new())
    }

    /// Run a command envelope (`{"cmd":"...", ...}`) and return the response JSON.
    #[napi]
    #[allow(clippy::needless_pass_by_value)]
    pub fn command(&mut self, cmd_json: String) -> napi::Result<String> {
        self.0
            .command_json(&cmd_json)
            .map_err(|e| napi::Error::from_reason(e.to_string()))
    }

    /// The crate version.
    #[napi]
    pub fn version(&self) -> &'static str {
        strategy_ci_core::VERSION
    }
}
