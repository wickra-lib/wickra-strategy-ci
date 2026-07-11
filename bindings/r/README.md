# Wickra Strategy-CI — R

R bindings for the Wickra Strategy-CI test runner over its C ABI hub, via `.Call`.
A session drives the deterministic core over a JSON boundary, so the result is
byte-identical to every other Wickra Strategy-CI binding.

## Build & test

The C ABI header and shared library are provided out-of-tree through two
environment variables (set by CI / the installer):

```bash
export WKSTRATEGYCI_INC=/path/to/bindings/c/include   # the header dir
export WKSTRATEGYCI_LIB=/path/to/target/release       # the library dir
R CMD INSTALL bindings/r
Rscript bindings/r/tests/run_tests.R
```

At run time the loader must find the shared library on `LD_LIBRARY_PATH`
(Linux), `DYLD_LIBRARY_PATH` (macOS) or `PATH` (Windows).

## Usage

```r
library(wickrastrategyci)

session <- wkstrategyci_new()
response <- wkstrategyci_command(session, '{"cmd":"run_test","test":{ },"data":{ }}')
cat(response)
```

## Surface

- **`wkstrategyci_new()`** — create a stateless test session (an external
  pointer; freed by a finalizer). Tests and data are passed with each command.
- **`wkstrategyci_command(session, cmd_json)`** — run a command envelope
  (`{"cmd":"...", ...}`) and return the response JSON. Commands: `run_test`,
  `bless`, `run_suite`, `list`, `version`.
- **`wkstrategyci_version()`** — the crate version.

Internal errors come back as an `{"ok": false, "error": ...}` response, not as an
R error.

## Determinism

The response bytes are identical across languages and between the parallel and
sequential execution paths, because the whole test runner lives once in the Rust
core and this binding forwards its JSON verbatim.

## See also

- The main project: <https://github.com/wickra-lib/wickra-strategy-ci>
- Documentation: <https://wickra.org>

## License

Dual-licensed under either MIT or Apache-2.0, at your option.
