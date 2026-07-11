//! Suite determinism: running the same corpus twice yields byte-identical
//! `SuiteResult`s, and the whole-suite path agrees with running each test alone.
//! The parallel and sequential engines produce the same bytes because
//! `run_suite` sorts by id; CI exercises both by running this test with the
//! `parallel` feature on and off (`--no-default-features`).

mod common;

use strategy_ci_core::{run_suite, run_test};

#[test]
fn run_suite_is_deterministic() {
    let tests = common::load_tests();
    let data = common::load_data();

    let a = run_suite(&tests, &data).expect("run a");
    let b = run_suite(&tests, &data).expect("run b");

    let sa = serde_json::to_string(&a).expect("ser a");
    let sb = serde_json::to_string(&b).expect("ser b");
    assert_eq!(sa, sb, "run_suite must be byte-deterministic");
}

#[test]
fn suite_agrees_with_individual_runs() {
    let tests = common::load_tests();
    let data = common::load_data();

    let suite = run_suite(&tests, &data).expect("run suite");

    // The suite result is sorted by id; build the same-sorted individual results.
    let mut individual: Vec<_> = tests
        .iter()
        .map(|t| run_test(t, &data).expect("run test"))
        .collect();
    individual.sort_by(|x, y| x.id.cmp(&y.id));

    assert_eq!(
        serde_json::to_string(&suite.results).expect("ser suite"),
        serde_json::to_string(&individual).expect("ser individual"),
        "whole-suite path must equal the per-test path",
    );
}
