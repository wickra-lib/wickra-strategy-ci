<p align="center">
  <a href="https://wickra.org"><img src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/wickra-banner.webp?v=514" alt="Wickra Strategy-CI — golden-pin your strategy's backtest report, catch regressions in CI, and property-test against fuzzed data, in ten languages plus a reusable GitHub Action" width="100%"></a>
</p>

[![CI](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-strategy-ci/ci.svg)](https://github.com/wickra-lib/wickra-strategy-ci/actions/workflows/ci.yml)
[![codecov](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-strategy-ci/codecov.svg)](https://codecov.io/gh/wickra-lib/wickra-strategy-ci)
[![NuGet](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-strategy-ci/nuget.svg)](https://www.nuget.org/packages/Wickra.StrategyCi)
[![License: MIT OR Apache-2.0](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-strategy-ci/license.svg)](https://github.com/wickra-lib/wickra-strategy-ci#license)

# Wickra Strategy-CI — C#

---

**Jest for trading strategies — for C#. `dotnet add package Wickra.StrategyCi` — prebuilt native library, no system dependencies.**

.NET bindings for the Wickra Strategy-CI test runner over its C ABI hub. Golden-pin
a strategy's backtest report, assert per-field tolerances and invariant properties,
and fuzz-test against seeded data perturbations — byte-identical to every other
Wickra Strategy-CI binding.

## Install

```bash
dotnet add package Wickra.StrategyCi
```

The native library ships prebuilt per platform under `runtimes/<rid>/native/`,
selected automatically. There is nothing to compile. Targets .NET 8 and later.

### Building from this repository (contributors)

**What is in this directory.** | Path | What it is |
|------|------------|
| [`WickraStrategyCi/`](WickraStrategyCi/) | The library. Its [`README.md`](WickraStrategyCi/README.md) is the package description NuGet renders — usage, the resolver, the command protocol. |
| [`WickraStrategyCi.Tests/`](WickraStrategyCi.Tests/) | The test suite: the session surface, the golden corpus, and the cross-language check against `golden/expected/`. |

The two READMEs are deliberately different documents. This one is the landing
page for someone browsing the repository; the one beside the `.csproj` is
packaged into the `.nupkg` (`<PackageReadmeFile>`) and is what a reader sees on
nuget.org, so it links absolutely and assumes no checkout.

**Building and testing locally.** The binding is a thin P/Invoke layer, so it needs the C ABI library built first:

```bash
cargo build -p wickra-strategy-ci-c --release
dotnet test bindings/csharp/WickraStrategyCi.Tests/WickraStrategyCi.Tests.csproj -c Release
```

In a dev checkout the resolver probes the Cargo `target/{release,debug}` tree, so
no staging step is needed. In a published package the native library comes from
the NuGet `runtimes/<rid>/native/` payload, staged by the release pipeline for
`win-x64`, `win-arm64`, `linux-x64`, `linux-arm64`, `osx-x64` and `osx-arm64`.

## Quick start

### The surface

One type. `Session` is `IDisposable` and drives the core over JSON:

```csharp
using Wickra.StrategyCi;

using var session = new Session();
string response = session.Command(commandJson);
```

Domain errors — an unknown command, a malformed test — come back in-band as
`{"ok":false,...}` rather than as exceptions, because they are data about the
request. An exception means the call itself could not be made.

See [`docs/TESTS.md`](../../docs/TESTS.md) for the command envelope and
[`../../README.md`](../../README.md) for the project overview.

## Benchmark

Every binding forwards to the same data-driven Rust core, so what this one adds is
the call overhead of `[LibraryImport]` P/Invoke over the C ABI, not a different result. The core's throughput is
measured by the repository's benchmark suite and the nightly `bench.yml` run; the
numbers, the machine and how to reproduce them are in the repository
[BENCHMARKS.md](https://github.com/wickra-lib/wickra-strategy-ci/blob/main/BENCHMARKS.md).

## Documentation

The full guide, the spec reference and the API documentation live in the main
repository and the documentation site:

- **Repository:** <https://github.com/wickra-lib/wickra-strategy-ci>
- **Docs** (guides, spec reference, cookbook): <https://strategy-ci.wickra.org>
- **Runnable example:** [`examples/csharp/`](https://github.com/wickra-lib/wickra-strategy-ci/tree/main/examples/csharp)

Wickra Strategy-CI ships native bindings for Python, Node.js, WASM and Rust, plus a C ABI hub that any
C-capable language (C, C++, C#, Go, Java, R) links against — all forwarding to the
same data-driven, `unsafe`-forbidden Rust core.

## Security

Found a security issue? **Please don't open a public issue.** Report it privately
via the repository's *Security* tab (*"Report a vulnerability"*) or email
**support@wickra.org** with a subject line starting `[wickra security]`. Full
policy: <https://github.com/wickra-lib/wickra-strategy-ci/blob/main/SECURITY.md>.

## Disclaimer

`wickra-strategy-ci` is research and engineering tooling, not financial advice. A
passing test attests only that a strategy's backtest report matches its pinned
expectation under the given data — it makes no claim about the quality,
profitability or future performance of any strategy. Trading carries risk; you
are responsible for your own decisions.

## License

Licensed under either of [Apache-2.0](https://github.com/wickra-lib/wickra-strategy-ci/blob/main/LICENSE-APACHE)
or [MIT](https://github.com/wickra-lib/wickra-strategy-ci/blob/main/LICENSE-MIT) at your option.
