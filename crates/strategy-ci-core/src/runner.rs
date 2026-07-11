//! The test runner: forward a strategy to the backtest engine, pin and assert
//! the report, run properties, and fuzz.

use std::collections::BTreeMap;

use rand::SeedableRng;
use rand_pcg::Pcg64;
use serde_json::json;
use wickra_backtest_core::Candle;

use crate::error::{Error, Result};
use crate::model::{
    round8, FuzzFailure, ReportJson, StrategyJson, StrategyTest, SuiteResult, TestResult,
};
use crate::{property, tolerance};

/// Run the backtest for one strategy over one candle series, returning the report
/// as a schema-agnostic JSON value. The strategy is forwarded verbatim to the
/// engine's `run_json`; strategy-ci never parses it.
fn run_backtest(strategy: &StrategyJson, candles: &[Candle]) -> Result<ReportJson> {
    let request = json!({ "spec": strategy, "candles": candles });
    let report_str = wickra_backtest_core::run_json(&request.to_string())
        .map_err(|e| Error::Backtest(e.to_string()))?;
    Ok(serde_json::from_str(&report_str)?)
}

/// Deep-round every numeric leaf of a report to eight decimals — used when
/// blessing a golden so the stored value matches what the diff compares against.
fn round_report(report: &ReportJson) -> ReportJson {
    match report {
        serde_json::Value::Number(n) => n
            .as_f64()
            .and_then(|f| serde_json::Number::from_f64(round8(f)))
            .map_or_else(|| report.clone(), serde_json::Value::Number),
        serde_json::Value::Array(items) => {
            serde_json::Value::Array(items.iter().map(round_report).collect())
        }
        serde_json::Value::Object(map) => serde_json::Value::Object(
            map.iter()
                .map(|(k, v)| (k.clone(), round_report(v)))
                .collect(),
        ),
        other => other.clone(),
    }
}

#[cfg(feature = "proof")]
fn report_hash(report: &ReportJson) -> Result<String> {
    let canonical = proof_core::canonicalize(report).map_err(|e| Error::Backtest(e.to_string()))?;
    Ok(blake3::hash(canonical.as_bytes()).to_hex().to_string())
}

fn run_fuzz(
    test: &StrategyTest,
    candles: &[Candle],
    seed: u64,
    runs: u32,
) -> Result<Vec<FuzzFailure>> {
    let perturbation = test
        .fuzz
        .as_ref()
        .map(|f| f.perturbation)
        .ok_or_else(|| Error::BadSpec("fuzz axis missing".into()))?;
    let mut rng = Pcg64::seed_from_u64(seed);
    let mut failures = Vec::new();
    for run in 0..runs {
        let perturbed = perturbation.apply(candles, &mut rng);
        let report = run_backtest(&test.strategy, &perturbed)?;
        for result in property::check_all(&test.property_checks, &report) {
            if !result.passed {
                failures.push(FuzzFailure {
                    run,
                    property: result.property,
                    detail: result.detail,
                });
            }
        }
    }
    Ok(failures)
}

/// Run one strategy test to a full [`TestResult`].
pub fn run_test(test: &StrategyTest, data: &BTreeMap<String, Vec<Candle>>) -> Result<TestResult> {
    let candles = data
        .get(&test.dataset_ref)
        .ok_or_else(|| Error::Data(format!("missing dataset: {}", test.dataset_ref)))?;
    let report = run_backtest(&test.strategy, candles)?;

    let diff = match &test.expected {
        Some(expected) => tolerance::diff_reports(expected, &report, &test.tolerances),
        None => Vec::new(),
    };
    let property_results = property::check_all(&test.property_checks, &report);
    let fuzz_failures = match &test.fuzz {
        Some(fuzz) => run_fuzz(test, candles, fuzz.seed, fuzz.runs)?,
        None => Vec::new(),
    };

    #[cfg(feature = "proof")]
    let report_hash = Some(report_hash(&report)?);
    #[cfg(not(feature = "proof"))]
    let report_hash = None;

    let passed = TestResult::is_pass(&diff, &property_results, &fuzz_failures);
    Ok(TestResult {
        id: test.id.clone(),
        passed,
        diff,
        property_results,
        fuzz_failures,
        report_hash,
    })
}

/// Re-run a test and return a copy with its `expected` golden set to the freshly
/// produced (rounded) report.
pub fn bless(test: &StrategyTest, data: &BTreeMap<String, Vec<Candle>>) -> Result<StrategyTest> {
    let candles = data
        .get(&test.dataset_ref)
        .ok_or_else(|| Error::Data(format!("missing dataset: {}", test.dataset_ref)))?;
    let report = run_backtest(&test.strategy, candles)?;
    let mut blessed = test.clone();
    blessed.expected = Some(round_report(&report));
    Ok(blessed)
}

/// Run a suite of tests, sorted by id. Tests are independent, so with the
/// `parallel` feature they run concurrently; the sorted output is identical
/// either way.
pub fn run_suite(
    tests: &[StrategyTest],
    data: &BTreeMap<String, Vec<Candle>>,
) -> Result<SuiteResult> {
    #[cfg(feature = "parallel")]
    let mut results: Vec<TestResult> = {
        use rayon::prelude::*;
        tests
            .par_iter()
            .map(|test| run_test(test, data))
            .collect::<Result<Vec<_>>>()?
    };
    #[cfg(not(feature = "parallel"))]
    let mut results: Vec<TestResult> = tests
        .iter()
        .map(|test| run_test(test, data))
        .collect::<Result<Vec<_>>>()?;

    results.sort_by(|a, b| a.id.cmp(&b.id));
    let passed = results.iter().filter(|r| r.passed).count();
    let failed = results.len() - passed;
    Ok(SuiteResult {
        results,
        passed,
        failed,
    })
}
