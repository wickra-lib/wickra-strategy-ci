<p align="center">
  <a href="https://wickra.org"><img src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/wickra-banner.webp?v=514" alt="Wickra Strategy-CI — golden-pin your strategy's backtest report, catch regressions in CI, and property-test against fuzzed data, in ten languages plus a reusable GitHub Action" width="100%"></a>
</p>

[![CI](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-strategy-ci/ci.svg)](https://github.com/wickra-lib/wickra-strategy-ci/actions/workflows/ci.yml)
[![codecov](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-strategy-ci/codecov.svg)](https://codecov.io/gh/wickra-lib/wickra-strategy-ci)
[![GitHub release](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-strategy-ci/release.svg)](https://github.com/wickra-lib/wickra-strategy-ci/releases/latest)
[![License: MIT OR Apache-2.0](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-strategy-ci/license.svg)](https://github.com/wickra-lib/wickra-strategy-ci#license)

# Wickra Strategy-CI — C / C++

---

**Jest for trading strategies — for C / C++. `cargo build -p wickra-strategy-ci-c --release` — a prebuilt shared/static library plus a generated `wickra_strategy_ci.h`, no system dependencies.**

The C ABI is the hub every C-capable language (C, C++, C#, Go, Java, R) links
against. It exposes `wickra-strategy-ci-core` as a tiny, JSON-shaped surface built as
both a `cdylib` (dynamic library) and a `staticlib`.

## Install

Grab the prebuilt header + library for your platform from the
[GitHub releases](https://github.com/wickra-lib/wickra-strategy-ci/releases) — each archive
has `wickra_strategy_ci.h`, the C++ wrapper where the binding ships one, and the shared/static
library — or build from source:

```bash
cargo build -p wickra-strategy-ci-c --release
# -> target/release/libwickra_strategy_ci.{so,dylib} or wickra_strategy_ci.dll (+ import lib) + a staticlib
```

Then compile against the header and link the library.

### Building from this repository (contributors)

```bash
cargo build -p wickra-strategy-ci-c --release   # cdylib + staticlib in target/release/
cbindgen --config cbindgen.toml --output include/wickra_strategy_ci.h
```

The generated header is committed and must not be edited by hand.

## Quick start

[`examples/c/run.c`](https://github.com/wickra-lib/wickra-strategy-ci/blob/main/examples/c/run.c) is the runnable example the CI smoke job executes; in full:

```c
/* A runnable C example against the wickra-strategy-ci C ABI: run a golden test
 * and confirm it passes. The response comes back as JSON via the length-out
 * protocol — a first call learns the length, the second fills the buffer. */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "wickra_strategy_ci.h"

static const char *CMD =
    "{\"cmd\":\"bless\",\"test\":{\"id\":\"ema_crossover\",\"strategy\":{"
    "\"symbol\":\"AAA\",\"timeframe\":\"1h\","
    "\"indicators\":{\"fast\":{\"type\":\"Ema\",\"params\":[3]},"
    "\"slow\":{\"type\":\"Ema\",\"params\":[8]}},"
    "\"entry\":{\"cross_above\":[\"fast\",\"slow\"]},"
    "\"exit\":{\"cross_below\":[\"fast\",\"slow\"]},"
    "\"sizing\":{\"type\":\"fixed_fraction\",\"fraction\":0.95}},"
    "\"dataset_ref\":\"AAA\",\"property_checks\":[{\"kind\":\"no_nan\"}]},"
    "\"data\":{\"AAA\":["
    "{\"time\":1700000000,\"open\":120,\"high\":121,\"low\":119,\"close\":120,\"volume\":1000},"
    "{\"time\":1700003600,\"open\":120,\"high\":121,\"low\":117,\"close\":118,\"volume\":1000},"
    "{\"time\":1700007200,\"open\":118,\"high\":119,\"low\":115,\"close\":116,\"volume\":1000},"
    "{\"time\":1700010800,\"open\":116,\"high\":117,\"low\":113,\"close\":114,\"volume\":1000},"
    "{\"time\":1700014400,\"open\":114,\"high\":115,\"low\":111,\"close\":112,\"volume\":1000},"
    "{\"time\":1700018000,\"open\":112,\"high\":113,\"low\":109,\"close\":110,\"volume\":1000},"
    "{\"time\":1700021600,\"open\":110,\"high\":111,\"low\":107,\"close\":108,\"volume\":1000},"
    "{\"time\":1700025200,\"open\":108,\"high\":113,\"low\":107,\"close\":112,\"volume\":1000},"
    "{\"time\":1700028800,\"open\":112,\"high\":117,\"low\":111,\"close\":116,\"volume\":1000},"
    "{\"time\":1700032400,\"open\":116,\"high\":121,\"low\":115,\"close\":120,\"volume\":1000},"
    "{\"time\":1700036000,\"open\":120,\"high\":125,\"low\":119,\"close\":124,\"volume\":1000},"
    "{\"time\":1700039600,\"open\":124,\"high\":129,\"low\":123,\"close\":128,\"volume\":1000}"
    "]}}";

int main(void) {
    printf("wickra-strategy-ci %s\n", wickra_strategy_ci_version());

    WickraStrategyCi *session = wickra_strategy_ci_new();
    if (!session) {
        fprintf(stderr, "failed to create session\n");
        return 1;
    }

    int32_t len = wickra_strategy_ci_command(session, CMD, NULL, 0);
    if (len < 0) {
        fprintf(stderr, "command failed (code %d)\n", len);
        wickra_strategy_ci_free(session);
        return 1;
    }
    char *buf = (char *)malloc((size_t)len + 1);
    wickra_strategy_ci_command(session, CMD, buf, (size_t)len + 1);

    /* The blessed test carries an expected report; a fresh bless always has one. */
    int ok = strstr(buf, "\"expected\"") != NULL;
    printf("blessed test: %s\n", ok ? "PASS (golden pinned)" : "FAIL");

    free(buf);
    wickra_strategy_ci_free(session);
    return ok ? 0 : 1;
}
```

### Surface

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

### Command / response protocol

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

### Determinism

The response is byte-identical to every other Wickra Strategy-CI binding and to
the reference CLI, because the whole test runner lives once in `wickra-strategy-ci-core`
and each binding forwards its JSON verbatim.

### Safety

The FFI functions are `unsafe`: `handle` must come from `..._new` and not be
freed twice; `cmd_json` must be NUL-terminated; and `out` must have `cap`
writable bytes (or be null for a length query). The release profile aborts on
panic so nothing unwinds across the boundary.

## Benchmark

Every binding forwards to the same data-driven Rust core, so what this one adds is
the call overhead of the C ABI itself, not a different result. The core's throughput is
measured by the repository's benchmark suite and the nightly `bench.yml` run; the
numbers, the machine and how to reproduce them are in the repository
[BENCHMARKS.md](https://github.com/wickra-lib/wickra-strategy-ci/blob/main/BENCHMARKS.md).

## Documentation

The full guide, the spec reference and the API documentation live in the main
repository and the documentation site:

- **Repository:** <https://github.com/wickra-lib/wickra-strategy-ci>
- **Docs** (guides, spec reference, cookbook): <https://strategy-ci.wickra.org>
- **Runnable example:** [`examples/c/`](https://github.com/wickra-lib/wickra-strategy-ci/tree/main/examples/c)

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
