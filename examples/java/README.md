# Wickra Strategy-CI examples — Java

Runnable Java examples for the [Wickra Strategy-CI Java binding](../../bindings/java). The binding reaches the C ABI
through the Foreign Function & Memory API (JDK 22+), so build the library once
and point the JVM at it with `-Dnative.lib.dir`:

```bash
cargo build -p wickra-strategy-ci-c --release
```

## Run

As the CI examples job runs it, from the repository root:

```bash
javac ... && java --enable-native-access=ALL-UNNAMED ...
```

## The examples

| Example | What it does |
|---------|--------------|
| `Run.java` | A runnable Java example: golden-pin a strategy with `bless`, then re-run it and confirm it passes. |
