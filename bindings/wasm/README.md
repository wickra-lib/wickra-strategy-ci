<p align="center">
  <a href="https://wickra.org"><img src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/wickra-banner.webp?v=514" alt="Wickra Strategy-CI — golden-pin your strategy's backtest report, catch regressions in CI, and property-test against fuzzed data, in ten languages plus a reusable GitHub Action" width="100%"></a>
</p>

[![CI](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-strategy-ci/ci.svg)](https://github.com/wickra-lib/wickra-strategy-ci/actions/workflows/ci.yml)
[![codecov](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-strategy-ci/codecov.svg)](https://codecov.io/gh/wickra-lib/wickra-strategy-ci)
[![npm](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-strategy-ci/npm.svg)](https://www.npmjs.com/package/wickra-strategy-ci-wasm)
[![License: MIT OR Apache-2.0](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-strategy-ci/license.svg)](https://github.com/wickra-lib/wickra-strategy-ci#license)

# Wickra Strategy-CI — WASM

---

**Jest for trading strategies — for WASM. `npm install wickra-strategy-ci-wasm` — pure WebAssembly, runs anywhere a modern JS engine does.**

WebAssembly bindings for the Wickra Strategy-CI test runner, compiled from Rust
with [wasm-bindgen](https://wasm-bindgen.github.io/wasm-bindgen/). A `Session` drives
the deterministic core over a JSON boundary, so a browser front-end runs against
the exact same core as every other Wickra Strategy-CI binding.

## Install

```bash
npm install wickra-strategy-ci-wasm
```

### Building from this repository (contributors)

```bash
wasm-pack build --target web      # for a browser bundler
wasm-pack build --target nodejs   # for node:test / Node.js
```

The output lands in `pkg/`.

## Quick start

```js
import init, { Session } from "wickra-strategy-ci-wasm";

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

### Surface

- **`new Session()`** — a stateless test session; tests and data are passed with
  each command.
- **`session.command(cmdJson) -> string`** — run a command envelope
  (`{"cmd":"...", ...}`) and return the response JSON. Commands: `run_test`,
  `bless`, `run_suite`, `list`, `version`.
- **`session.version() -> string`** and the module-level **`version()`** — the
  crate version.

Internal errors come back as an `{"ok": false, "error": ...}` response, not as a
thrown exception.

### Determinism

The backtest engine runs sequentially in the browser sandbox (no rayon thread
pool), which is byte-identical to the native run — the exact cross-language
golden invariant. The response bytes match every other binding.

## Benchmark

Every binding forwards to the same data-driven Rust core, so what this one adds is
the call overhead of wasm-bindgen, not a different result. The core's throughput is
measured by the repository's benchmark suite and the nightly `bench.yml` run; the
numbers, the machine and how to reproduce them are in the repository
[BENCHMARKS.md](https://github.com/wickra-lib/wickra-strategy-ci/blob/main/BENCHMARKS.md).

## Documentation

The full guide, the spec reference and the API documentation live in the main
repository and the documentation site:

- **Repository:** <https://github.com/wickra-lib/wickra-strategy-ci>
- **Docs** (guides, spec reference, cookbook): <https://strategy-ci.wickra.org>
- **Runnable example:** [`examples/wasm/`](https://github.com/wickra-lib/wickra-strategy-ci/tree/main/examples/wasm)

- The main project: <https://github.com/wickra-lib/wickra-strategy-ci>
- Documentation: <https://wickra.org>

Wickra Strategy-CI ships native bindings for Python, Node.js, WASM and Rust, plus a C ABI hub that any
C-capable language (C, C++, C#, Go, Java, R) links against — all forwarding to the
same data-driven, `unsafe`-forbidden Rust core.

## Security

Found a security issue? **Please don't open a public issue.** Report it privately
via the repository's *Security* tab (*"Report a vulnerability"*) or email
**support@wickra.org** with a subject line starting `[wickra security]`. Full
policy: <https://github.com/wickra-lib/wickra-strategy-ci/blob/main/SECURITY.md>.

## Disclaimer

`wickra-strategy-ci` is research and engineering tooling, not financial advice. A
passing test attests only that a strategy's backtest report matches its pinned
expectation under the given data — it makes no claim about the quality,
profitability or future performance of any strategy. Trading carries risk; you
are responsible for your own decisions.

## License

Licensed under either of [Apache-2.0](https://github.com/wickra-lib/wickra-strategy-ci/blob/main/LICENSE-APACHE)
or [MIT](https://github.com/wickra-lib/wickra-strategy-ci/blob/main/LICENSE-MIT) at your option.
