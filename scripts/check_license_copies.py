#!/usr/bin/env python3
"""Every published package must carry the licence texts it claims.

The repository is dual-licensed and every manifest says so, but an SPDX
expression is a reference to two documents, not the documents. A package that
ships the expression alone leaves whoever received it with terms they have to go
and find.

Cargo is the strict case and the reason this runs before a release rather than
at publish time: it decides what to package from **git**, so a copy that is
untracked makes `cargo publish` refuse the dirty tree, and one that is gitignored
is dropped from the tarball without a word. The copies therefore have to be
committed, and this checks that they are -- present, tracked, and byte-identical
to the roots they copy.

npm is the lenient case: it packs from the working tree, so the release workflow
stages the copies moments before publishing and proves with `npm pack --dry-run`
that they landed. What this checks for npm is the other half -- that each
manifest's `files` field actually names them, since a staged file nobody lists
is a staged file nobody ships.

Run from the repository root:  python scripts/check_license_copies.py
"""

from __future__ import annotations

import json
import os
import subprocess
import sys

ROOT = os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)), ".."))
TEXTS = ("LICENSE-MIT", "LICENSE-APACHE")

# Crates that are published to crates.io. A crate with `publish = false` ships
# nowhere and needs no copy.
PUBLISHED_CRATES = ("strategy-ci-core", "strategy-ci-cli")

NPM_PLATFORMS = [
    "darwin-arm64", "darwin-x64", "linux-arm64-gnu",
    "linux-x64-gnu", "win32-arm64-msvc", "win32-x64-msvc",
]


def tracked() -> set[str]:
    out = subprocess.run(["git", "ls-files"], cwd=ROOT, capture_output=True,
                         text=True, check=True).stdout
    return set(out.split("\n"))


def body(path: str) -> str:
    with open(os.path.join(ROOT, path), encoding="utf-8") as handle:
        return handle.read().replace("\r\n", "\n")


def main() -> int:
    failures: list[str] = []
    index = tracked()

    roots = {name: body(name) for name in TEXTS}

    # --- cargo: the copies must be committed, and must match ----------------
    for crate in PUBLISHED_CRATES:
        for name in TEXTS:
            rel = f"crates/{crate}/{name}"
            if rel not in index:
                failures.append(
                    f"{rel}: not tracked -- cargo packages from git, so this "
                    f"would be missing from the .crate")
                continue
            if body(rel) != roots[name]:
                failures.append(f"{rel}: differs from the root {name}")
        print(f"  crate {crate}: {', '.join(TEXTS)}")

    # --- npm: every manifest must list them in `files` ----------------------
    manifests = ["bindings/node/package.json"] + [
        f"bindings/node/npm/{platform}/package.json" for platform in NPM_PLATFORMS
    ]
    for manifest in manifests:
        with open(os.path.join(ROOT, manifest), encoding="utf-8") as handle:
            files = json.load(handle).get("files", [])
        missing = [name for name in TEXTS if name not in files]
        if missing:
            failures.append(
                f"{manifest}: `files` does not name {', '.join(missing)}, "
                f"so the staged copies would not be packed")
        else:
            print(f"  npm   {manifest.split('/')[-2]}: listed in `files`")

    if failures:
        print("\nlicence texts would not travel with the package:", file=sys.stderr)
        for line in failures:
            print(f"  {line}", file=sys.stderr)
        return 1

    print(f"\n{len(PUBLISHED_CRATES)} crates and {len(manifests)} npm packages "
          f"carry both licence texts")
    return 0


if __name__ == "__main__":
    sys.exit(main())
