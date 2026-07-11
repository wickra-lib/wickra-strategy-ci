# Wickra Strategy-CI — WASM

WebAssembly bindings for the Wickra Strategy-CI test runner, compiled from Rust
with [wasm-bindgen](https://wasm-bindgen.github.io/wasm-bindgen/). A `Session` drives
the deterministic core over a JSON boundary, so a browser front-end runs against
the exact same core as every other Wickra Strategy-CI binding.

## Build

```bash
wasm-pack build --target web      # for a browser bundler
wasm-pack build --target nodejs   # for node:test / Node.js
```

The output lands in `pkg/`.

## Usage

```js
import init, { Session } from "./pkg/wickra_strategy_ci_wasm.js";

await init();
const session = new Session();
const response = session.command(JSON.stringify({
  cmd: "run_test",
  test: {
    id: "momentum",
    strategy: { /* an opaque wickra-backtest StrategySpec */ },
    dataset_ref: "sym-01",
    property_checks: [{ kind: "no_nan" }],
  },
  data: { "sym-01": [ /* candles */ ] },
}));
const result = JSON.parse(response);
console.assert(result.passed);
```

## Surface

- **`new Session()`** — a stateless test session; tests and data are passed with
  each command.
- **`session.command(cmdJson) -> string`** — run a command envelope
  (`{"cmd":"...", ...}`) and return the response JSON. Commands: `run_test`,
  `bless`, `run_suite`, `list`, `version`.
- **`session.version() -> string`** and the module-level **`version()`** — the
  crate version.

Internal errors come back as an `{"ok": false, "error": ...}` response, not as a
thrown exception.

## Determinism

The backtest engine runs sequentially in the browser sandbox (no rayon thread
pool), which is byte-identical to the native run — the exact cross-language
golden invariant. The response bytes match every other binding.

## See also

- The main project: <https://github.com/wickra-lib/wickra-strategy-ci>
- Documentation: <https://wickra.org>

## License

Dual-licensed under either [MIT](../../LICENSE-MIT) or
[Apache-2.0](../../LICENSE-APACHE), at your option.
