# Fuzzing Wickra Strategy-CI

[`cargo-fuzz`](https://rust-fuzz.github.io/book/cargo-fuzz.html) harnesses for the parsing and stateful entry points of Wickra Strategy-CI. Fuzzing requires a nightly Rust toolchain; CI runs every target for 30 seconds on the family's pinned `nightly-2026-07-01`.

## Setup

```bash
cargo install cargo-fuzz
rustup toolchain install nightly-2026-07-01
```

The date is the family's fuzz nightly, pinned in `ci.yml`: a rolling `nightly`
regressed with a codegen ICE unrelated to this code, so every repository moves
the date together, on purpose.

## Targets

| Target | What it exercises |
| --- | --- |
| `test_parse` | The parsing surface: arbitrary bytes are parsed as a `StrategyTest` (JSON). |
| `property_eval` | The property/flatten surface: arbitrary bytes are parsed as a report JSON value, flattened, and checked against the full property set. |
| `diff_reports` | The golden-diff surface: two arbitrary report JSON values are diffed under a tolerance map derived from the input. |
| `run_test` | The end-to-end runner: a fixed, valid SMA-crossover strategy is run over a candle series synthesized from the fuzz input. |

## Run

```bash
# From the repository root:
cargo +nightly-2026-07-01 fuzz run --target x86_64-unknown-linux-gnu test_parse
cargo +nightly-2026-07-01 fuzz run --target x86_64-unknown-linux-gnu property_eval
cargo +nightly-2026-07-01 fuzz run --target x86_64-unknown-linux-gnu diff_reports
cargo +nightly-2026-07-01 fuzz run --target x86_64-unknown-linux-gnu run_test
```

Each run continues until a crash is found or it is interrupted. A short
time-boxed smoke run is what CI does:

```bash
cargo +nightly-2026-07-01 fuzz run --target x86_64-unknown-linux-gnu test_parse -- -max_total_time=30
```

The expectation for every target is that it never panics: malformed or
adversarial input must surface as an `Err` or an in-band error, never a crash.
