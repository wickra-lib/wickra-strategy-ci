# Wickra Strategy-CI — C#

.NET bindings for the Wickra Strategy-CI test runner over its C ABI hub. Golden-pin
a strategy's backtest report, assert per-field tolerances and invariant properties,
and fuzz-test against seeded data perturbations — byte-identical to every other
Wickra Strategy-CI binding.

```bash
dotnet add package Wickra.StrategyCi
```

## What is in this directory

| Path | What it is |
|------|------------|
| [`WickraStrategyCi/`](WickraStrategyCi/) | The library. Its [`README.md`](WickraStrategyCi/README.md) is the package description NuGet renders — usage, the resolver, the command protocol. |
| [`WickraStrategyCi.Tests/`](WickraStrategyCi.Tests/) | The test suite: the session surface, the golden corpus, and the cross-language check against `golden/expected/`. |

The two READMEs are deliberately different documents. This one is the landing
page for someone browsing the repository; the one beside the `.csproj` is
packaged into the `.nupkg` (`<PackageReadmeFile>`) and is what a reader sees on
nuget.org, so it links absolutely and assumes no checkout.

## Building and testing locally

The binding is a thin P/Invoke layer, so it needs the C ABI library built first:

```bash
cargo build -p wickra-strategy-ci-c --release
dotnet test bindings/csharp/WickraStrategyCi.Tests/WickraStrategyCi.Tests.csproj -c Release
```

In a dev checkout the resolver probes the Cargo `target/{release,debug}` tree, so
no staging step is needed. In a published package the native library comes from
the NuGet `runtimes/<rid>/native/` payload, staged by the release pipeline for
`win-x64`, `win-arm64`, `linux-x64`, `linux-arm64`, `osx-x64` and `osx-arm64`.

## The surface

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
