<!--
Keep it short. One logical change per PR.

For a change that touches the wire format, the binding surface, or the numbers a
golden pins, use the long-form template instead by appending `?template=detailed.md`
to the compare URL. It walks through wire-format impact, binding parity and
determinism.
-->

## What

<!-- What does this change and why? -->

## Checklist

- [ ] `cargo fmt --all` and `cargo clippy --workspace --all-targets --all-features -- -D warnings` are clean
- [ ] `cargo test --workspace --all-features` and `--no-default-features` pass (parallel == sequential)
- [ ] `cargo deny check` is clean
- [ ] Tests added/updated (prefer hand-computed expectations for core changes)
- [ ] Tests stay data (a serde `StrategyTest`), never Rust closures
- [ ] Binding surface mirrored across languages; golden reports regenerated if the schema changed
- [ ] `CHANGELOG.md` updated under `[Unreleased]`
