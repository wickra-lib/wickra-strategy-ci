<p align="center">
  <a href="https://wickra.org"><img src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/wickra-banner.webp?v=514-7" alt="Wickra Strategy-CI — golden-pin your strategy's backtest report, catch regressions in CI, and property-test against fuzzed data, in ten languages plus a reusable GitHub Action" width="100%"></a>
</p>

[![CI](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-strategy-ci/ci.svg)](https://github.com/wickra-lib/wickra-strategy-ci/actions/workflows/ci.yml)
[![codecov](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-strategy-ci/codecov.svg)](https://codecov.io/gh/wickra-lib/wickra-strategy-ci)
[![Go module](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-strategy-ci/go.svg)](https://pkg.go.dev/github.com/wickra-lib/wickra-strategy-ci-go)
[![License: MIT OR Apache-2.0](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-strategy-ci/license.svg)](https://github.com/wickra-lib/wickra-strategy-ci#license)

# Wickra Strategy-CI — Go

---

**Jest for trading strategies — for Go. `go get github.com/wickra-lib/wickra-strategy-ci-go` — over the C ABI via cgo, prebuilt library bundled in the module.**

Go bindings for the Wickra Strategy-CI test runner over its C ABI hub via cgo. A
`Session` drives the deterministic core over a JSON boundary, so the result is
byte-identical to every other Wickra Strategy-CI binding.

## Install

Use the published **`wickra-strategy-ci-go`** module, which bundles the prebuilt C ABI
library for every platform, so `go get` + `go build` works with no extra steps
(a C compiler is still required, as the binding uses cgo):

```bash
go get github.com/wickra-lib/wickra-strategy-ci-go
```

`wickra-strategy-ci-go` is generated from this directory by the release pipeline: it mirrors
the Go sources, the vendored C ABI header (`include/wickra_strategy_ci.h`) and the prebuilt
libraries under `lib/<goos>_<goarch>/`. On Linux/macOS the library path is baked
in via rpath; on Windows the DLL must be discoverable at run time (next to the
executable or on `PATH`).

The prebuilt C ABI library is staged per platform under `lib/<goos>_<goarch>/`
and the header is vendored under `include/`. For a local build, copy the library
built by `cargo build -p wickra-strategy-ci-c --release` into the matching
`lib/<goos>_<goarch>/` directory (on Windows, ensure that directory is on `PATH`
when running tests).

### Building from this repository (contributors)

This `bindings/go` directory is the development source. To build it directly,
compile the C ABI and stage the library into the per-platform directory cgo
links against:

```bash
cargo build -p wickra-strategy-ci-c --release
mkdir -p bindings/go/lib/linux_amd64                 # match your GOOS_GOARCH
cp target/release/libwickra_strategy_ci.so bindings/go/lib/linux_amd64/
```

Then, with the library on the loader path, run `go test ./...` from this directory.

## Quick start

```go
package main

import (
	"fmt"

	wickra "github.com/wickra-lib/wickra-strategy-ci/bindings/go"
)

func main() {
	s := wickra.New()
	defer s.Close()

	resp, err := s.Command(`{"cmd":"run_test","test":{ /* ... */ },"data":{ /* ... */ }}`)
	if err != nil {
		panic(err)
	}
	fmt.Println(resp)
}
```

### Surface

- **`New() *Session`** — a stateless test session; tests and data are passed with
  each command. Call `Close` when done.
- **`(*Session) Command(cmdJSON string) (string, error)`** — run a command
  envelope (`{"cmd":"...", ...}`) and return the response JSON. Commands:
  `run_test`, `bless`, `run_suite`, `list`, `version`.
- **`Version() string`** — the crate version.

Internal errors come back as an `{"ok": false, "error": ...}` response, not as a
returned `error`. The `error` is reserved for hard failures at the C ABI boundary.

### Determinism

The response bytes are identical across languages and between the parallel and
sequential execution paths, because the whole test runner lives once in the Rust
core and this binding forwards its JSON verbatim.

## Benchmark

Every binding forwards to the same data-driven Rust core, so what this one adds is
the call overhead of cgo over the C ABI, not a different result. The core's throughput is
measured by the repository's benchmark suite and the nightly `bench.yml` run; the
numbers, the machine and how to reproduce them are in the repository
[BENCHMARKS.md](https://github.com/wickra-lib/wickra-strategy-ci/blob/main/BENCHMARKS.md).

## Documentation

The full guide, the spec reference and the API documentation live in the main
repository and the documentation site:

- **Repository:** <https://github.com/wickra-lib/wickra-strategy-ci>
- **Docs** (guides, spec reference, cookbook): <https://strategy-ci.wickra.org>
- **Runnable example:** [`examples/go/`](https://github.com/wickra-lib/wickra-strategy-ci/tree/main/examples/go)

- The main project: <https://github.com/wickra-lib/wickra-strategy-ci>
- Documentation: <https://wickra.org>

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
