# Tolerances and the golden diff

The golden diff compares a freshly-recomputed `BacktestReport` against the test's
pinned `expected` report, field by field, under per-field **tolerances**. It
answers "did the numbers change beyond what I allow?".

## Flattening

Both reports are flattened into a sorted map of numeric leaves before comparison:

- a scalar `x` maps to `"x"`;
- a nested object `a.b` maps to `"a.b"`;
- an array element maps to `"equity_curve[3]"`;
- non-numeric leaves (strings, bools, null) are ignored.

Numeric leaves are rounded to eight decimals (`round8`) before diffing and before
being written into a blessed golden, so the Rust CLI and every binding agree on
the last bit.

## Tolerance encoding

`tolerances` is a map from a field key to a tolerance:

```jsonc
"tolerances": {
  "*": { "kind": "rel", "value": 0.0001 },          // default for every field
  "metrics.sharpe": { "kind": "abs", "value": 0.01 } // override for one field
}
```

| `kind` | Passes when |
|--------|-------------|
| `abs` | `\|actual - expected\| <= value` |
| `rel` | `\|actual - expected\| <= value * max(\|expected\|, \|actual\|, 1.0)` |

## Key resolution

For a field `metrics.pnl`, the diff looks up its tolerance by:

1. the exact key `metrics.pnl`;
2. otherwise the wildcard `*`.

So `"*"` sets a blanket tolerance and named keys override it. A field with no
matching tolerance must match exactly (after `round8`).

## Diff outcome

Every field that differs beyond tolerance — or is present in one report but not
the other — becomes a `FieldDiff { field, expected, actual, within_tolerance,
kind }`, where `kind` is `mismatch`, `missing` (only in the golden), or `extra`
(only in the actual). A test passes its golden axis iff the diff is empty. Fields
that differ but stay within tolerance are **not** listed.

## Choosing tolerances

- Use a small **relative** wildcard (`rel`, `1e-4`) as the default — it scales
  with the magnitude of each field and is robust to trivial float drift.
- Use **absolute** overrides for fields near zero (a ratio, a count) where a
  relative bound is meaningless.
- Tighten toward zero once a strategy is stable; a wide tolerance hides real
  regressions.
