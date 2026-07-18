//! Inline unit tests for the core.

use std::collections::BTreeMap;

use serde_json::json;

use crate::{
    bless, check_all, diff_reports, flatten_report, round8, run_suite, run_test, Candle, DiffKind,
    Perturbation, Property, Session, StrategyTest, Tolerance,
};

fn candle(time: i64, close: f64) -> Candle {
    Candle {
        time,
        open: close,
        high: close + 1.0,
        low: close - 1.0,
        close,
        volume: 100.0,
    }
}

/// A deterministic oscillating series long enough for a fast/slow SMA cross.
fn series() -> Vec<Candle> {
    (0..40)
        .map(|i| candle(i, 100.0 + 10.0 * ((i as f64) * 0.5).sin()))
        .collect()
}

fn data() -> BTreeMap<String, Vec<Candle>> {
    BTreeMap::from([("sym-01".to_string(), series())])
}

fn strategy() -> serde_json::Value {
    json!({
        "symbol": "TEST",
        "timeframe": "1h",
        "indicators": {
            "fast": { "type": "Sma", "params": [2] },
            "slow": { "type": "Sma", "params": [5] }
        },
        "entry": { "cross_above": ["fast", "slow"] },
        "exit": { "cross_below": ["fast", "slow"] },
        "sizing": { "type": "fixed_fraction", "fraction": 1.0 }
    })
}

fn base_test() -> StrategyTest {
    StrategyTest {
        id: "momentum".into(),
        strategy: strategy(),
        dataset_ref: "sym-01".into(),
        expected: None,
        tolerances: BTreeMap::new(),
        property_checks: vec![],
        fuzz: None,
    }
}

#[test]
fn flatten_walks_scalars_arrays_and_objects() {
    let report = json!({
        "pnl": 12.5,
        "equity_curve": [1.0, 2.0, 3.0],
        "nested": { "a": 4.0 },
        "label": "ignored"
    });
    let flat = flatten_report(&report);
    assert_eq!(flat.get("pnl"), Some(&12.5));
    assert_eq!(flat.get("equity_curve[1]"), Some(&2.0));
    assert_eq!(flat.get("nested.a"), Some(&4.0));
    assert!(!flat.contains_key("label"));
}

#[test]
fn round8_rounds_to_eight_decimals() {
    assert!((round8(1.123_456_789) - 1.123_456_79).abs() < 1e-12);
}

#[test]
fn diff_reports_respects_tolerances_and_kinds() {
    let expected = json!({ "pnl": 100.0, "sharpe": 1.0, "only_expected": 5.0 });
    let actual = json!({ "pnl": 100.05, "sharpe": 2.0, "only_actual": 7.0 });
    let tolerances = BTreeMap::from([
        ("pnl".to_string(), Tolerance::Abs { value: 0.1 }),
        ("sharpe".to_string(), Tolerance::Abs { value: 0.0 }),
    ]);
    let diffs = diff_reports(&expected, &actual, &tolerances);
    // pnl is within tolerance → not listed; sharpe mismatches; the two singletons.
    let fields: Vec<&str> = diffs.iter().map(|d| d.field.as_str()).collect();
    assert_eq!(fields, ["only_actual", "only_expected", "sharpe"]);
    let sharpe = diffs.iter().find(|d| d.field == "sharpe").unwrap();
    assert_eq!(sharpe.kind, DiffKind::Mismatch);
    assert_eq!(
        diffs
            .iter()
            .find(|d| d.field == "only_expected")
            .unwrap()
            .kind,
        DiffKind::Missing
    );
    assert_eq!(
        diffs
            .iter()
            .find(|d| d.field == "only_actual")
            .unwrap()
            .kind,
        DiffKind::Extra
    );
}

#[test]
fn self_diff_of_overflowing_field_is_empty() {
    // `round8` of a very large but finite value overflows to +inf, and
    // `inf - inf` is NaN — which must not read as a mismatch. A report always
    // diffs empty against itself with no tolerances, even for such fields.
    assert!(round8(1e308).is_infinite());
    let report = json!({ "huge": 1e308, "neg": -1e308, "ok": 1.5 });
    assert!(diff_reports(&report, &report, &BTreeMap::new()).is_empty());
}

#[test]
fn diff_star_prefix_tolerance_matches_indexed_keys() {
    let expected = json!({ "equity_curve": [1.0, 2.0, 3.0] });
    let actual = json!({ "equity_curve": [1.0001, 2.0001, 3.0001] });
    let tolerances = BTreeMap::from([(
        "equity_curve[*]".to_string(),
        Tolerance::Abs { value: 1e-3 },
    )]);
    assert!(diff_reports(&expected, &actual, &tolerances).is_empty());
}

