# Wickra Strategy-CI — C ABI

The C ABI is the hub every C-capable language (C, C++, C#, Go, Java, R) links
against. It exposes `strategy-ci-core` as a tiny, JSON-shaped surface built as
both a `cdylib` (dynamic library) and a `staticlib`.

## Surface

```c
#include "wickra_strategy_ci.h"

WickraStrategyCi *wickra_strategy_ci_new(void);
void              wickra_strategy_ci_free(WickraStrategyCi *handle);
int32_t           wickra_strategy_ci_command(WickraStrategyCi *handle,
                                             const char *cmd_json,
                                             char *out, size_t cap);
const char       *wickra_strategy_ci_version(void);
```

- **`wickra_strategy_ci_new`** builds a stateless session. Returns `NULL` only on
  allocation failure — tests and data are passed with each command.
- **`wickra_strategy_ci_free`** destroys a handle (null is a no-op).
- **`wickra_strategy_ci_command`** runs a command JSON and writes the response
  JSON into the caller's buffer using the length-out protocol below.
- **`wickra_strategy_ci_version`** returns a static, NUL-terminated version string
  (do not free).

## Command / response protocol

Everything goes through `wickra_strategy_ci_command`. A command is an envelope
`{"cmd":"...", ...}`; the response is always a JSON string. Commands: `run_test`,
`bless`, `run_suite`, `list`, `version`. The `test`/`tests` carry an opaque
`StrategySpec` sub-JSON forwarded verbatim to the backtest engine, and `data`
maps each symbol to its candle array.

`command` returns the response length in bytes (excluding the NUL). Size the
buffer with a first call, then read with a second:

```c
int32_t len = wickra_strategy_ci_command(h, cmd, NULL, 0);   /* length query */
char *buf = malloc((size_t)len + 1);
wickra_strategy_ci_command(h, cmd, buf, (size_t)len + 1);     /* fills buf + NUL */
```

If `len < cap` the response plus a NUL is written; otherwise `out` is untouched.
Negative returns are argument errors only: `-1` null handle/command, `-2`
non-UTF-8 command, `-3` a caught panic (never in normal use). Internal errors
(bad spec, missing data, a backtest error) come back as a non-negative-length
`{"ok":false,"error":"..."}` response, so every language parses them uniformly.

## Building

```bash
cargo build -p wickra-strategy-ci-c --release   # cdylib + staticlib in target/release/
cbindgen --config cbindgen.toml --output include/wickra_strategy_ci.h
```

The generated header is committed and must not be edited by hand.

## Determinism

The response is byte-identical to every other Wickra Strategy-CI binding and to
the reference CLI, because the whole test runner lives once in `strategy-ci-core`
and each binding forwards its JSON verbatim.

## Safety

The FFI functions are `unsafe`: `handle` must come from `..._new` and not be
freed twice; `cmd_json` must be NUL-terminated; and `out` must have `cap`
writable bytes (or be null for a length query). The release profile aborts on
panic so nothing unwinds across the boundary.

## See also

- The main project: <https://github.com/wickra-lib/wickra-strategy-ci>
- Documentation: <https://wickra.org>

## License

Dual-licensed under either [MIT](../../LICENSE-MIT) or
[Apache-2.0](../../LICENSE-APACHE), at your option.
