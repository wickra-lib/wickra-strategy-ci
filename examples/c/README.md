# Wickra Strategy-CI — C / C++ examples

The Wickra Strategy-CI C ABI is a single shared/static library plus a generated header
([`bindings/c/include/wickra_strategy_ci.h`](../../bindings/c/include/wickra_strategy_ci.h)). Any C-capable
language links against the same artifact; these examples show the plain-C path
and, through [`wickra_strategy_ci.hpp`](../../bindings/c/include/wickra_strategy_ci.hpp), the C++ one.

## Build the library

From the workspace root:

```sh
cargo build -p wickra-strategy-ci-c --release
```

This produces, in `target/release/`:

| Platform | Shared library | Link target |
|----------|----------------|-------------|
| Linux    | `libwickra_strategy_ci.so`     | `-lwickra_strategy_ci` |
| macOS    | `libwickra_strategy_ci.dylib`  | `-lwickra_strategy_ci` |
| Windows (MSVC) | `wickra_strategy_ci.dll` | `wickra_strategy_ci.dll.lib` (import lib) |

A static library (`libwickra_strategy_ci.a` / `wickra_strategy_ci.lib`) is emitted alongside.

## Build and run the examples

With CMake, as the CI C ABI job does:

```sh
cmake -S examples/c -B examples/c/build
cmake --build examples/c/build --config Release
ctest --test-dir examples/c/build -C Release --output-on-failure
```

## The examples

| Example | What it does |
|---------|--------------|
| `run.c` | A runnable C example against the wickra-strategy-ci C ABI: run a golden test |
| `run.cpp` | A runnable C++ example against the wickra-strategy-ci C ABI: bless a test and confirm the golden is pinned. |

## Usage shape

Every call follows the same handle discipline: construct from a spec JSON, drive
with command JSON, read the response, free the handle exactly once. `wickra_strategy_ci.h` is
the whole contract; the C++ header, where one ships, wraps the handle in a
move-only RAII type. See [`bindings/c/README.md`](../../bindings/c/README.md).
