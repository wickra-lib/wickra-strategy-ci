#![no_main]
//! Fuzz the property/flatten surface: arbitrary bytes are parsed as a report
//! JSON value, flattened, and checked against the full property set. None of it
//! may panic, and the result must be deterministic and one-per-property.

use libfuzzer_sys::fuzz_target;
use serde_json::Value;
use strategy_ci_core::{check_all, flatten_report, Property};

fuzz_target!(|data: &[u8]| {
    let Ok(text) = std::str::from_utf8(data) else {
        return;
    };
    let Ok(report) = serde_json::from_str::<Value>(text) else {
        return;
    };

    // Flatten is total and stable.
    let flat_a = flatten_report(&report);
    let flat_b = flatten_report(&report);
    assert_eq!(flat_a, flat_b);

    let props = [
        Property::NoNaN,
        Property::MonotoneEquity,
        Property::MaxDrawdownLe { value: 1.0 },
        Property::MinTradesGe { value: 1.0 },
        Property::SharpeGe { value: 0.0 },
        Property::PnlGe { value: 0.0 },
        Property::FieldInRange {
            field: "metrics.win_rate".into(),
            min: 0.0,
            max: 100.0,
        },
    ];
    let r1 = check_all(&props, &report);
    let r2 = check_all(&props, &report);
    assert_eq!(r1.len(), props.len(), "one result per property");
    assert_eq!(
        serde_json::to_string(&r1).unwrap(),
        serde_json::to_string(&r2).unwrap(),
        "property evaluation must be deterministic"
    );
});
