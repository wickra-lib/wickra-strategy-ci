//! The cross-language golden anchor: run the committed corpus and assert the
//! `SuiteResult` and each per-test `TestResult` match their pinned goldens
//! byte-for-byte. Every language binding runs this same corpus and must produce
//! the identical bytes; this test is the Rust side of that contract.

mod common;

use std::fs;

use strategy_ci_core::{run_suite, run_test, TestResult};

#[test]
fn suite_matches_expected_bytes() {
    let tests = common::load_tests();
    let data = common::load_data();
    let suite = run_suite(&tests, &data).expect("run suite");

    // Every test in the committed corpus passes.
    assert_eq!(
        suite.failed, 0,
        "golden corpus must be all-green: {suite:?}"
    );
    assert_eq!(suite.passed, tests.len());

    // Each per-test TestResult equals its pinned golden/expected/<id>.json,
    // compared as the exact serde bytes the bindings also emit. The optional
    // `proof` feature adds a `report_hash` the default (binding) build never
    // emits, so it is cleared before the byte-comparison.
    let expected_dir = common::golden_dir().join("expected");
    for result in &suite.results {
        let path = expected_dir.join(format!("{}.json", result.id));
        let want =
            fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
        let mut result = result.clone();
        result.report_hash = None;
        let got = serde_json::to_string(&result).expect("serialize result");
        assert_eq!(
            got,
            want.trim(),
            "golden mismatch for {} (re-bless if the engine changed deliberately)",
            result.id
        );
    }
}

#[test]
fn per_test_run_matches_suite() {
    let tests = common::load_tests();
    let data = common::load_data();
    let expected_dir = common::golden_dir().join("expected");

    for test in &tests {
        let mut result: TestResult = run_test(test, &data).expect("run test");
        result.report_hash = None; // feature-independent (see suite test)
        let path = expected_dir.join(format!("{}.json", test.id));
        let want = fs::read_to_string(&path).expect("read expected");
        let got = serde_json::to_string(&result).expect("serialize");
        assert_eq!(
            got,
            want.trim(),
            "single-run golden mismatch for {}",
            test.id
        );
    }
}
