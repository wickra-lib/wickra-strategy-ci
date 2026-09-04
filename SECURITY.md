# Security Policy

`wickra-strategy-ci` is test tooling: it replays recorded market data through the
`wickra-backtest` engine and compares the resulting report to a pinned one. It
places no orders, holds no credentials, and opens no network connections. The
attack surface is therefore narrow — principally the parsing of untrusted test
JSON, `StrategySpec` sub-JSON and OHLCV data as it crosses the C ABI and WASM
boundary, and the fact that the tool is typically run inside CI on
attacker-influenced pull-request content. See
[THREAT_MODEL.md](THREAT_MODEL.md) for the asset inventory and trust
boundaries.

## Supported versions

This project is pre-release: no version has shipped to a registry yet. Security
fixes target the `main` branch, and will target the most recent published version
once `0.1.0` is released.

| Version | Supported |
|---------|-----------|
| `main` | ✅ |
| `0.1.0` (unreleased) | ✅ |

## Reporting a vulnerability

**Please do not open a public issue, pull request or discussion for security
problems.** Report privately through either channel:

- GitHub → the repository's **Security** tab → **Report a vulnerability**
  (private advisory), or
- email **support@wickra.org**.

Include a description, affected version/commit, reproduction steps and impact.

We aim to acknowledge within a few days, agree a disclosure timeline, and credit
reporters who wish to be named once a fix ships.

## Scope

In scope: memory-safety or panic-across-FFI flaws in the C ABI hub and its
buffer protocol, denial-of-service through a hostile `StrategyTest`, dataset or
`StrategySpec` (for example unbounded allocation while parsing), and any input
that makes a binding return a corrupted or non-deterministic report. Out of
scope: incorrect backtest mathematics (a functional bug in the engine, not a
vulnerability here) and advisories in third-party crates that are already tracked
and triaged.

## Vulnerability disclosure (VEX)

This repository ships a machine-readable VEX record in
[`osv-scanner.toml`](osv-scanner.toml), kept in lock-step with the cargo-deny
advisory ignore list in [`deny.toml`](deny.toml). Any advisory assessed as not
affecting `wickra-strategy-ci` is documented there with a reason, so downstream
scanners see an explicit, auditable justification rather than an unexplained
suppression.
