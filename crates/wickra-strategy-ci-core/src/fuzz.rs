//! Deterministic data perturbation for the fuzz axis. Every perturbation draws
//! from a seeded PRNG (`rand_pcg`); nothing here touches the thread RNG, so a
//! `(seed, runs, perturbation)` triple is fully reproducible.

use rand::{Rng, RngExt};

use crate::model::Perturbation;
use wickra_backtest_core::Candle;

/// Restore a candle's own invariants after its fields have been moved.
///
/// A bar means `high` is the session's maximum and `low` its minimum, so the
/// four prices always satisfy `low <= min(open, close)` and
/// `high >= max(open, close)`. A perturbation that moves each field
/// independently does not preserve that: scaling a bar whose range is narrower
/// than `amount` can put the high below the low, and shifting the close alone
/// can push it outside the range the bar recorded.
///
/// The engine does not validate its input, so such a bar is accepted silently
/// and a property that fails on it says nothing about the strategy. Rather than
/// reject the draw and skew the distribution, the prices are re-ordered:
/// `high` becomes the maximum of the four and `low` the minimum. That keeps the
/// perturbation the RNG produced and yields a bar a market could print.
///
/// Volume is clamped at zero for the same reason — a negative traded quantity
/// is not a market a strategy should be judged against.
fn well_formed(candle: Candle) -> Candle {
    let high = candle
        .high
        .max(candle.low)
        .max(candle.open)
        .max(candle.close);
    let low = candle
        .low
        .min(candle.high)
        .min(candle.open)
        .min(candle.close);
    Candle {
        high,
        low,
        volume: candle.volume.max(0.0),
        ..candle
    }
}

impl Perturbation {
    /// Produce a perturbed copy of `candles`, advancing `rng`. Timestamps are
    /// never changed; only prices and volume move, and every candle produced
    /// here satisfies the OHLC ordering invariants (see [`well_formed`]).
    #[must_use]
    pub fn apply<R: Rng>(&self, candles: &[Candle], rng: &mut R) -> Vec<Candle> {
        match *self {
            Perturbation::Jitter { amount } => candles
                .iter()
                .map(|c| {
                    let mut scale = |x: f64| x * (1.0 + rng.random_range(-amount..=amount));
                    well_formed(Candle {
                        time: c.time,
                        open: scale(c.open),
                        high: scale(c.high),
                        low: scale(c.low),
                        close: scale(c.close),
                        volume: scale(c.volume),
                    })
                })
                .collect(),
            Perturbation::GapShock { amount } => candles
                .iter()
                .map(|c| {
                    well_formed(Candle {
                        close: c.close + rng.random_range(-amount..=amount) * c.close,
                        ..*c
                    })
                })
                .collect(),
            Perturbation::Dropout { p } => {
                let kept: Vec<Candle> = candles
                    .iter()
                    .copied()
                    .filter(|_| rng.random::<f64>() >= p)
                    .collect();
                // Always keep at least two candles so the engine has a stream to run.
                if kept.len() >= 2 {
                    kept
                } else {
                    candles.iter().take(2).copied().collect()
                }
            }
        }
    }
}
