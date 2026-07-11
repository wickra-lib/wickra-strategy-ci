//! Serde-boundary conformance: every enum variant round-trips through its pinned
//! JSON tag, `flatten_report` produces the documented dotted keys, and malformed
//! inputs fail definitively rather than silently.

use serde_json::json;
use strategy_ci_core::flatten_report;

/// A value round-trips iff serializing it yields the pinned JSON and parsing that
/// JSON yields an equal value.
fn roundtrip<T>(value: &T, pinned: serde_json::Value)
where
    T: serde::Serialize + serde::de::DeserializeOwned + PartialEq + std::fmt::Debug,
{
    let serialized = serde_json::to_value(value).expect("serialize");
    assert_eq!(serialized, pinned, "serialized tag drift");
    let parsed: T = serde_json::from_value(pinned).expect("parse");
    assert_eq!(&parsed, value, "round-trip inequality");
}

#[test]
fn property_tags_are_pinned() {
    use strategy_ci_core::Property;
    roundtrip(&Property::NoNaN, json!({"kind": "no_nan"}));
    roundtrip(
        &Property::MonotoneEquity,
        json!({"kind": "monotone_equity"}),
    );
    roundtrip(
        &Property::MaxDrawdownLe { value: 0.5 },
        json!({"kind": "max_drawdown_le", "value": 0.5}),
    );
    roundtrip(
        &Property::MinTradesGe { value: 3.0 },
        json!({"kind": "min_trades_ge", "value": 3.0}),
    );
    roundtrip(
        &Property::SharpeGe { value: 1.0 },
        json!({"kind": "sharpe_ge", "value": 1.0}),
    );
    roundtrip(
        &Property::PnlGe { value: 0.0 },
        json!({"kind": "pnl_ge", "value": 0.0}),
    );
    roundtrip(
        &Property::FieldInRange {
            field: "metrics.win_rate".into(),
            min: 0.0,
            max: 100.0,
        },
        json!({"kind": "field_in_range", "field": "metrics.win_rate", "min": 0.0, "max": 100.0}),
    );
}

#[test]
fn perturbation_tags_are_pinned() {
    use strategy_ci_core::Perturbation;
    roundtrip(
        &Perturbation::Jitter { amount: 0.01 },
        json!({"kind": "jitter", "amount": 0.01}),
    );
    roundtrip(
        &Perturbation::Dropout { p: 0.1 },
        json!({"kind": "dropout", "p": 0.1}),
    );
    roundtrip(
        &Perturbation::GapShock { amount: 0.05 },
        json!({"kind": "gap_shock", "amount": 0.05}),
    );
}

#[test]
fn tolerance_and_diffkind_tags_are_pinned() {
    use strategy_ci_core::{DiffKind, Tolerance};
    roundtrip(
        &Tolerance::Abs { value: 0.1 },
        json!({"kind": "abs", "value": 0.1}),
    );
    roundtrip(
        &Tolerance::Rel { value: 0.001 },
        json!({"kind": "rel", "value": 0.001}),
    );
    roundtrip(&DiffKind::Mismatch, json!("mismatch"));
    roundtrip(&DiffKind::Missing, json!("missing"));
    roundtrip(&DiffKind::Extra, json!("extra"));
}

#[test]
fn flatten_report_produces_dotted_keys() {
    let report = json!({
        "pnl": 12.5,
        "metrics": { "max_drawdown": -0.2, "num_trades": 8 },
        "equity_curve": [10.0, 11.0, 10.5],
        "note": "ignored",     // non-numeric leaf: dropped
        "ok": true,            // non-numeric leaf: dropped
    });
    let flat = flatten_report(&report);
    assert_eq!(flat.get("pnl"), Some(&12.5));
    assert_eq!(flat.get("metrics.max_drawdown"), Some(&-0.2));
    assert_eq!(flat.get("metrics.num_trades"), Some(&8.0));
    assert_eq!(flat.get("equity_curve[0]"), Some(&10.0));
    assert_eq!(flat.get("equity_curve[2]"), Some(&10.5));
    assert!(!flat.contains_key("note"));
    assert!(!flat.contains_key("ok"));
}

#[test]
fn unknown_property_kind_is_rejected() {
    use strategy_ci_core::Property;
    let err = serde_json::from_value::<Property>(json!({"kind": "teleport"}));
    assert!(err.is_err(), "an unknown property kind must not parse");
}

#[test]
fn missing_required_field_is_rejected() {
    use strategy_ci_core::Property;
    // max_drawdown_le requires `value`.
    let err = serde_json::from_value::<Property>(json!({"kind": "max_drawdown_le"}));
    assert!(err.is_err(), "a missing required field must not parse");
}
