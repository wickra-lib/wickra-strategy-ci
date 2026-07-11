# Strategy tests

A `StrategyTest` is **data, not code** — a serde JSON document that ties a
strategy to the data it runs on and the expectations it must meet. The same test
document produces a byte-identical result in every language binding.

## Schema

```jsonc
{
  "id": "ema_crossover",          // unique test id (sorts a suite)
  "strategy": { /* ... */ },       // an opaque wickra-backtest StrategySpec
  "dataset_ref": "sym-01",         // basename of a <SYMBOL>.csv under --data
  "expected": { /* ... */ },       // the pinned golden BacktestReport (optional; written by bless)
  "tolerances": { "*": { "kind": "rel", "value": 0.0001 } },  // per-field diff bounds
  "property_checks": [ { "kind": "no_nan" } ],                 // invariants
  "fuzz": { "seed": 42, "runs": 8, "perturbation": { "kind": "jitter", "amount": 0.001 } }
}
```

- **`strategy`** is forwarded verbatim to the `wickra-backtest` engine — Strategy-CI
  never parses it, so the strategy DSL and its determinism live in one place.
- **`dataset_ref`** names a CSV under the `--data` directory (`ts,open,high,low,close,volume`).
- **`expected`** is the golden report; it is absent until you `bless` (see below).
- **`tolerances`**, **`property_checks`** and **`fuzz`** are the three test axes
  (see [TOLERANCES.md](TOLERANCES.md), [PROPERTIES.md](PROPERTIES.md),
  [FUZZING.md](FUZZING.md)).

## The three axes

Every run of a test yields a `TestResult`. A test **passes** iff:

1. **Golden diff** is empty — the recomputed `BacktestReport` matches `expected`
   within `tolerances` (a test with no `expected` skips this axis).
2. **Every property** holds on the run.
3. **No fuzz run** fails a property.

## CLI

```bash
# Run a file or a directory of tests; exits non-zero if any fails (the CI gate).
wickra-strategy-ci run tests/ --data data/

# JSON output (a SuiteResult) for machine consumption.
wickra-strategy-ci run tests/ --data data/ --format json

# Re-run every test and write its fresh report back as the expected golden.
wickra-strategy-ci bless tests/ --data data/

# List the test ids under a path.
wickra-strategy-ci list tests/
```

## The command protocol

Every binding drives the same JSON command envelope through its `Session`:

| Command | Shape | Returns |
|---------|-------|---------|
| `run_test` | `{cmd, test, data}` | a `TestResult` |
| `bless` | `{cmd, test, data}` | the blessed `StrategyTest` (with `expected`) |
| `run_suite` | `{cmd, tests, data}` | a `SuiteResult` (sorted by id) |
| `list` | `{cmd, tests}` | `{ids}` |
| `version` | `{cmd}` | `{version}` |

`data` is a symbol-keyed map of candle arrays. Internal errors come back in-band
as `{"ok": false, "error": ...}`, never as an exception.

## Blessing

Goldens are produced, never hand-written:

```bash
wickra-strategy-ci bless tests/ --data data/
git add tests/ && git commit -m "bless strategy goldens"
```

A golden diff on CI means the strategy's behaviour changed — re-bless
deliberately, or fix the regression. See [golden/README.md](../golden/README.md).
