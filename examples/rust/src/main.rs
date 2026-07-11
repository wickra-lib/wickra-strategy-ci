//! A runnable Rust example: golden-pin a strategy's backtest report, confirm the
//! test passes on re-run, then show that a doctored golden is caught. The report
//! is recomputed from `(strategy, data)` by the engine, so a fabricated number
//! cannot slip past the diff.
//!
//! ```bash
//! cargo run -p wickra-strategy-ci-example
//! ```

use std::collections::BTreeMap;

use serde_json::json;
use strategy_ci_core::{bless, run_test, Candle, StrategyTest};

const SYMBOL: &str = "AAA";

fn test_doc() -> StrategyTest {
    serde_json::from_value(json!({
        "id": "ema_crossover",
        "strategy": {
            "symbol": SYMBOL,
            "timeframe": "1h",
            "indicators": {
                "fast": { "type": "Ema", "params": [3] },
                "slow": { "type": "Ema", "params": [8] }
            },
            "entry": { "cross_above": ["fast", "slow"] },
            "exit": { "cross_below": ["fast", "slow"] },
            "sizing": { "type": "fixed_fraction", "fraction": 0.95 }
        },
        "dataset_ref": SYMBOL,
        "tolerances": { "*": { "kind": "rel", "value": 0.0001 } },
        "property_checks": [{ "kind": "no_nan" }]
    }))
    .expect("valid test document")
}

/// A short V-shaped price path so the fast/slow EMA cross fires at least once.
fn candles() -> Vec<Candle> {
    let closes = [
        120.0, 118.0, 116.0, 114.0, 112.0, 110.0, 108.0, 112.0, 116.0, 120.0, 124.0, 128.0,
    ];
    closes
        .iter()
        .enumerate()
        .map(|(i, &c)| {
            let o = if i == 0 { c } else { closes[i - 1] };
            Candle {
                time: 1_700_000_000 + i64::try_from(i).unwrap() * 3600,
                open: o,
                high: o.max(c) + 1.0,
                low: o.min(c) - 1.0,
                close: c,
                volume: 1000.0,
            }
        })
        .collect()
}

fn main() {
    let mut data = BTreeMap::new();
    data.insert(SYMBOL.to_string(), candles());

    println!("wickra-strategy-ci {}", strategy_ci_core::VERSION);

    // Bless the golden, then re-run: a freshly-blessed test passes cleanly.
    let blessed = bless(&test_doc(), &data).expect("bless");
    let result = run_test(&blessed, &data).expect("run");
    assert!(result.passed, "a freshly-blessed test must pass");
    println!(
        "blessed test: PASS (diff empty, {} checks)",
        result.property_results.len()
    );

    // Doctor the golden report: a single wrong number is caught by the diff.
    let mut doctored = blessed;
    if let Some(expected) = doctored.expected.as_mut() {
        expected["metrics"]["pnl"] = json!(99_999.0);
    }
    let result = run_test(&doctored, &data).expect("run");
    assert!(!result.passed, "a doctored golden must be caught");
    println!(
        "doctored golden: FAIL ({} field diff on {})",
        result.diff.len(),
        result.diff.first().map_or("?", |d| d.field.as_str())
    );
}
