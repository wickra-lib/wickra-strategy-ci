# Documentation

These pages are kept **beside the code** on purpose: each one describes a part of
the `StrategyTest` wire format, and that format is versioned with the crate. A
reader who checks out a tag gets the documentation that matches it.

| Page | What it covers |
|------|----------------|
| [`TESTS.md`](TESTS.md) | The `StrategyTest` model — ids, dataset refs, the opaque `StrategySpec`, and how a suite is laid out on disk. |
| [`TOLERANCES.md`](TOLERANCES.md) | The golden diff: report flattening, the `abs` / `rel` tolerance kinds, key resolution and `[*]` patterns. |
| [`PROPERTIES.md`](PROPERTIES.md) | The invariant properties a report is checked against, independent of any pinned golden. |
| [`FUZZING.md`](FUZZING.md) | The seeded data perturbations, their determinism guarantee, and the candle invariants they preserve. |
| [`GITHUB_ACTION.md`](GITHUB_ACTION.md) | Inputs, outputs and versioning of the composite action. |
| [`Cookbook.md`](Cookbook.md) | Task-shaped recipes: pin a first golden, widen a tolerance, add a property, debug a fuzz failure. |

## What is not here

The **Wickra ecosystem** documentation — the indicator reference, the quickstarts
for all ten languages, and the data layer — lives at
**[docs.wickra.org](https://docs.wickra.org)**, built from the separate
[`wickra-docs`](https://github.com/wickra-lib/wickra-docs) repository. Strategy-CI
does not restate it.

The **backtest engine** whose reports this tool pins is documented in
[`wickra-backtest`](https://github.com/wickra-lib/wickra-backtest). Strategy-CI
forwards the `StrategySpec` verbatim and never parses it, so the spec's own
schema is defined there, not here.

## Editing

Open a pull request against this repository; there is no separate site to update
for these pages. Changes that touch the wire format also need a `CHANGELOG.md`
entry.
