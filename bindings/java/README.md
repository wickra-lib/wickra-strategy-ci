# Wickra Strategy-CI — Java

JVM bindings for the Wickra Strategy-CI test runner over its C ABI hub, using the
Foreign Function & Memory API (FFM / Project Panama). A `Session` drives the
deterministic core over a JSON boundary, so the result is byte-identical to every
other Wickra Strategy-CI binding.

## Requirements

- JDK 22 or newer (FFM is stable since JDK 22).
- Run with `--enable-native-access=ALL-UNNAMED`.

## Build & test

```bash
cargo build -p wickra-strategy-ci-c          # stages target/debug/<lib>
mvn -q test -Dnative.lib.dir=target/debug    # or the default from the pom
```

The native library location is read from the `native.lib.dir` system property;
the pom defaults it to the workspace `target/debug` directory.

## Usage

```java
import org.wickra.strategyci.Session;

try (Session session = new Session()) {
    String response = session.command("""
        {"cmd":"run_test","test":{ },"data":{ }}
        """);
    System.out.println(response);
}
```

## Surface

- **`new Session()`** — a stateless test session; tests and data are passed with
  each command. Implements `AutoCloseable`.
- **`session.command(cmdJson) -> String`** — run a command envelope
  (`{"cmd":"...", ...}`) and return the response JSON. Commands: `run_test`,
  `bless`, `run_suite`, `list`, `version`.
- **`Session.version() -> String`** — the crate version.

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
