#![no_main]
//! Fuzz the golden-diff surface: two arbitrary report JSON values are diffed
//! under a tolerance map derived from the input. The diff must never panic, must
//! be deterministic, and a report diffed against itself with no tolerances must
//! be empty.

use std::collections::BTreeMap;

use libfuzzer_sys::fuzz_target;
use serde_json::Value;
use strategy_ci_core::{diff_reports, Tolerance};

fuzz_target!(|data: &[u8]| {
    let Ok(text) = std::str::from_utf8(data) else {
        return;
    };
    // Split the input on the first NUL into two JSON documents; parse each.
    let mut parts = text.splitn(2, '\0');
    let Some(Ok(expected)) = parts.next().map(serde_json::from_str::<Value>) else {
        return;
    };
    let actual = match parts.next().map(serde_json::from_str::<Value>) {
        Some(Ok(a)) => a,
        _ => expected.clone(),
    };

    // A wildcard tolerance keyed from the first input byte keeps the map bounded.
    let mut tolerances = BTreeMap::new();
    if let Some(&b) = data.first() {
        let value = f64::from(b) / 255.0;
        let tol = if b & 1 == 0 {
            Tolerance::Abs { value }
        } else {
            Tolerance::Rel { value }
        };
        tolerances.insert("*".to_string(), tol);
    }

    let d1 = diff_reports(&expected, &actual, &tolerances);
    let d2 = diff_reports(&expected, &actual, &tolerances);
    assert_eq!(
        serde_json::to_string(&d1).unwrap(),
        serde_json::to_string(&d2).unwrap(),
        "diff must be deterministic"
    );

    // Self-diff with no tolerances is always empty.
    assert!(diff_reports(&expected, &expected, &BTreeMap::new()).is_empty());
});
