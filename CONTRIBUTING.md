# Contributing to wickra-strategy-ci

Thanks for your interest. Issues, bug reports, ideas and pull requests are all
welcome at <https://github.com/wickra-lib/wickra-strategy-ci>. For larger changes,
open an issue first so we can agree on the approach.

## Orientation

- The core — the `StrategyTest` model, tolerances, property definitions, the
  fuzz-data generator and the test runner — lives in `crates/strategy-ci-core`.
  A test is **data, not code**: a serde tree carrying an opaque `StrategySpec`
  sub-JSON that is passed verbatim to the `wickra-backtest` engine, so the same
  test crosses the C ABI and WASM unchanged.
- The reference consumer is `crates/strategy-ci-cli` (the `wickra-strategy-ci`
  binary) plus the composite GitHub Action in `action.yml`.
- Every language binding lives under `bindings/<lang>/` and exposes the same
  data-driven surface: a `Session` handle plus `command(json) -> json` and
  `version`. Bindings must preserve the **golden-parity invariant**: given the
  tests + data in `golden/{specs,data}/`, the same command produces the
  byte-identical result in `golden/expected/`.

## The dev loop

Every change runs green locally before a commit:

```bash
cargo fmt --all
cargo test --workspace --all-features
cargo test --workspace --no-default-features   # sequential path == parallel path
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo deny check
```

`cargo fmt --all` and the `clippy -D warnings` gate are enforced in CI on three
operating systems, across both the default (rayon `parallel`) and
`--no-default-features` (sequential / WASM) feature sets — a test suite must
produce a byte-identical result either way.

## Conventions

- **Commits are signed** and follow Conventional Commits (`feat:`, `fix:`,
  `chore:`, `docs:`…). One logical change per commit. Open a PR against `main`;
  do not push to `main` directly.
- **All public artifacts are in English** — code, comments, commit messages, PR
  titles and bodies, issues and docs.
- **No secrets, ever** — not in code, tests, fixtures, logs, issues or PRs. Any
  live-universe path is opt-in behind the `live` feature and never uses real
  keys in tests.
- **Production code only** — no mocks outside `#[cfg(test)]`, no TODO stubs, and
  no defensive branches that can never run (they fail coverage).

## Adding an assertion or a property

Assertions and properties are serde enums, so extending the runner means adding a
variant, not a closure. A new report-field assertion, tolerance mode or invariant
property is added to `crates/strategy-ci-core/src/model.rs` (or `property.rs`) and
handled in `src/runner.rs`, with a serde round-trip test and a golden fixture. The
backtest itself comes from the
[wickra-backtest](https://github.com/wickra-lib/wickra-backtest) engine — the
`StrategySpec` is passed through verbatim, and no backtest logic lives here. See
`docs/TESTS.md`, `docs/PROPERTIES.md` and `docs/TOLERANCES.md`.

## Developer Certificate of Origin

Contributions are accepted under the [DCO](DCO); sign off your commits with
`git commit -s`. By contributing you agree your work is dual-licensed under
`MIT OR Apache-2.0`.
