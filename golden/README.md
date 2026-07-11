# Golden fixtures

The canonical, cross-language golden corpus for `wickra-strategy-ci`. Every
binding (Rust, Python, Node.js, WASM, Go, C#, Java, R) runs this same corpus and
must produce a **byte-identical** `SuiteResult`. Do **not** edit any file here by
hand — they are all machine-generated and pinned.

## Layout

- **`data/`** — the deterministic candle universe (`sym-01`…`sym-06`, 48 bars
  each). Each `<symbol>.csv` is `ts,open,high,low,close,volume`.
- **`tests/`** — the canonical `StrategyTest`s. Each carries an opaque
  `wickra-backtest` strategy, its dataset reference, per-field tolerances,
  invariant `property_checks`, an optional `fuzz` axis, and — once blessed — the
  pinned `expected` report.
- **`expected/`** — one `TestResult` JSON per test, the runner-golden that the
  cross-language conformance tests assert against.

## Data formula

Each symbol's close is a fixed, reproducible path:

```
close(i) = base + amp * sin(i / k) + drift * i          (i = 0 .. 47)
open(0)  = close(0);   open(i) = close(i-1)
high(i)  = max(open(i), close(i)) + 1
low(i)   = min(open(i), close(i)) - 1
ts(i)    = 1_700_000_000 + i * 3600
```

| symbol | base | amp | k   | drift | volume |
|--------|------|-----|-----|-------|--------|
| sym-01 | 100  | 10  | 4.0 | 0.05  | 1000   |
| sym-02 | 200  | 8   | 6.0 | 0.10  | 1500   |
| sym-03 | 50   | 5   | 3.0 | 0.20  | 800    |
| sym-04 | 150  | 15  | 5.0 | -0.05 | 1200   |
| sym-05 | 75   | 6   | 8.0 | 0.15  | 600    |
| sym-06 | 120  | 12  | 4.5 | 0.02  | 2000   |

## Regenerating (two stages, never by hand)

1. **Report golden** — writes `expected` into each `tests/*.json`:

   ```bash
   cargo run -p wickra-strategy-ci -- bless golden/tests/ --data golden/data/
   ```

2. **Runner golden** — writes each `expected/<id>.json` `TestResult`:

   ```bash
   cargo test -p strategy-ci-core golden -- --ignored --nocapture
   ```

Both outputs are byte-exact and reproducible; a diff in CI means the engine's
output changed and the goldens must be re-blessed deliberately.
