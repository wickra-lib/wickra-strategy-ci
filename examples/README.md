# Examples

Runnable, self-contained examples of Wickra Strategy-CI in every supported
language. Each one golden-pins a small EMA-crossover strategy with `bless`, then
re-runs it and confirms the test passes — the backtest report is recomputed by
the engine, so the golden is an exact, reproducible anchor. All examples share
the same strategy, data, and JSON command protocol.

| Language | Path | Run |
|----------|------|-----|
| Rust | `rust/` | `cargo run -p wickra-strategy-ci-example` |
| Python | `python/run.py` | `pip install wickra-strategy-ci && python examples/python/run.py` |
| Node.js | `node/run.js` | `cd examples/node && npm install && node run.js` |
| WASM | via the browser | see `bindings/wasm/README.md` |
| C / C++ | `c/` | `cmake -S examples/c -B examples/c/build && cmake --build examples/c/build && ctest --test-dir examples/c/build` |
| Go | `go/run.go` | `cd examples/go && go run .` |
| C# | `csharp/Run/` | `dotnet run --project examples/csharp/Run` |
| Java | `java/Run.java` | `javac`/`java` — see the header comment in `Run.java` |
| R | `r/run.R` | `Rscript examples/r/run.R` |

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
