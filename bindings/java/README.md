<p align="center">
  <a href="https://wickra.org"><img src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/wickra-banner.webp?v=514-7" alt="Wickra Strategy-CI — golden-pin your strategy's backtest report, catch regressions in CI, and property-test against fuzzed data, in ten languages plus a reusable GitHub Action" width="100%"></a>
</p>

[![CI](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-strategy-ci/ci.svg)](https://github.com/wickra-lib/wickra-strategy-ci/actions/workflows/ci.yml)
[![codecov](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-strategy-ci/codecov.svg)](https://codecov.io/gh/wickra-lib/wickra-strategy-ci)
[![Maven Central](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-strategy-ci/maven.svg)](https://central.sonatype.com/artifact/org.wickra/wickra-strategy-ci)
[![License: MIT OR Apache-2.0](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-strategy-ci/license.svg)](https://github.com/wickra-lib/wickra-strategy-ci#license)

# Wickra Strategy-CI — Java

---

**Jest for trading strategies — for Java. `org.wickra:wickra-strategy-ci` — prebuilt native library inside the jar, no JNI, no system dependencies.**

JVM bindings for the Wickra Strategy-CI test runner over its C ABI hub, using the
Foreign Function & Memory API (FFM / Project Panama). A `Session` drives the
deterministic core over a JSON boundary, so the result is byte-identical to every
other Wickra Strategy-CI binding.

## Requirements

- JDK 22 or newer (FFM is stable since JDK 22).
- Run with `--enable-native-access=ALL-UNNAMED`.

## Install

Maven:

```xml
<dependency>
  <groupId>org.wickra</groupId>
  <artifactId>wickra-strategy-ci</artifactId>
  <version>0.1.3</version>
</dependency>
```

Gradle:

```kotlin
implementation("org.wickra:wickra-strategy-ci:0.1.3")
```

The native library ships prebuilt per platform inside the jar and is
extracted automatically on first use. There is nothing to compile.

### Building from this repository (contributors)

```bash
cargo build -p wickra-strategy-ci-c          # stages target/debug/<lib>
mvn -q test -Dnative.lib.dir=target/debug    # or the default from the pom
```

The native library location is read from the `native.lib.dir` system property;
the pom defaults it to the workspace `target/debug` directory.

## Quick start

```java
import org.wickra.strategyci.Session;

try (Session session = new Session()) {
    String response = session.command("""
        {"cmd":"run_test","test":{ },"data":{ }}
        """);
    System.out.println(response);
}
```

### Surface

- **`new Session()`** — a stateless test session; tests and data are passed with
  each command. Implements `AutoCloseable`.
- **`session.command(cmdJson) -> String`** — run a command envelope
  (`{"cmd":"...", ...}`) and return the response JSON. Commands: `run_test`,
  `bless`, `run_suite`, `list`, `version`.
- **`Session.version() -> String`** — the crate version.

Internal errors come back as an `{"ok": false, "error": ...}` response, not as a
thrown exception.

### Determinism

The response bytes are identical across languages and between the parallel and
sequential execution paths, because the whole test runner lives once in the Rust
core and this binding forwards its JSON verbatim.

## Benchmark

Every binding forwards to the same data-driven Rust core, so what this one adds is
the call overhead of the Java Foreign Function & Memory API over the C ABI, not a different result. The core's throughput is
measured by the repository's benchmark suite and the nightly `bench.yml` run; the
numbers, the machine and how to reproduce them are in the repository
[BENCHMARKS.md](https://github.com/wickra-lib/wickra-strategy-ci/blob/main/BENCHMARKS.md).

## Documentation

The full guide, the spec reference and the API documentation live in the main
repository and the documentation site:

- **Repository:** <https://github.com/wickra-lib/wickra-strategy-ci>
- **Docs** (guides, spec reference, cookbook): <https://strategy-ci.wickra.org>
- **Runnable example:** [`examples/java/`](https://github.com/wickra-lib/wickra-strategy-ci/tree/main/examples/java)

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
