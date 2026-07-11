//! Property-based invariants over the pure comparison and check functions:
//! whatever random (finite) report is fed in, nothing panics and every result is
//! deterministic. Random *strategies* need a valid engine spec, so the fuzzed
//! axis here is the report/tolerance/property surface — the part that must stay
//! byte-stable across all ten bindings.

use std::collections::BTreeMap;

use proptest::prelude::*;
use serde_json::{json, Value};
use strategy_ci_core::{check_all, diff_reports, flatten_report, Property, Tolerance};

/// A small report: a flat object of finite f64 metrics plus a short equity array.
fn report_strategy() -> impl Strategy<Value = Value> {
    let metric = ("[a-d]", -1e6f64..1e6f64);
    (
        prop::collection::vec(metric, 0..6),
        prop::collection::vec(-1e6f64..1e6f64, 0..8),
    )
        .prop_map(|(metrics, equity)| {
            let mut obj = serde_json::Map::new();
            for (k, v) in metrics {
                obj.insert(k, json!(v));
            }
            obj.insert("equity_curve".into(), json!(equity));
            Value::Object(obj)
        })
}

fn tolerances_strategy() -> impl Strategy<Value = BTreeMap<String, Tolerance>> {
    prop::collection::btree_map(
        "[a-d*]",
        prop_oneof![
            (0.0f64..10.0).prop_map(|value| Tolerance::Abs { value }),
            (0.0f64..1.0).prop_map(|value| Tolerance::Rel { value }),
        ],
        0..4,
    )
}

proptest! {
    #[test]
    fn flatten_never_panics_and_is_stable(report in report_strategy()) {
        let a = flatten_report(&report);
        let b = flatten_report(&report);
        prop_assert_eq!(a, b);
    }

    #[test]
    fn diff_is_deterministic(
        expected in report_strategy(),
        actual in report_strategy(),
        tolerances in tolerances_strategy(),
    ) {
        let d1 = diff_reports(&expected, &actual, &tolerances);
        let d2 = diff_reports(&expected, &actual, &tolerances);
        prop_assert_eq!(
            serde_json::to_string(&d1).unwrap(),
            serde_json::to_string(&d2).unwrap()
        );
    }

    #[test]
    fn identical_reports_have_no_diff(report in report_strategy()) {
        // With no tolerances, a report compared against itself is a perfect match.
        let diffs = diff_reports(&report, &report, &BTreeMap::new());
        prop_assert!(diffs.is_empty(), "self-diff must be empty: {:?}", diffs);
    }

    #[test]
    fn properties_are_deterministic(report in report_strategy()) {
        let props = vec![
            Property::NoNaN,
            Property::MonotoneEquity,
            Property::PnlGe { value: 0.0 },
            Property::FieldInRange { field: "a".into(), min: -1.0, max: 1.0 },
        ];
        let r1 = check_all(&props, &report);
        let r2 = check_all(&props, &report);
        prop_assert_eq!(
            serde_json::to_string(&r1).unwrap(),
            serde_json::to_string(&r2).unwrap()
        );
        // One result per property, in order.
        prop_assert_eq!(r1.len(), props.len());
    }
}
