//! Deterministic data perturbation for the fuzz axis. Every perturbation draws
//! from a seeded PRNG (`rand_pcg`); nothing here touches the thread RNG, so a
//! `(seed, runs, perturbation)` triple is fully reproducible.

use rand::Rng;

use crate::model::Perturbation;
use wickra_backtest_core::Candle;

impl Perturbation {
    /// Produce a perturbed copy of `candles`, advancing `rng`. Timestamps are
    /// never changed; only prices and volume move.
    #[must_use]
    pub fn apply<R: Rng>(&self, candles: &[Candle], rng: &mut R) -> Vec<Candle> {
        match *self {
            Perturbation::Jitter { amount } => candles
                .iter()
                .map(|c| {
                    let mut scale = |x: f64| x * (1.0 + rng.random_range(-amount..=amount));
                    Candle {
                        time: c.time,
                        open: scale(c.open),
                        high: scale(c.high),
                        low: scale(c.low),
                        close: scale(c.close),
                        volume: scale(c.volume),
                    }
                })
                .collect(),
            Perturbation::GapShock { amount } => candles
                .iter()
                .map(|c| Candle {
                    close: c.close + rng.random_range(-amount..=amount) * c.close,
                    ..*c
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
