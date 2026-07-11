//! Command implementations: run, bless, list.

use std::fs;
use std::path::Path;

use strategy_ci_core::{bless as bless_test, run_suite, SuiteResult, TestResult};

use crate::args::Format;
use crate::load::{load_data, load_tests};

/// Run a suite and print it. Returns the number of failing tests (0 = all pass).
pub fn run(path: &Path, data_dir: &Path, format: Format, fail_fast: bool) -> Result<usize, String> {
    let tests: Vec<_> = load_tests(path)?.into_iter().map(|(_, t)| t).collect();
    let data = load_data(data_dir)?;
    let suite = run_suite(&tests, &data).map_err(|e| e.to_string())?;

    match format {
        Format::Json => println!(
            "{}",
            serde_json::to_string(&suite).map_err(|e| e.to_string())?
        ),
        Format::Text => print!("{}", render_text(&suite, fail_fast)),
    }
    Ok(suite.failed)
}

/// Re-run each test and write its blessed golden back to its source file.
pub fn bless(path: &Path, data_dir: &Path) -> Result<(), String> {
    let tests = load_tests(path)?;
    let data = load_data(data_dir)?;
    for (file, test) in tests {
        let blessed = bless_test(&test, &data).map_err(|e| e.to_string())?;
        let json = serde_json::to_string_pretty(&blessed).map_err(|e| e.to_string())?;
        fs::write(&file, format!("{json}\n"))
            .map_err(|e| format!("write {}: {e}", file.display()))?;
        println!("blessed {}", file.display());
    }
    Ok(())
}

/// Print the sorted test ids found under a path.
pub fn list(path: &Path) -> Result<(), String> {
    let mut ids: Vec<String> = load_tests(path)?.into_iter().map(|(_, t)| t.id).collect();
    ids.sort();
    for id in ids {
        println!("{id}");
    }
    Ok(())
}

fn render_text(suite: &SuiteResult, fail_fast: bool) -> String {
    use std::fmt::Write;
    let mut out = String::new();
    for result in &suite.results {
        render_one(&mut out, result);
        if fail_fast && !result.passed {
            break;
        }
    }
    let _ = write!(out, "\n{} passed, {} failed\n", suite.passed, suite.failed);
    out
}

fn render_one(out: &mut String, result: &TestResult) {
    use std::fmt::Write;
    let status = if result.passed { "PASS" } else { "FAIL" };
    let _ = writeln!(out, "{status} {}", result.id);
    if result.passed {
        return;
    }
    for diff in &result.diff {
        let _ = writeln!(
            out,
            "  diff {:?} {}: expected {:?} actual {:?}",
            diff.kind, diff.field, diff.expected, diff.actual
        );
    }
    for property in result.property_results.iter().filter(|p| !p.passed) {
        let detail = property.detail.as_deref().unwrap_or("");
        let _ = writeln!(out, "  property {:?} failed {detail}", property.property);
    }
    for failure in &result.fuzz_failures {
        let detail = failure.detail.as_deref().unwrap_or("");
        let _ = writeln!(
            out,
            "  fuzz run {} {:?} failed {detail}",
            failure.run, failure.property
        );
    }
}