#[test]
fn properties_evaluate_against_flat_report() {
    let report = json!({
        "pnl": 50.0, "sharpe": 1.5, "max_drawdown": -0.2, "num_trades": 8,
        "win_rate": 0.6, "equity_curve": [100.0, 101.0, 101.0, 102.0]
    });
    let props = vec![
        Property::NoNaN,
        Property::MonotoneEquity,
        Property::MaxDrawdownLe { value: 0.5 },
        Property::MinTradesGe { value: 5.0 },
        Property::SharpeGe { value: 1.0 },
        Property::PnlGe { value: 0.0 },
        Property::FieldInRange {
            field: "win_rate".into(),
            min: 0.0,
            max: 1.0,
        },
    ];
    let results = check_all(&props, &report);
    assert!(results.iter().all(|r| r.passed), "{results:?}");

    // A falling equity curve breaks MonotoneEquity; a missing field fails.
    let bad = json!({ "equity_curve": [100.0, 90.0] });
    let r = check_all(&[Property::MonotoneEquity], &bad);
    assert!(!r[0].passed);
    let missing = check_all(&[Property::SharpeGe { value: 1.0 }], &json!({}));
    assert!(!missing[0].passed);
}

#[test]
fn fuzz_perturbation_is_deterministic() {
    use rand::SeedableRng;
    let candles = series();
    let jitter = Perturbation::Jitter { amount: 0.01 };
    let mut a = rand_pcg::Pcg64::seed_from_u64(7);
    let mut b = rand_pcg::Pcg64::seed_from_u64(7);
    assert_eq!(
        jitter.apply(&candles, &mut a),
        jitter.apply(&candles, &mut b)
    );

    let dropout = Perturbation::Dropout { p: 0.5 };
    let mut r = rand_pcg::Pcg64::seed_from_u64(1);
    assert!(dropout.apply(&candles, &mut r).len() >= 2);
}

#[test]
fn run_test_and_bless_and_suite_end_to_end() {
    let data = data();
    let mut test = base_test();
    test.property_checks = vec![Property::NoNaN];

    // No golden yet: the diff axis is skipped, so pass depends on properties.
    let result = run_test(&test, &data).unwrap();
    assert!(result.diff.is_empty());
    assert_eq!(result.property_results.len(), 1);
    assert!(result.passed);

    // Blessing pins the report; the blessed test then matches itself exactly.
    let blessed = bless(&test, &data).unwrap();
    assert!(blessed.expected.is_some());
    let rerun = run_test(&blessed, &data).unwrap();
    assert!(rerun.diff.is_empty(), "{:?}", rerun.diff);
    assert!(rerun.passed);

    let suite = run_suite(std::slice::from_ref(&blessed), &data).unwrap();
    assert_eq!(suite.passed, 1);
    assert_eq!(suite.failed, 0);
}

#[test]
fn fuzz_axis_runs_and_reports_clean() {
    let data = data();
    let mut test = base_test();
    test.property_checks = vec![Property::NoNaN];
    test.fuzz = Some(crate::FuzzSpec {
        seed: 42,
        runs: 4,
        perturbation: Perturbation::Jitter { amount: 0.001 },
    });
    let result = run_test(&test, &data).unwrap();
    assert!(result.fuzz_failures.is_empty());
    assert!(result.passed);
}

#[test]
fn session_dispatches_commands() {
    let mut session = Session::new();
    let data = data();

    let run_cmd = json!({ "cmd": "run_test", "test": base_test(), "data": data }).to_string();
    let response = session.command_json(&run_cmd).unwrap();
    assert!(response.contains("\"id\":\"momentum\""));

    let list_cmd = json!({ "cmd": "list", "tests": [base_test()] }).to_string();
    assert_eq!(
        session.command_json(&list_cmd).unwrap(),
        json!({ "ids": ["momentum"] }).to_string()
    );

    let version = session
        .command_json(&json!({ "cmd": "version" }).to_string())
        .unwrap();
    assert!(version.contains(crate::VERSION));
}

#[test]
fn session_reports_errors_as_json_not_panics() {
    let mut session = Session::new();
    let bad = session
        .command_json(&json!({ "cmd": "nope" }).to_string())
        .unwrap();
    assert!(bad.contains("\"ok\":false"));
    let missing_data = json!({ "cmd": "run_test", "test": base_test(), "data": {} }).to_string();
    let response = session.command_json(&missing_data).unwrap();
    assert!(response.contains("missing dataset"));
}

#[test]
fn model_serde_round_trips() {
    let test = base_test();
    let json_str = serde_json::to_string(&test).unwrap();
    let back: StrategyTest = serde_json::from_str(&json_str).unwrap();
    assert_eq!(test, back);
}
