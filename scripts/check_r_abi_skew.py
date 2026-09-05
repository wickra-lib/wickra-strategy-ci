#!/usr/bin/env python3
"""Assert that the R binding can link against the C ABI its version names.

Every other binding ships its native code in the same artifact as its wrapper, so
the two can never disagree. R is the exception: `bindings/r/configure` downloads
a prebuilt `wickra-strategy-ci-c-<triple>.tar.gz` from the GitHub release named
by `DESCRIPTION: Version`, and compiles `src/wickra_strategy_ci.c` against it.
The wrapper comes from the working tree; the ABI comes from a published release.

Our own CI never sees that pairing. The R job sets WKSTRATEGYCI_INC and
WKSTRATEGYCI_LIB, which takes configure's dev-override branch and builds against
the header and library in the tree -- where they match by construction. So a
green R job says nothing about the path a real installation takes.

Two failures are possible and neither is visible from a build:

1. **The release does not exist yet.** During the window between bumping
   `DESCRIPTION` and pushing the tag, `configure` points at an asset nobody has
   published. That is expected and is reported here as a warning, not an error --
   it is the normal state of the repository between a bump and a release.

2. **The wrapper calls a symbol the named ABI does not export.** This is the
   real skew: `src/wickra_strategy_ci.c` is compiled against a *downloaded*
   header, so a function added to the tree's header but not present in the
   released one is a link error on the user's machine and nowhere else.

    python scripts/check_r_abi_skew.py            # offline checks only
    python scripts/check_r_abi_skew.py --release  # also ask GitHub for the asset

Run from the repository root.
"""

from __future__ import annotations

import json
import os
import re
import sys
import urllib.error
import urllib.request

ROOT = os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)), ".."))
REPO = "wickra-lib/wickra-strategy-ci"

# The triples configure knows how to resolve. A platform the wrapper claims to
# support but the release does not build for is a silent install failure.
TRIPLES = (
    "x86_64-unknown-linux-gnu",
    "aarch64-unknown-linux-gnu",
    "x86_64-apple-darwin",
    "aarch64-apple-darwin",
    "x86_64-pc-windows-msvc",
    "aarch64-pc-windows-msvc",
)


def read(path: str) -> str:
    with open(os.path.join(ROOT, path), encoding="utf-8") as handle:
        return handle.read()


def strip_comments(source: str) -> str:
    """Drop block and line comments so a symbol named in prose is not a call."""
    source = re.sub(r"/\*.*?\*/", " ", source, flags=re.S)
    return re.sub(r"//[^\n]*", " ", source)


def main() -> int:
    version = re.search(r"^Version:\s*(\S+)", read("bindings/r/DESCRIPTION"), re.M)
    if not version:
        print("bindings/r/DESCRIPTION has no Version:", file=sys.stderr)
        return 1
    version = version.group(1)
    tag = f"v{version}"
    print(f"  DESCRIPTION names {tag}")

    failures: list[str] = []

    # --- 1. the wrapper only calls what the header declares -----------------
    header = read("bindings/c/include/wickra_strategy_ci.h")
    declared = set(re.findall(r"\b(wickra_strategy_ci_[a-z_]+)\s*\(", header))
    glue = strip_comments(read("bindings/r/src/wickra_strategy_ci.c"))
    called = set(re.findall(r"\b(wickra_strategy_ci_[a-z_]+)\s*\(", glue))
    undeclared = sorted(called - declared)
    if undeclared:
        failures.append(
            "the R glue calls symbols the C ABI header does not declare: "
            + ", ".join(undeclared))
    else:
        print(f"  glue calls {len(called)} symbol(s), all declared by the header")

    # --- 2. configure agrees with the release layout ------------------------
    configure = read("bindings/r/configure")
    for triple in TRIPLES:
        if triple not in configure and triple not in read("bindings/r/configure.win"):
            failures.append(f"configure resolves no asset for {triple}")
    asset = f"wickra-strategy-ci-c-${{triple}}.tar.gz".replace("${triple}", "<triple>")
    if "wickra-strategy-ci-c-" not in configure:
        failures.append(f"configure does not name the {asset} release asset")
    else:
        print(f"  configure resolves {len(TRIPLES)} triples of {asset}")

    # --- 3. optionally: does that release actually carry them? --------------
    if "--release" in sys.argv:
        url = f"https://api.github.com/repos/{REPO}/releases/tags/{tag}"
        try:
            with urllib.request.urlopen(url, timeout=30) as response:
                names = {a["name"] for a in json.load(response).get("assets", [])}
        except urllib.error.HTTPError as error:
            if error.code == 404:
                print(f"  note: release {tag} does not exist yet -- expected "
                      f"between a version bump and its tag")
                names = None
            else:
                raise
        if names is not None:
            for triple in TRIPLES:
                name = f"wickra-strategy-ci-c-{triple}.tar.gz"
                if name not in names:
                    failures.append(f"release {tag} carries no {name}")
            if not failures:
                print(f"  release {tag} carries all {len(TRIPLES)} C ABI assets")

    if failures:
        print("\nR binding would not link against the ABI it names:", file=sys.stderr)
        for line in failures:
            print(f"  {line}", file=sys.stderr)
        return 1

    print("\nthe R wrapper and the C ABI it names agree")
    return 0


if __name__ == "__main__":
    sys.exit(main())
