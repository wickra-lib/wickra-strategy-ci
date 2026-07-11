# Property checks

Property checks are invariants that must hold for **any** run of a strategy,
independent of a pinned golden. Where a golden diff catches "the numbers
changed", a property catches "the numbers are wrong" — a negative equity, a NaN
Sharpe, too few trades. They are listed under a test's `property_checks` and
evaluated against the recomputed `BacktestReport`.

## Encoding

Each check is an internally-tagged object keyed by `kind` (snake_case):

```jsonc
"property_checks": [
  { "kind": "no_nan" },
  { "kind": "max_drawdown_le", "value": 5.0 },
  { "kind": "field_in_range", "field": "metrics.win_rate", "min": 0.0, "max": 100.0 }
]
```

## Available properties

| `kind` | Fields | Holds when |
|--------|--------|-----------|
| `no_nan` | — | No numeric report field is NaN or infinite. |
| `monotone_equity` | — | The equity curve never falls (within f64 noise). |
| `max_drawdown_le` | `value` | `max_drawdown >= -\|value\|` (drawdown no worse than the bound). |
| `min_trades_ge` | `value` | `num_trades >= value`. |
| `sharpe_ge` | `value` | `sharpe >= value`. |
| `pnl_ge` | `value` | `pnl >= value`. |
| `field_in_range` | `field, min, max` | `min <= report[field] <= max`. |

## Field resolution

A scalar named by a property (`max_drawdown`, `num_trades`, `sharpe`, `pnl`, or a
`field_in_range` name) is resolved by its **bare name first, then the
`metrics.<name>` path**. `wickra-backtest` nests its scalars under a `metrics`
object, so `max_drawdown_le` finds `metrics.max_drawdown`; a synthetic report
with a top-level `max_drawdown` also resolves. `field_in_range` can name a full
dotted path (`metrics.win_rate`) or an array index (`equity_curve[3]`) — the
report is flattened first (see [TOLERANCES.md](TOLERANCES.md) for the flatten
rules).

## Result

Each property yields a `PropertyResult { property, passed, detail }`; a failing
check carries a `detail` string (e.g. `missing: max_drawdown`). A missing field
is a **failure**, not a skip — a property that can never see its field is a bug
in the test, and surfacing it loudly is the point.
