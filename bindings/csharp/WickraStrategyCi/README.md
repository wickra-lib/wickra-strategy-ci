# Wickra Strategy-CI — C#

.NET bindings for the Wickra Strategy-CI test runner over its C ABI hub. A
`Session` drives the deterministic core over a JSON boundary, so the result is
byte-identical to every other Wickra Strategy-CI binding.

## Install

```bash
dotnet add package Wickra.StrategyCi
```

The correct native library is resolved automatically per platform from the
NuGet `runtimes/<rid>/native/` payload; in a dev checkout the resolver also
probes the Cargo `target/{release,debug}` tree.

## Usage

```csharp
using Wickra.StrategyCi;

using var session = new Session();
string response = session.Command("""
{
  "cmd": "run_test",
  "test": {
    "id": "momentum",
    "strategy": { },
    "dataset_ref": "sym-01",
    "property_checks": [{ "kind": "no_nan" }]
  },
  "data": { "sym-01": [ ] }
}
""");
Console.WriteLine(response);
```

## Surface

- **`new Session()`** — a stateless test session; tests and data are passed with
  each command. Implements `IDisposable`.
- **`session.Command(cmdJson) -> string`** — run a command envelope
  (`{"cmd":"...", ...}`) and return the response JSON. Commands: `run_test`,
  `bless`, `run_suite`, `list`, `version`.
- **`Session.Version() -> string`** — the crate version.

Internal errors come back as an `{"ok": false, "error": ...}` response, not as a
thrown exception.

## Determinism

The response bytes are identical across languages and between the parallel and
sequential execution paths, because the whole test runner lives once in the Rust
core and this binding forwards its JSON verbatim.

## See also

- The main project: <https://github.com/wickra-lib/wickra-strategy-ci>
- Documentation: <https://wickra.org>

## License

Dual-licensed under either MIT or Apache-2.0, at your option.
