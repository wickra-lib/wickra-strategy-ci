# Threat model

Wickra Strategy-CI is a library, a CLI and a GitHub Action that run trading
strategy tests. It holds no secrets and makes no outbound network calls in its
default configuration.

## Assets

- The integrity of test results — a passing suite must reflect a real match
  between a strategy's backtest report and its pinned expectation.
- The determinism guarantee — identical inputs must yield byte-identical results
  across languages and execution paths.

## Actors

- **Contributors** — trusted; changes land via signed commits and PR review.
- **Downstream users** — run the CLI or Action in their own repositories over
  their own test and data files.
- **Untrusted input** — the test JSON and dataset files the runner consumes,
  including the opaque `StrategySpec` sub-JSON.

## Threats and mitigations

- **Malicious test input.** The Action runs in third-party repositories over
  input the maintainer does not control. Test JSON is **data, not code**: it is
  deserialized into a serde model, and the `StrategySpec` is forwarded verbatim to
  `wickra-backtest::run`. Nothing in a test is executed as code, no shell is
  spawned, and no path outside the provided test/data directories is read. A
  malformed spec surfaces as a `wickra-backtest` error, captured in the result.
- **Panic across the FFI boundary.** The release profile sets `panic = "abort"`
  so a panic can never unwind across the C ABI. Bindings never `unwrap`/`expect`
  outside tests.
- **Supply chain.** Dependencies are audited with `cargo deny` and `osv-scanner`;
  GitHub Actions are SHA-pinned; releases carry build provenance attestations.
- **Non-determinism.** A hidden `HashMap` iteration order or an unseeded PRNG
  would break the cross-language golden invariant; the golden tests and the
  sequential-vs-parallel equivalence test guard against it.

## Out of scope

No live market data or exchange keys are used by default; any such path would be
opt-in behind a feature flag and would never place real credentials in tests or
fixtures.
