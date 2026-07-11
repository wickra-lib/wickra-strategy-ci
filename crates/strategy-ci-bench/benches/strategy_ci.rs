//! Criterion benchmarks for the Strategy-CI core.
//!
//! `run_suite` is measured across the cross-product of test counts {10, 100,
//! 1000} and dataset sizes {small = 200 bars, large = 2000 bars}, so the report
//! captures how running a golden-diff + property suite scales with both the
//! number of tests and the length of the data each test backtests over. The
//! parallel and sequential paths are selected at compile time by the `parallel`
//! feature; run the bench with and without `--no-default-features` to compare.

use std::collections::BTreeMap;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use serde_json::json;
use strategy_ci_core::{run_suite, Candle, StrategyTest};

/// A deterministic, non-degenerate `bars`-long candle universe for one symbol.
fn universe(bars: usize) -> Vec<Candle> {
    let closes: Vec<f64> = (0..bars)
        .map(|i| 100.0 + 10.0 * (i as f64 * 0.1).sin())
        .collect();
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

/// `count` SMA-crossover tests, all sharing one dataset symbol.
fn suite(count: usize) -> Vec<StrategyTest> {
    (0..count)
        .map(|i| {
            serde_json::from_value(json!({
                "id": format!("t{i:04}"),
                "strategy": {
                    "symbol": "BENCH",
                    "timeframe": "1h",
                    "indicators": {
                        "fast": { "type": "Sma", "params": [3] },
                        "slow": { "type": "Sma", "params": [8] }
                    },
                    "entry": { "cross_above": ["fast", "slow"] },
                    "exit": { "cross_below": ["fast", "slow"] },
                    "sizing": { "type": "fixed_fraction", "fraction": 1.0 }
                },
                "dataset_ref": "BENCH",
                "property_checks": [{ "kind": "no_nan" }]
            }))
            .expect("valid test document")
        })
        .collect()
}

fn bench_run_suite(c: &mut Criterion) {
    let mut group = c.benchmark_group("run_suite");
    for &(label, bars) in &[("small", 200usize), ("large", 2000usize)] {
        let mut data = BTreeMap::new();
        data.insert("BENCH".to_string(), universe(bars));
        for &count in &[10usize, 100, 1000] {
            let tests = suite(count);
            group.throughput(Throughput::Elements(count as u64));
            group.bench_with_input(
                BenchmarkId::new(label, count),
                &(tests, data.clone()),
                |b, (tests, data)| b.iter(|| run_suite(tests, data).expect("run suite")),
            );
        }
    }
    group.finish();
}

criterion_group!(benches, bench_run_suite);
criterion_main!(benches);
