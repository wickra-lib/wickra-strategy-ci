#!/usr/bin/env python3
"""Binding READMEs must not use repository-relative links.

Each `bindings/*/README.md` is, or is one workflow line away from being, the long
description of a published package: PyPI renders the Python one, NuGet the C#
one, pkg.go.dev the Go one, r-universe the R one. A link like
`../../docs/TESTS.md` resolves on GitHub and nowhere else -- on a registry page
it is simply broken, and nothing in the build says so, because the file it points
at does exist in the repository.

So the rule is: anything that ships as package metadata links absolutely. The
repository's own README is exempt and deliberately keeps relative links -- it is
read in the repository, where they resolve.

A relative link *within the same binding directory* is allowed: it travels with
the package, so it still resolves wherever the package is unpacked. Only links
that climb out of the directory are broken by publishing.

Run from the repository root:  python scripts/check_readme_links.py
"""

from __future__ import annotations

import glob
import os
import re
import sys

ROOT = os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)), ".."))

# `bindings/csharp/README.md` is the directory's landing page and is read in the
# repository; the packaged description is the one beside the .csproj.
NOT_PACKAGED = {"bindings/csharp/README.md"}

LINK = re.compile(r"\[[^\]]*\]\(([^)]+)\)")


def main() -> int:
    failures: list[str] = []
    checked = 0

    candidates = sorted(
        glob.glob(os.path.join(ROOT, "bindings", "*", "README.md")) +
        glob.glob(os.path.join(ROOT, "bindings", "*", "*", "README.md"))
    )

    for path in candidates:
        rel = os.path.relpath(path, ROOT).replace(os.sep, "/")
        if rel in NOT_PACKAGED:
            continue
        checked += 1
        with open(path, encoding="utf-8") as handle:
            text = handle.read()

        for target in LINK.findall(text):
            target = target.split()[0].strip()
            if target.startswith(("http://", "https://", "#", "mailto:")):
                continue
            # Climbing out of the package directory is what publishing breaks.
            if target.startswith("../") or target.startswith("/"):
                failures.append(f"{rel}: relative link out of the package: {target}")

    if failures:
        print("links that resolve on GitHub and nowhere else:", file=sys.stderr)
        for line in failures:
            print(f"  {line}", file=sys.stderr)
        print("\nUse an absolute https://github.com/wickra-lib/wickra-strategy-ci/... "
              "URL in anything that ships as package metadata.", file=sys.stderr)
        return 1

    print(f"{checked} packaged READMEs link absolutely")
    return 0


if __name__ == "__main__":
    sys.exit(main())
