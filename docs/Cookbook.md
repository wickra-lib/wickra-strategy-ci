# Cookbook

Task-oriented recipes. See [TESTS.md](TESTS.md) for the full `StrategyTest`
schema, [PROPERTIES.md](PROPERTIES.md), [TOLERANCES.md](TOLERANCES.md) and
[FUZZING.md](FUZZING.md) for the three test axes, and
[GITHUB_ACTION.md](GITHUB_ACTION.md) for the composite Action.

## Run a suite from the CLI

```bash
# Run every test under tests/ against the CSVs under data/.
# Exits non-zero if any test fails — this is the CI gate.
wickra-strategy-ci run tests/ --data data/

# Machine-readable SuiteResult for further processing.
wickra-strategy-ci run tests/ --data data/ --format json

# List the test ids that would run.
wickra-strategy-ci list tests/
```

## Bless a golden

Goldens are produced by the runner, never hand-written. Re-run every test and
write its fresh `BacktestReport` back into each test's `expected`:

```bash
wickra-strategy-ci bless tests/ --data data/
git add tests/ && git commit -m "bless strategy goldens"
```

A golden diff on CI afterwards means the strategy's behaviour changed — re-bless
deliberately, or fix the regression.

## Gate strategies in CI as a GitHub Action

```yaml
# .github/workflows/strategy-ci.yml
jobs:
  strategy-tests:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: wickra-lib/wickra-strategy-ci@v1
        with:
          tests: tests/
          data: data/
```

The Action installs the released CLI and runs the suite; a failing test fails the
job. The `@v1` moving major tag tracks the latest 1.x release.

## Add a property check

Properties are invariants that must hold for **any** run — independent of a
pinned golden. Add them under a test's `property_checks`:

```jsonc
"property_checks": [
  { "kind": "no_nan" },
  { "kind": "max_drawdown_le", "value": 20.0 },
  { "kind": "min_trades_ge", "value": 5 },
  { "kind": "field_in_range", "field": "metrics.win_rate", "min": 0.0, "max": 100.0 }
]
```

A scalar named by a property resolves by bare name first, then the
`metrics.<name>` path (see [PROPERTIES.md](PROPERTIES.md)).

## Loosen or tighten a tolerance

`tolerances` maps a field key to a per-field bound; `"*"` is the blanket default
and named keys override it:

```jsonc
"tolerances": {
  "*": { "kind": "rel", "value": 0.0001 },
  "metrics.sharpe": { "kind": "abs", "value": 0.01 }
}
```

Use a relative wildcard as the default; use absolute overrides for fields near
zero (a ratio, a count). See [TOLERANCES.md](TOLERANCES.md).

## Guard against overfitting with fuzz

Perturb the input data with a seeded PRNG and re-check every property, so a
strategy that only survives one exact price path fails:

```jsonc
"fuzz": {
  "seed": 42,
  "runs": 8,
  "perturbation": { "kind": "jitter", "amount": 0.001 }
}
```

Keep `runs` modest (8–16) on hot CI paths — each run re-backtests a perturbed
dataset. See [FUZZING.md](FUZZING.md).

## Run from any language

Every binding drives the same JSON command envelope through a `Session`:

```python
from wickra_strategy_ci import Session
import json

session = Session()
result = json.loads(session.command(json.dumps({
    "cmd": "run_suite",
    "tests": tests,   # list of StrategyTest documents
    "data": data,     # {symbol: [candle, ...]}
})))
print(result["passed"], result["failed"])
```

The same envelope works from Rust, Node.js, WASM, C, C++, C#, Go, Java and R —
byte-identical results across all of them. See the per-binding READMEs under
`bindings/`.
