#![no_main]
//! Fuzz the parsing surface: arbitrary bytes are parsed as a `StrategyTest`
//! (JSON). Malformed input must surface as a clean `Err`, never a panic. A
//! successfully parsed test must re-serialize and re-parse to an equal value
//! (serde round-trip stability).

use libfuzzer_sys::fuzz_target;
use strategy_ci_core::StrategyTest;

fuzz_target!(|data: &[u8]| {
    let Ok(text) = std::str::from_utf8(data) else {
        return;
    };
    let Ok(test) = serde_json::from_str::<StrategyTest>(text) else {
        return;
    };
    // A parsed test round-trips: serialize → parse → equal.
    let serialized = serde_json::to_string(&test).expect("serialize a parsed test");
    let reparsed: StrategyTest =
        serde_json::from_str(&serialized).expect("re-parse a serialized test");
    assert_eq!(reparsed, test, "StrategyTest serde round-trip is not stable");
});
