# Wickra Strategy-CI examples

Runnable, self-contained examples of Wickra Strategy-CI in every supported
language. Each one golden-pins a small EMA-crossover strategy with `bless`, then
re-runs it and confirms the test passes — the backtest report is recomputed by
the engine, so the golden is an exact, reproducible anchor. All examples share
the same strategy, data, and JSON command protocol.

## Rust — `examples/rust/`

| Example | What it does |
| --- | --- |
| `src/main.rs` | A runnable Rust example: golden-pin a strategy's backtest report, confirm the test passes on re-run, then show that a doctored golden is caught. |

## C / C++ — `examples/c/`

Build the library first (`cargo build -p wickra-strategy-ci-c --release`), then build and run
the examples via CMake, as the CI C ABI job does:

```bash
cmake -S examples/c -B examples/c/build
cmake --build examples/c/build --config Release
ctest --test-dir examples/c/build -C Release --output-on-failure
```

| Example | What it does |
| --- | --- |
| `run.c` | A runnable C example against the wickra-strategy-ci C ABI: run a golden test |
| `run.cpp` | A runnable C++ example against the wickra-strategy-ci C ABI: bless a test and confirm the golden is pinned. |

## C# — `examples/csharp/`

| Example | What it does |
| --- | --- |
| `Run/Program.cs` | A runnable C# example: golden-pin a strategy with `bless`, then re-run it and confirm it passes. |

## Go — `examples/go/`

| Example | What it does |
| --- | --- |
| `run.go` | A runnable Go example: golden-pin a strategy with `bless`, then re-run it and confirm it passes. |

## R — `examples/r/`

| Example | What it does |
| --- | --- |
| `run.R` | A runnable R example: golden-pin a strategy with bless, then re-run it and confirm it passes. |

## Java — `examples/java/`

| Example | What it does |
| --- | --- |
| `Run.java` | A runnable Java example: golden-pin a strategy with `bless`, then re-run it and confirm it passes. |

## Python — `examples/python/`

| Example | What it does |
| --- | --- |
| `run.py` | A runnable Python example: golden-pin a strategy with `bless`, then re-run it |

## Node.js — `examples/node/`

| Example | What it does |
| --- | --- |
| `run.js` | A runnable Node.js example: golden-pin a strategy with `bless`, then re-run it and confirm it passes. |

## WASM — `examples/wasm/`

Build the WASM package, serve the repository root, and open the page in a browser;
the module script inside it is what runs (CI parses it with `node --check`):

```bash
wasm-pack build bindings/wasm --target web
python -m http.server 8000     # then open http://localhost:8000/examples/wasm/
```

| Example | What it does |
| --- | --- |
| `run.mjs` | A runnable WASM example: golden-pin a strategy with `bless`, then re-run it and confirm it passes. |

## Example datasets

The examples are self-contained: the spec and the input are inline, so there is
no shared `data/` directory to load. The cross-language golden fixtures, which
every binding is checked against byte for byte, live in [`../golden/`](../golden).

## Native library

The C, Go, C#, Java and R examples link the C ABI library. Build it once and
stage it where each toolchain expects:

```bash
cargo build --release -p wickra-strategy-ci-c
# C:   picked up from target/release by CMake
# Go:  copy into bindings/go/lib/<goos>_<goarch>/
# C#:  copied next to the example by the .csproj
# Java: pass -Dnative.lib.dir=target/release (or target/debug)
# R:   set WKSTRATEGYCI_INC / WKSTRATEGYCI_LIB before R CMD INSTALL
```

Every example prints the same two lines: the library version and
`blessed test: PASS`.
