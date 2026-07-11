# Wickra Strategy-CI — Go

Go bindings for the Wickra Strategy-CI test runner over its C ABI hub via cgo. A
`Session` drives the deterministic core over a JSON boundary, so the result is
byte-identical to every other Wickra Strategy-CI binding.

## Install

```bash
go get github.com/wickra-lib/wickra-strategy-ci/bindings/go
```

The prebuilt C ABI library is staged per platform under `lib/<goos>_<goarch>/`
and the header is vendored under `include/`. For a local build, copy the library
built by `cargo build -p wickra-strategy-ci-c --release` into the matching
`lib/<goos>_<goarch>/` directory (on Windows, ensure that directory is on `PATH`
when running tests).

## Usage

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

## Surface

- **`New() *Session`** — a stateless test session; tests and data are passed with
  each command. Call `Close` when done.
- **`(*Session) Command(cmdJSON string) (string, error)`** — run a command
  envelope (`{"cmd":"...", ...}`) and return the response JSON. Commands:
  `run_test`, `bless`, `run_suite`, `list`, `version`.
- **`Version() string`** — the crate version.

Internal errors come back as an `{"ok": false, "error": ...}` response, not as a
returned `error`. The `error` is reserved for hard failures at the C ABI boundary.

## Determinism

The response bytes are identical across languages and between the parallel and
sequential execution paths, because the whole test runner lives once in the Rust
core and this binding forwards its JSON verbatim.

## See also

- The main project: <https://github.com/wickra-lib/wickra-strategy-ci>
- Documentation: <https://wickra.org>

## License

Dual-licensed under either [MIT](../../LICENSE-MIT) or
[Apache-2.0](../../LICENSE-APACHE), at your option.
