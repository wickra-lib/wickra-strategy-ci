---
name: Performance regression
about: A suite got measurably slower, or memory use grew, between two versions.
title: "[perf] <short description>"
labels: ["performance"]
assignees: []
---

## What got slower

<!-- Which command, which suite shape. e.g. `run` over 500 tests x 2000 bars. -->

## Measurements

<!--
Numbers from the same machine, otherwise the comparison says nothing. Report
medians, and say how many runs each figure came from.
-->

| Version / commit | Suite | Per test | Peak RSS |
|------------------|-------|----------|----------|
| known good: | | | |
| current: | | | |

- Runs per measurement:
- Machine (CPU, cores, RAM, OS):

## Suite shape

- Number of tests:
- Bars per dataset:
- Fuzz `runs` per test (if any):
- Feature flags: `parallel` on / off, `proof` on / off

## How you measured

```bash
# the exact commands, including any warm-up
```

<!--
If you can, narrow it: does `cargo bench -p strategy-ci-bench` show it too? That
separates a runner regression from an engine regression in wickra-backtest.
-->

- [ ] `cargo bench -p strategy-ci-bench` reproduces it
- [ ] Only the CLI reproduces it (so it is loading, parsing or output, not the runner)

## Suspected cause

<!-- A dependency bump, a commit range, an engine version. Optional. -->
