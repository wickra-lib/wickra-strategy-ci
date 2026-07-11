//! Shared helpers for the integration tests: load the repo-root golden corpus.

use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

use strategy_ci_core::{Candle, StrategyTest};

/// The repo-root `golden/` directory, resolved from this crate's manifest dir.
#[must_use]
pub fn golden_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("golden")
}

/// Load every `golden/tests/*.json` test, sorted by path.
#[must_use]
pub fn load_tests() -> Vec<StrategyTest> {
    let dir = golden_dir().join("tests");
    let mut files: Vec<PathBuf> = fs::read_dir(&dir)
        .expect("read golden/tests")
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().and_then(|e| e.to_str()) == Some("json"))
        .collect();
    files.sort();
    files
        .iter()
        .map(|f| {
            let content = fs::read_to_string(f).expect("read test");
            serde_json::from_str(&content).expect("parse test")
        })
        .collect()
}

/// Load every `golden/data/<symbol>.csv` into a symbol-keyed candle map.
#[must_use]
pub fn load_data() -> BTreeMap<String, Vec<Candle>> {
    let dir = golden_dir().join("data");
    let mut data = BTreeMap::new();
    for entry in fs::read_dir(&dir).expect("read golden/data") {
        let path = entry.expect("dir entry").path();
        if path.extension().and_then(|e| e.to_str()) != Some("csv") {
            continue;
        }
        let symbol = path
            .file_stem()
            .and_then(|s| s.to_str())
            .expect("csv stem")
            .to_string();
        let content = fs::read_to_string(&path).expect("read csv");
        data.insert(symbol, parse_csv(&content));
    }
    data
}

fn parse_csv(content: &str) -> Vec<Candle> {
    let mut candles = Vec::new();
    for (idx, line) in content.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let cols: Vec<&str> = line.split(',').map(str::trim).collect();
        let time = match cols[0].parse::<i64>() {
            Ok(t) => t,
            Err(_) if idx == 0 => continue, // header row
            Err(e) => panic!("bad ts on line {}: {e}", idx + 1),
        };
        let f = |i: usize| cols[i].parse::<f64>().expect("numeric field");
        candles.push(Candle {
            time,
            open: f(1),
            high: f(2),
            low: f(3),
            close: f(4),
            volume: f(5),
        });
    }
    candles
}
