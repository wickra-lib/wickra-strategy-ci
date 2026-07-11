# The Wickra Strategy-CI GitHub Action

Run your trading-strategy tests in CI the way you run unit tests: golden-pin a
strategy's backtest report, catch regressions on every push, and property/fuzz
test it against perturbed data. A failing test fails the workflow.

## Quick start

Add a workflow to your strategy repository:

```yaml
name: strategy-ci
on: [push, pull_request]

permissions:
  contents: read

jobs:
  strategies:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: wickra-lib/wickra-strategy-ci@v1
        with:
          tests: tests          # a StrategyTest file or a directory of them
          data: data            # a directory of <SYMBOL>.csv OHLCV files
```

The action installs the released CLI (no Rust toolchain needed in your repo),
runs every `StrategyTest` under `tests`, and exits non-zero — failing the job —
if any golden diff, property check, or fuzz run fails.

## Inputs

| Input | Default | Description |
|-------|---------|-------------|
| `tests` | `tests` | Path to a `StrategyTest` JSON file or a directory searched recursively. |
| `data` | `data` | Path to the OHLCV data directory (`<SYMBOL>.csv`). |
| `version` | `latest` | The CLI version/tag to install (e.g. `v0.1.0`), or `latest`. |
| `format` | `text` | Output format: `text` or `json`. |
| `fail-fast` | `false` | Stop after the first failing test. |

## Outputs

| Output | Description |
|--------|-------------|
| `result` | The JSON `SuiteResult` (populated when `format: json`). |

Consume it in a later step:

```yaml
      - id: sci
        uses: wickra-lib/wickra-strategy-ci@v1
        with: { tests: tests, data: data, format: json }
      - run: echo '${{ steps.sci.outputs.result }}' | jq '.failed'
```

## Versioning

Pin to the moving major tag `@v1` to get patch and minor updates automatically,
or pin an exact release (`@v0.1.0`) for full reproducibility. The `v1` tag is
advanced to each `v1.x.y` release.

## Blessing goldens

Goldens are produced with the CLI locally and committed, never edited by hand:

```bash
wickra-strategy-ci bless tests/ --data data/   # writes each test's expected report
git add tests/ && git commit -m "bless strategy goldens"
```

A change in a golden diff on CI means the strategy's behaviour changed — re-bless
deliberately, or fix the regression.

## Notes

- Until the first published release, the action builds the CLI from source as a
  fallback (slower); once releases carry per-OS CLI binaries it downloads them.
- The action is a composite action; all internal steps run under `bash`.
