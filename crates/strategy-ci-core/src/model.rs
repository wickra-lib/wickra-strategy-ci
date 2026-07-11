//! The data model: tests, tolerances, properties, fuzz specs, and results.
//!
//! Everything here is serde, and the JSON representation is the language
//! boundary — it must be byte-identical across all ten bindings, so enum tags
//! and field names are fixed. Numeric leaves are rounded to eight decimals
//! (`round8`) before diffing and serialization so the Rust CLI and the bindings
//! never disagree on the last bit.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

/// An opaque backtest strategy definition. It is **not** parsed here; it is
/// forwarded verbatim to the `wickra-backtest` engine, whose schema owns it.
pub type StrategyJson = serde_json::Value;

/// A `BacktestReport` as produced by the engine, held schema-agnostically so new
/// report fields flow through without a code change.
pub type ReportJson = serde_json::Value;

/// Round to eight decimals — the canonical numeric precision for diffs and for
/// the values stored in a blessed golden.
#[must_use]
pub fn round8(x: f64) -> f64 {
    (x * 1e8).round() / 1e8
}

/// Flatten a report into a sorted map of numeric leaves. A scalar `x` maps to its
/// key; `equity_curve[i]` maps to `"equity_curve[i]"`; a nested object `a.b` maps
/// to `"a.b"`. Non-numeric leaves (strings, bools, null) are ignored — only f64
/// values are compared. The `BTreeMap` makes iteration order deterministic.
#[must_use]
pub fn flatten_report(report: &ReportJson) -> BTreeMap<String, f64> {
    let mut out = BTreeMap::new();
    flatten_into(report, String::new(), &mut out);
    out
}

fn flatten_into(value: &serde_json::Value, prefix: String, out: &mut BTreeMap<String, f64>) {
    match value {
        serde_json::Value::Number(n) => {
            if let Some(f) = n.as_f64() {
                out.insert(prefix, f);
            }
        }
        serde_json::Value::Array(items) => {
            for (i, item) in items.iter().enumerate() {
                flatten_into(item, format!("{prefix}[{i}]"), out);
            }
        }
        serde_json::Value::Object(map) => {
            for (key, item) in map {
                let next = if prefix.is_empty() {
                    key.clone()
                } else {
                    format!("{prefix}.{key}")
                };
                flatten_into(item, next, out);
            }
        }
        // Strings, bools and null carry no numeric value to compare.
        _ => {}
    }
}

/// A per-field tolerance for the golden diff.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Tolerance {
    /// Within `value` of the expected, in absolute terms.
    Abs { value: f64 },
    /// Within `value` relative to `max(|expected|, |actual|, 1.0)`.
    Rel { value: f64 },
}

/// An invariant that must hold for a run, independent of any golden value.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Property {
    /// No numeric report field is NaN or infinite.
    #[serde(rename = "no_nan")]
    NoNaN,
    /// `equity_curve` never falls (within a tiny f64-noise tolerance).
    MonotoneEquity,
    /// `|max_drawdown| <= value`.
    MaxDrawdownLe { value: f64 },
    /// `num_trades >= value`.
    MinTradesGe { value: f64 },
    /// `sharpe >= value`.
    SharpeGe { value: f64 },
    /// `pnl >= value`.
    PnlGe { value: f64 },
    /// `min <= report[field] <= max`.
    FieldInRange { field: String, min: f64, max: f64 },
}

/// How the input data is perturbed for a fuzz run.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Perturbation {
    /// Each price/volume field is scaled by `1 + Uniform(-amount, +amount)`.
    Jitter { amount: f64 },
    /// Each candle is dropped with probability `p` (at least two are kept).
    Dropout { p: f64 },
    /// `Uniform(-amount, +amount) * close` is added to `close` only (gaps).
    GapShock { amount: f64 },
}

/// A fuzz axis: perturb the data `runs` times from `seed` and re-check properties.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub struct FuzzSpec {
    pub seed: u64,
    pub runs: u32,
    pub perturbation: Perturbation,
}

/// A single strategy test: a strategy, the dataset it runs on, an optional golden
/// report with tolerances, invariant properties, and an optional fuzz axis.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct StrategyTest {
    pub id: String,
    /// Opaque `wickra-backtest` StrategySpec, forwarded verbatim.
    pub strategy: StrategyJson,
    /// Dataset basename (without `.csv`), resolved under the data directory.
    pub dataset_ref: String,
    /// The pinned golden report; `None` until blessed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected: Option<ReportJson>,
    #[serde(default)]
    pub tolerances: BTreeMap<String, Tolerance>,
    #[serde(default)]
    pub property_checks: Vec<Property>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fuzz: Option<FuzzSpec>,
}

/// The nature of a field-level difference between two reports.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DiffKind {
    /// Both reports have the field but the values differ beyond tolerance.
    Mismatch,
    /// The field is in the expected report but not the actual.
    Missing,
    /// The field is in the actual report but not the expected.
    Extra,
}

/// One field that differs between the golden and the actual report.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct FieldDiff {
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actual: Option<f64>,
    pub within_tolerance: bool,
    pub kind: DiffKind,
}

/// The outcome of one property check.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PropertyResult {
    pub property: Property,
    pub passed: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
}

/// A property that failed on a specific fuzz run.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct FuzzFailure {
    pub run: u32,
    pub property: Property,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
}

/// The full result of running one test.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TestResult {
    pub id: String,
    pub passed: bool,
    /// Empty when there is no golden or the match is perfect.
    pub diff: Vec<FieldDiff>,
    pub property_results: Vec<PropertyResult>,
    /// Empty when there is no fuzz axis or every run stayed clean.
    pub fuzz_failures: Vec<FuzzFailure>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_hash: Option<String>,
}

/// The result of running a whole suite, sorted by test id.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SuiteResult {
    pub results: Vec<TestResult>,
    pub passed: usize,
    pub failed: usize,
}

impl TestResult {
    /// A test passes iff its golden diff is empty, every property passed, and no
    /// fuzz run failed. A test with no golden simply skips the diff axis.
    #[must_use]
    pub fn is_pass(
        diff: &[FieldDiff],
        properties: &[PropertyResult],
        fuzz: &[FuzzFailure],
    ) -> bool {
        diff.is_empty() && properties.iter().all(|p| p.passed) && fuzz.is_empty()
    }
}
