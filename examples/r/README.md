# Wickra Strategy-CI examples — R

Runnable R examples for the [Wickra Strategy-CI R binding](../../bindings/r). The package compiles a thin
`.Call` glue layer against the C ABI library, so build the library and install
the package first (the CI examples job does exactly this):

```bash
cargo build -p wickra-strategy-ci-c --release
R CMD INSTALL bindings/r
```

## Run

```bash
Rscript examples/r/<example>.R
```

## The examples

| Example | What it does |
|---------|--------------|
| `run.R` | A runnable R example: golden-pin a strategy with bless, then re-run it and confirm it passes. |
