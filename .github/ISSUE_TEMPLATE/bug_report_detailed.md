---
name: Bug report (detailed)
about: A thorough report for a reproducible defect — cross-language disagreement, a wrong diff, a non-deterministic run.
title: "[bug] <short description>"
labels: ["bug"]
assignees: []
---

## Summary

<!-- One or two sentences. What is wrong? -->

## Severity

- [ ] Wrong result (a report passes that should fail, or fails that should pass)
- [ ] Bindings disagree (the same test JSON gives different results per language)
- [ ] Non-deterministic (the same input gives different results across runs)
- [ ] Crash / panic / memory error
- [ ] Usability or error-message problem

## Minimal reproduction

<!--
The smallest StrategyTest and dataset that reproduces it. Trim the strategy and
the candle series until removing anything else makes the problem disappear.
-->

```jsonc
// the test
```

```csv
# the candles
time,open,high,low,close,volume
```

```bash
# the exact command
wickra-strategy-ci run ... --data ...
```

## Expected vs actual

- Expected:
- Actual:

<!-- Paste the FieldDiff / property result / error verbatim, not a summary. -->

## Does it reproduce elsewhere?

<!--
Cross-language bugs are the highest-value reports. If you can, run the same test
through a second binding and say whether it agrees.
-->

| Binding | Result |
|---------|--------|
| Rust CLI | |
| other | |

- [ ] Reproduces with `--no-default-features` (sequential path) as well as the
      default parallel path

## Environment

- `wickra-strategy-ci` version / commit:
- Binding and its version:
- OS and architecture:
- Rust version (if building from source):

## Anything else

<!-- When it started, a suspected commit, a workaround you found. -->
