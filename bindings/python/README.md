# Wickra Strategy-CI — Python

Python bindings for the Wickra Strategy-CI test runner, built with
[PyO3](https://pyo3.rs/) and [maturin](https://www.maturin.rs/). A `Session`
drives the deterministic core over a JSON boundary, so the result is
byte-identical to every other Wickra Strategy-CI binding.

## Install

```bash
pip install wickra-strategy-ci
```

## Usage

```python
import json
from wickra_strategy_ci import Session

session = Session()
response = session.command(json.dumps({
    "cmd": "run_test",
    "test": {
        "id": "momentum",
        "strategy": { ... },        # an opaque wickra-backtest StrategySpec
        "dataset_ref": "sym-01",
        "property_checks": [{"kind": "no_nan"}],
    },
    "data": {"sym-01": [ ... ]},    # candles per symbol
}))
result = json.loads(response)
assert result["passed"]
```

## Surface

- **`Session()`** — a stateless test session; tests and data are passed with each
  command.
- **`Session.command(cmd_json) -> str`** — run a command envelope
  (`{"cmd":"...", ...}`) and return the response JSON. Commands: `run_test`,
  `bless`, `run_suite`, `list`, `version`.
- **`Session.version() -> str`** and **`__version__`** — the crate version.

The `test`/`tests` carry an opaque `StrategySpec` sub-JSON forwarded verbatim to
the backtest engine; internal errors come back as an `{"ok": false, "error": ...}`
response, not as an exception.

## Determinism

The response bytes are identical across languages and between the parallel and
sequential execution paths, because the whole test runner lives once in the Rust
core and this binding forwards its JSON verbatim.

## Building from source

```bash
maturin develop --release
pytest -q
```

## See also

- The main project: <https://github.com/wickra-lib/wickra-strategy-ci>
- Documentation: <https://wickra.org>

## License

Dual-licensed under either [MIT](../../LICENSE-MIT) or
[Apache-2.0](../../LICENSE-APACHE), at your option.
