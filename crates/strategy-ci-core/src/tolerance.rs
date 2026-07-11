//! The golden diff: compare an expected report to an actual one, field by field,
//! under per-field tolerances.

use std::collections::BTreeMap;

use crate::model::{flatten_report, round8, DiffKind, FieldDiff, ReportJson, Tolerance};

/// Resolve the tolerance for a flattened key. Exact keys win; then the longest
/// `foo[*]` prefix pattern that matches; then a `"*"` default; then `Abs{0.0}`
/// (an exact match is required).
fn resolve(key: &str, tolerances: &BTreeMap<String, Tolerance>) -> Tolerance {
    if let Some(t) = tolerances.get(key) {
        return *t;
    }
    let mut best: Option<(usize, Tolerance)> = None;
    for (pat, tol) in tolerances {
        if let Some(idx) = pat.find("[*]") {
            let base = &pat[..idx];
            if key == base || key.starts_with(&format!("{base}[")) {
                let len = base.len();
                if best.is_none_or(|(blen, _)| len > blen) {
                    best = Some((len, *tol));
                }
            }
        }
    }
    if let Some((_, tol)) = best {
        return tol;
    }
    tolerances
        .get("*")
        .copied()
        .unwrap_or(Tolerance::Abs { value: 0.0 })
}

/// True iff `actual` is within `tol` of `expected`. NaN on either side is never
/// within tolerance.
fn within(expected: f64, actual: f64, tol: Tolerance) -> bool {
    if expected.is_nan() || actual.is_nan() {
        return false;
    }
    let d = (actual - expected).abs();
    match tol {
        Tolerance::Abs { value } => d <= value,
        Tolerance::Rel { value } => {
            let scale = expected.abs().max(actual.abs()).max(1.0);
            d <= value * scale
        }
    }
}

/// Diff two reports into the list of fields that differ. A perfect match yields an
/// empty vector. Values are rounded to eight decimals before comparison, and the
/// `BTreeMap` iteration order makes the output deterministic.
#[must_use]
pub fn diff_reports(
    expected: &ReportJson,
    actual: &ReportJson,
    tolerances: &BTreeMap<String, Tolerance>,
) -> Vec<FieldDiff> {
    let e = flatten_report(expected);
    let a = flatten_report(actual);

    let mut keys: Vec<&String> = e.keys().chain(a.keys()).collect();
    keys.sort_unstable();
    keys.dedup();

    let mut diffs = Vec::new();
    for key in keys {
        // Every key comes from `e` or `a` (or both); the three arms below are
        // exhaustive without a dead "neither" case.
        if let (Some(ev), Some(av)) = (e.get(key), a.get(key)) {
            let ev = round8(*ev);
            let av = round8(*av);
            if !within(ev, av, resolve(key, tolerances)) {
                diffs.push(FieldDiff {
                    field: key.clone(),
                    expected: Some(ev),
                    actual: Some(av),
                    within_tolerance: false,
                    kind: DiffKind::Mismatch,
                });
            }
        } else if let Some(ev) = e.get(key) {
            diffs.push(FieldDiff {
                field: key.clone(),
                expected: Some(round8(*ev)),
                actual: None,
                within_tolerance: false,
                kind: DiffKind::Missing,
            });
        } else if let Some(av) = a.get(key) {
            diffs.push(FieldDiff {
                field: key.clone(),
                expected: None,
                actual: Some(round8(*av)),
                within_tolerance: false,
                kind: DiffKind::Extra,
            });
        }
    }
    diffs
}
