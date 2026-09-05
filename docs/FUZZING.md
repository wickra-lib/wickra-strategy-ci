# Fuzz testing

A golden pins one history; a property holds on one run. The **fuzz** axis guards
against overfitting: it perturbs the input data with a seeded PRNG, re-runs the
strategy, and re-checks every property. A strategy that only survives one exact
price path fails here.

## Encoding

A test's optional `fuzz` block:

```jsonc
"fuzz": {
  "seed": 42,                                   // deterministic PRNG seed
  "runs": 8,                                    // number of perturbed re-runs
  "perturbation": { "kind": "jitter", "amount": 0.001 }
}
```

The PRNG is a seeded `rand_pcg` generator — **never** `thread_rng` — so a fuzz
run is bit-for-bit reproducible: the same `seed` and `runs` produce the same
perturbed datasets and the same failures in every language.

## Perturbations

| `kind` | Fields | Effect |
|--------|--------|--------|
| `jitter` | `amount` | Each price/volume field is scaled by `1 + Uniform(-amount, +amount)`. |
| `dropout` | `p` | Each candle is dropped with probability `p` (at least two are kept). |
| `gap_shock` | `amount` | `Uniform(-amount, +amount) * close` is added to `close` only (gaps). |

### Candle invariants

Every candle a perturbation produces is **well-formed**: `high` is the maximum
of the four prices, `low` the minimum, and volume is never negative.

This matters because the price fields move independently. Scaling `high` and
`low` by separate draws inverts them whenever the jitter exceeds the bar's own
range — on a bar with a 0.1% range, a 5% jitter does so about 40% of the time —
and `gap_shock` moves `close` without regard to the range it has to sit inside.
The engine does not validate its input, so such a bar would be accepted in
silence, and a property failing on it would say nothing about the strategy.

The ordering is repaired rather than the draw rejected: re-drawing until a bar
came out valid would quietly bias the distribution toward wide-range bars.
The prices the PRNG produced are all kept; only which one is called `high` and
which `low` is re-assigned. Determinism is unaffected — the repair is a pure
function of the draw, and it is a no-op on any bar that was already valid.

## What it checks

For each of `runs` re-runs, the perturbation is applied to the dataset and the
test's `property_checks` are evaluated on the resulting report. Any property that
fails on any run becomes a `FuzzFailure { run, property, detail }`, and the test
fails its fuzz axis. The golden diff is **not** re-checked under fuzz — perturbed
data produces a different report by design; only the invariants must survive.

## Choosing a fuzz spec

- Start with a small `jitter` (`1e-3`) and a handful of `runs` (8–16) — enough to
  shake out knife-edge signals without dominating CI time.
- Add `dropout` to test resilience to missing bars, `gap_shock` to test gap
  handling.
- Pair fuzz with meaningful properties: `no_nan` catches numeric blow-ups,
  `min_trades_ge` catches a strategy that stops trading under perturbation, and
  `max_drawdown_le` catches one that only looks safe on the pristine history.

The dedicated cargo-fuzz targets (`fuzz/`) exercise the parser and the
diff/property engines against arbitrary bytes; the per-test `fuzz` axis exercises
the *strategy* against perturbed but well-formed data.
