# Wickra Strategy-CI examples — C#

Runnable C# examples for the [Wickra Strategy-CI C# binding](../../bindings/csharp). The binding consumes the C ABI
library through P/Invoke, so build it once before running anything:

```bash
cargo build -p wickra-strategy-ci-c --release
```

## Run

As the CI examples job runs it, from the repository root:

```bash
dotnet run --project examples/csharp/<Example>
```

## The examples

| Example | What it does |
|---------|--------------|
| `Run/Program.cs` | A runnable C# example: golden-pin a strategy with `bless`, then re-run it and confirm it passes. |
