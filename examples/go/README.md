# Wickra Strategy-CI examples — Go

Runnable Go examples for the [Wickra Strategy-CI Go binding](../../bindings/go). The binding links against the
prebuilt C ABI library, so build and stage it once before running anything:

```bash
cargo build -p wickra-strategy-ci-c --release
cp target/release/libwickra_strategy_ci.so bindings/go/lib/linux_amd64/   # match your GOOS_GOARCH
```

## Run

As the CI examples job runs it, from the repository root:

```bash
cd examples/go && go run .
```

## The examples

| Example | What it does |
|---------|--------------|
| `run.go` | A runnable Go example: golden-pin a strategy with `bless`, then re-run it and confirm it passes. |
