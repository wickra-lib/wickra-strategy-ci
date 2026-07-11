#![no_main]
//! Fuzz the end-to-end runner: a fixed, valid SMA-crossover strategy is run over
//! a candle series synthesized from the fuzz input. The input only shapes the
//! (bounded, finite) prices, so the engine always receives a well-formed request
//! and `run_test` must never panic — domain errors surface as `Err`, not crashes.

use std::collections::BTreeMap;

use libfuzzer_sys::fuzz_target;
use serde_json::json;
use strategy_ci_core::{run_test, Candle, StrategyTest};

fuzz_target!(|data: &[u8]| {
    // Need a few bars for the slow SMA to warm up; bound the series length.
    if data.len() < 12 {
        return;
    }
    let candles: Vec<Candle> = data
        .iter()
        .take(200)
        .enumerate()
        .map(|(i, &b)| {
            // Map each byte to a finite price in [50, 150].
            let close = 50.0 + f64::from(b) * (100.0 / 255.0);
            Candle {
                time: 1_700_000_000 + i as i64 * 3600,
                open: close,
                high: close + 1.0,
                low: close - 1.0,
                close,
                volume: 100.0,
            }
        })
        .collect();

    let test: StrategyTest = serde_json::from_value(json!({
        "id": "fuzz",
        "strategy": {
            "symbol": "F",
            "timeframe": "1h",
            "indicators": {
                "fast": { "type": "Sma", "params": [2] },
                "slow": { "type": "Sma", "params": [5] }
            },
            "entry": { "cross_above": ["fast", "slow"] },
            "exit": { "cross_below": ["fast", "slow"] },
            "sizing": { "type": "fixed_fraction", "fraction": 1.0 }
        },
        "dataset_ref": "F",
        "property_checks": [{ "kind": "no_nan" }]
    }))
    .expect("the fixed test document is valid");

    let mut dataset = BTreeMap::new();
    dataset.insert("F".to_string(), candles);

    // Must not panic; Ok or Err are both acceptable outcomes.
    let _ = run_test(&test, &dataset);
});
