//! Invariant property checks over a flattened report.

use std::collections::BTreeMap;

use crate::model::{Property, PropertyResult, ReportJson};

/// Check every property against `report`, returning one result each (in the
/// order the properties were given).
#[must_use]
pub fn check_all(properties: &[Property], report: &ReportJson) -> Vec<PropertyResult> {
    let flat = crate::model::flatten_report(report);
    properties.iter().map(|p| check(p, &flat)).collect()
}

/// Check a single property against a flattened report.
#[must_use]
pub fn check(property: &Property, flat: &BTreeMap<String, f64>) -> PropertyResult {
    let (passed, detail) = match property {
        Property::NoNaN => match flat.iter().find(|(_, v)| v.is_nan() || v.is_infinite()) {
            Some((k, _)) => (false, Some(k.clone())),
            None => (true, None),
        },
        Property::MonotoneEquity => monotone_equity(flat),
        Property::MaxDrawdownLe { value } => match resolve(flat, "max_drawdown") {
            Some(dd) if !dd.is_nan() => (dd >= -value.abs(), None),
            _ => (false, Some("missing: max_drawdown".into())),
        },
        Property::MinTradesGe { value } => cmp_ge(flat, "num_trades", *value),
        Property::SharpeGe { value } => cmp_ge(flat, "sharpe", *value),
        Property::PnlGe { value } => cmp_ge(flat, "pnl", *value),
        Property::FieldInRange { field, min, max } => match resolve(flat, field) {
            Some(v) if !v.is_nan() => (v >= *min && v <= *max, None),
            _ => (false, Some(format!("missing: {field}"))),
        },
    };
    PropertyResult {
        property: property.clone(),
        passed,
        detail,
    }
}

/// Resolve a metric by its bare name, falling back to the `metrics.<name>` path.
/// `wickra-backtest` reports nest their scalars under a `metrics` object, while
/// a synthetic report may put them at the top level; both resolve here.
fn resolve(flat: &BTreeMap<String, f64>, name: &str) -> Option<f64> {
    flat.get(name)
        .or_else(|| flat.get(&format!("metrics.{name}")))
        .copied()
}

fn cmp_ge(flat: &BTreeMap<String, f64>, field: &str, value: f64) -> (bool, Option<String>) {
    match resolve(flat, field) {
        Some(v) if !v.is_nan() => (v >= value, None),
        _ => (false, Some(format!("missing: {field}"))),
    }
}

/// `equity_curve` is non-falling within a tiny f64-noise tolerance. The keys are
/// `equity_curve[i]`; a `BTreeMap` sorts them lexically, not numerically, so the
/// points are re-sorted by index before the sweep.
fn monotone_equity(flat: &BTreeMap<String, f64>) -> (bool, Option<String>) {
    let mut points: Vec<(usize, f64)> = flat
        .iter()
        .filter_map(|(k, v)| {
            let idx = k.strip_prefix("equity_curve[")?.strip_suffix(']')?;
            idx.parse::<usize>().ok().map(|i| (i, *v))
        })
        .collect();
    if points.is_empty() {
        return (false, Some("no equity_curve".into()));
    }
    points.sort_unstable_by_key(|(i, _)| *i);
    for window in points.windows(2) {
        let (prev_i, prev) = window[0];
        let (cur_i, cur) = window[1];
        if cur < prev - 1e-9 {
            return (false, Some(format!("equity_curve[{cur_i}] < [{prev_i}]")));
        }
    }
    (true, None)
}
