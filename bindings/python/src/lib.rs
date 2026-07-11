//! Python bindings for `wickra-strategy-ci` via `PyO3`.
//!
//! A [`Session`] wraps the core runner; `command` takes a request JSON and returns
//! the response JSON, so Python drives the exact same byte-identical surface as
//! every other binding.

use pyo3::prelude::*;

/// A stateless strategy-test session. Tests and data are passed with each command.
#[pyclass(unsendable)]
struct Session(strategy_ci_core::Session);

#[pymethods]
impl Session {
    #[new]
    fn new() -> Self {
        Session(strategy_ci_core::Session::new())
    }

    /// Run a command envelope (`{"cmd":"...", ...}`) and return the response JSON.
    fn command(&mut self, cmd_json: &str) -> PyResult<String> {
        self.0
            .command_json(cmd_json)
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
    }

    /// The crate version.
    #[staticmethod]
    fn version() -> &'static str {
        strategy_ci_core::VERSION
    }
}

#[pymodule]
fn _wickra_strategy_ci(m: &Bound<PyModule>) -> PyResult<()> {
    m.add_class::<Session>()?;
    m.add("__version__", strategy_ci_core::VERSION)?;
    Ok(())
}
