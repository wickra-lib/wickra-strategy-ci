#!/usr/bin/env python3
"""Assert that every binding exposes the same session surface.

Each binding is written separately and each has its own test suite, so a method
that goes missing in one of them fails nowhere: its tests simply stop exercising
it. Nothing compared the bindings *to each other*.

The C ABI header is the source of truth. Every binding is a consumer of it, so
this reads the exported symbols out of `bindings/c/include/wickra_strategy_ci.h`
and checks that each binding surfaces the operations they stand for.

Two operations are conditional on the language, not on the contract:

- **create** is a constructor in every object binding; only C and R name it.
- **release** exists where the caller owns the handle (C, Go, Java, C#, R) and
  is absent where the runtime owns it (Python, Node, WASM), because a
  garbage-collected binding has nothing for a caller to release. Requiring it
  everywhere would report a language property as drift; so a binding that owns
  handles must have it, and one that does not must not grow one.

Bindings may expose more than the contract -- an idiomatic helper is not drift --
so this checks that the contract is present, not that nothing else is.

WASM is the one binding whose published surface lives in a build artifact rather
than in the repository. Its source is read here all the same, since the
`#[wasm_bindgen]` items are what the artifact is generated from.

Run from the repository root:  python scripts/check_binding_surface.py
"""

from __future__ import annotations

import os
import re
import sys

ROOT = os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)), ".."))

# The operations the C ABI names, and the symbol each is spelled with.
ABI = {
    "create": "wickra_strategy_ci_new",
    "release": "wickra_strategy_ci_free",
    "command": "wickra_strategy_ci_command",
    "version": "wickra_strategy_ci_version",
}

# Bindings where the caller owns the handle and must be able to release it.
# Elsewhere the runtime owns it, and a release method would be a wart.
OWNS_HANDLE = {"c", "go", "java", "csharp", "r"}


def read(path: str) -> str:
    with open(os.path.join(ROOT, path), encoding="utf-8") as handle:
        return handle.read()


def truth() -> set[str]:
    """The operations the C ABI header actually exports."""
    header = read("bindings/c/include/wickra_strategy_ci.h")
    exported = set(re.findall(r"\b(wickra_strategy_ci_[a-z_]+)\s*\(", header))
    missing = [op for op, sym in ABI.items() if sym not in exported]
    if missing:
        raise SystemExit(
            "the C ABI header is missing " + ", ".join(sorted(missing)) +
            " -- the contract itself is broken, not a binding")
    return set(ABI)


# --- one extractor per binding: the operations its source exposes ------------
#
# Each reads the binding's own public surface in its own spelling. They are
# deliberately literal: a regex that matched too loosely would report a mention
# in a comment as an implementation.

def surface_c() -> set[str]:
    src = read("bindings/c/src/lib.rs")
    # `unsafe` sits between `pub` and `extern` on the two entry points that take
    # raw pointers, so it is optional in the pattern rather than absent from it.
    return {op for op, sym in ABI.items()
            if re.search(rf'pub (?:unsafe )?extern "C" fn {sym}\b', src)}


def surface_python() -> set[str]:
    src = read("bindings/python/src/lib.rs")
    found = {"create"} if re.search(r"fn new\(\)", src) else set()
    if re.search(r"fn command\(", src):
        found.add("command")
    if re.search(r"fn version\(", src):
        found.add("version")
    return found


def surface_node() -> set[str]:
    src = read("bindings/node/src/lib.rs")
    found = {"create"} if "napi(constructor)" in src else set()
    if re.search(r"pub fn command\(", src):
        found.add("command")
    if re.search(r"pub fn version\(", src):
        found.add("version")
    return found


def surface_wasm() -> set[str]:
    src = read("bindings/wasm/src/lib.rs")
    found = {"create"} if "wasm_bindgen(constructor)" in src else set()
    if re.search(r"pub fn command\(", src):
        found.add("command")
    if re.search(r"pub fn (instance_)?version\(", src):
        found.add("version")
    return found


def surface_go() -> set[str]:
    src = read("bindings/go/wickra.go")
    found = set()
    if re.search(r"^func New\(\)", src, re.M):
        found.add("create")
    if re.search(r"^func \(s \*Session\) Command\(", src, re.M):
        found.add("command")
    if re.search(r"^func \(s \*Session\) Close\(", src, re.M):
        found.add("release")
    if re.search(r"^func Version\(\)", src, re.M):
        found.add("version")
    return found


def surface_java() -> set[str]:
    src = read("bindings/java/src/main/java/org/wickra/strategyci/Session.java")
    found = set()
    if re.search(r"public Session\(\)", src):
        found.add("create")
    if re.search(r"public String command\(", src):
        found.add("command")
    if re.search(r"public void close\(\)", src):
        found.add("release")
    if re.search(r"public static String version\(\)", src):
        found.add("version")
    return found


def surface_csharp() -> set[str]:
    src = read("bindings/csharp/WickraStrategyCi/Session.cs")
    found = set()
    if re.search(r"public Session\(\)", src):
        found.add("create")
    if re.search(r"public string Command\(", src):
        found.add("command")
    if re.search(r"public void Dispose\(\)", src):
        found.add("release")
    if re.search(r"public static string Version\(\)", src):
        found.add("version")
    return found


def surface_r() -> set[str]:
    src = read("bindings/r/R/session.R")
    found = set()
    if re.search(r"^wkstrategyci_new <- function", src, re.M):
        found.add("create")
    if re.search(r"^wkstrategyci_command <- function", src, re.M):
        found.add("command")
    if re.search(r"^wkstrategyci_version <- function", src, re.M):
        found.add("version")
    # R releases through the external pointer's finalizer, registered in the C
    # glue rather than exposed as an R function.
    if "R_RegisterCFinalizer" in read("bindings/r/src/wickra_strategy_ci.c"):
        found.add("release")
    return found


EXTRACTORS = {
    "c": surface_c,
    "python": surface_python,
    "node": surface_node,
    "wasm": surface_wasm,
    "go": surface_go,
    "java": surface_java,
    "csharp": surface_csharp,
    "r": surface_r,
}


def main() -> int:
    contract = truth()
    failures: list[str] = []

    for binding, extract in sorted(EXTRACTORS.items()):
        expected = set(contract)
        if binding not in OWNS_HANDLE:
            expected.discard("release")

        found = extract()

        missing = sorted(expected - found)
        if missing:
            failures.append(f"{binding}: does not expose {', '.join(missing)}")

        # The other direction: a garbage-collected binding that grew a release
        # method is drift too, and a quieter kind.
        if binding not in OWNS_HANDLE and "release" in found:
            failures.append(
                f"{binding}: exposes release, but its runtime owns the handle")

        status = "ok" if not missing else "FAIL"
        print(f"  {binding:8} {status:4} {', '.join(sorted(found)) or '-'}")

    if failures:
        print("\nbinding surface drift:", file=sys.stderr)
        for line in failures:
            print(f"  {line}", file=sys.stderr)
        return 1

    print(f"\nall {len(EXTRACTORS)} bindings expose the C ABI contract")
    return 0


if __name__ == "__main__":
    sys.exit(main())
