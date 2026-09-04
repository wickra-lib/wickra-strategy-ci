<!--
The long-form pull-request template, for changes that touch the wire format, the
binding surface, or the numbers a golden pins.

GitHub does not offer a picker for these. To use it, append a query parameter to
the compare URL:

  ?template=detailed.md

For a small, self-contained change the default template is enough.
-->

## What

<!-- What changes, and why. Lead with the problem, not the patch. -->

## Why this shape

<!--
The alternatives you rejected and the reason. This is the part a reviewer cannot
reconstruct from the diff.
-->

## Wire-format impact

<!--
Everything here is data that every binding must agree on. Answer explicitly:
-->

- [ ] Purely additive — a new serde variant; existing tests deserialize unchanged
- [ ] Changes an existing field's meaning
- [ ] Changes the result schema

If any golden had to be re-blessed, say **which numbers moved and why they were
wrong before**. A re-blessed golden with no explanation is a silently accepted
regression.

## Binding parity

The core is reachable through one `command(json) -> json` boundary in ten
languages. A change to that boundary is not done until every binding sees it.

- [ ] Rust
- [ ] Python
- [ ] Node.js
- [ ] WASM
- [ ] C ABI (and its committed header)
- [ ] C++ / C# / Go / Java / R clients
- [ ] Not applicable — this does not touch the boundary

## Determinism

- [ ] `cargo test --workspace --all-features` and `--no-default-features` agree
      (the parallel and sequential paths produce identical results)
- [ ] Any randomness is drawn from a seeded `rand_pcg`, never the thread RNG
- [ ] Not applicable

## Verification

<!--
What you ran, and what it said. Paste the result lines rather than asserting
"tests pass".
-->

```
```

## Checklist

- [ ] `cargo fmt --all` and `cargo clippy --workspace --all-targets --all-features -- -D warnings` are clean
- [ ] `cargo deny check` is clean
- [ ] Tests added or updated, with hand-computed expectations for core changes
- [ ] Docs updated (`docs/`, rustdoc, README) where behaviour changed
- [ ] `CHANGELOG.md` updated under `[Unreleased]`
- [ ] Commits are signed, with no AI attribution or `Co-authored-by` trailers
