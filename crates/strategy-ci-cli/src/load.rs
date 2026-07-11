//! Loading tests and candle data from the filesystem.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use strategy_ci_core::{Candle, StrategyTest};

/// Load one test file or a directory of `*.json` tests, sorted by path so the
/// suite order is deterministic. Each entry keeps its source path so `bless` can
/// write the golden back.
pub fn load_tests(path: &Path) -> Result<Vec<(PathBuf, StrategyTest)>, String> {
    let mut files = Vec::new();
    collect_json(path, &mut files)?;
    files.sort();
    let mut tests = Vec::with_capacity(files.len());
    for file in files {
        let content =
            fs::read_to_string(&file).map_err(|e| format!("read {}: {e}", file.display()))?;
        let test: StrategyTest =
            serde_json::from_str(&content).map_err(|e| format!("parse {}: {e}", file.display()))?;
        tests.push((file, test));
    }
    if tests.is_empty() {
        return Err(format!("no tests found under {}", path.display()));
    }
    Ok(tests)
}

fn collect_json(path: &Path, out: &mut Vec<PathBuf>) -> Result<(), String> {
    if path.is_dir() {
        let mut entries: Vec<PathBuf> = fs::read_dir(path)
            .map_err(|e| format!("read dir {}: {e}", path.display()))?
            .filter_map(|e| e.ok().map(|e| e.path()))
            .collect();
        entries.sort();
        for entry in entries {
            collect_json(&entry, out)?;
        }
    } else if path.extension().and_then(|e| e.to_str()) == Some("json") {
        out.push(path.to_path_buf());
    }
    Ok(())
}

/// Load a directory of `<SYMBOL>.csv` candle files into a symbol-keyed map.
pub fn load_data(dir: &Path) -> Result<BTreeMap<String, Vec<Candle>>, String> {
    let mut data = BTreeMap::new();
    for entry in fs::read_dir(dir).map_err(|e| format!("read data dir {}: {e}", dir.display()))? {
        let path = entry.map_err(|e| e.to_string())?.path();
        if path.extension().and_then(|e| e.to_str()) != Some("csv") {
            continue;
        }
        let symbol = path
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or_else(|| format!("bad csv name: {}", path.display()))?
            .to_string();
        let content =
            fs::read_to_string(&path).map_err(|e| format!("read {}: {e}", path.display()))?;
        data.insert(symbol, parse_csv(&content)?);
    }
    Ok(data)
}

/// Parse a `ts,open,high,low,close,volume` CSV into candles. The `ts` column maps
/// to the candle's `time` field; a non-numeric first cell (the header) is skipped.
fn parse_csv(content: &str) -> Result<Vec<Candle>, String> {
    let mut candles = Vec::new();
    for (idx, line) in content.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let cols: Vec<&str> = line.split(',').map(str::trim).collect();
        if cols.len() < 6 {
            return Err(format!(
                "CSV line {}: expected 6 columns, got {}",
                idx + 1,
                cols.len()
            ));
        }
        let time = match cols[0].parse::<i64>() {
            Ok(t) => t,
            Err(_) if idx == 0 => continue, // header row
            Err(e) => return Err(format!("CSV line {}: bad ts: {e}", idx + 1)),
        };
        let field = |i: usize, name: &str| {
            cols[i]
                .parse::<f64>()
                .map_err(|e| format!("CSV line {}: {name}: {e}", idx + 1))
        };
        candles.push(Candle {
            time,
            open: field(1, "open")?,
            high: field(2, "high")?,
            low: field(3, "low")?,
            close: field(4, "close")?,
            volume: field(5, "volume")?,
        });
    }
    Ok(candles)
}
