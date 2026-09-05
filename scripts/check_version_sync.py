#!/usr/bin/env python3
"""Assert that every place carrying the release version agrees.

The version lives in a dozen files across six package managers, and a bump that
misses one produces a release where, say, the npm package pins a native binary
that was never published. That failure surfaces at install time, on a user's
machine, after the tag is already irreversible -- so it is worth a cheap check
before the tag rather than a patch release after it.

    python scripts/check_version_sync.py                   # all files agree
    python scripts/check_version_sync.py --previous 0.1.0  # and none is stale

The file list is explicit rather than a repository-wide grep on purpose: a grep
matches an unrelated `0.1.0` in a lockfile's third-party entry or in a "since
0.1.0" note in prose, and a check with false positives gets ignored. Adding a
version touchpoint means adding it here; that is the point.

Run from the repository root.
"""

from __future__ import annotations

import json
import os
import re
import sys

ROOT = os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)), ".."))

NPM_PLATFORMS = [
    "darwin-arm64", "darwin-x64", "linux-arm64-gnu",
    "linux-x64-gnu", "win32-arm64-msvc", "win32-x64-msvc",
]


def read(path: str) -> str:
    with open(os.path.join(ROOT, path), encoding="utf-8") as handle:
        return handle.read()


def toml_version(path: str, section: str | None = None) -> str | None:
    """First `version = "..."` in `path`, optionally after `section`."""
    text = read(path)
    if section:
        idx = text.find(section)
        if idx < 0:
            return None
        text = text[idx:]
    match = re.search(r'^\s*version\s*=\s*"([^"]+)"', text, re.M)
    return match.group(1) if match else None


def json_version(path: str) -> str | None:
    with open(os.path.join(ROOT, path), encoding="utf-8") as handle:
        return json.load(handle).get("version")


def xml_version(path: str) -> str | None:
    """The project's own <version>, which is the first one before <properties>."""
    text = read(path)
    head = text.split("<properties>", 1)[0]
    match = re.search(r"<version>([^<]+)</version>", head)
    return match.group(1) if match else None


def csproj_version(path: str) -> str | None:
    match = re.search(r"<Version>([^<]+)</Version>", read(path))
    return match.group(1) if match else None


def description_version(path: str) -> str | None:
    match = re.search(r"^Version:\s*(\S+)", read(path), re.M)
    return match.group(1) if match else None


def sources() -> list[tuple[str, str | None]]:
    found = [
        ("Cargo.toml [workspace.package]", toml_version("Cargo.toml", "[workspace.package]")),
        ("bindings/python/pyproject.toml", toml_version("bindings/python/pyproject.toml")),
        ("bindings/node/package.json", json_version("bindings/node/package.json")),
        ("bindings/java/pom.xml", xml_version("bindings/java/pom.xml")),
        ("bindings/csharp/.../WickraStrategyCi.csproj",
         csproj_version("bindings/csharp/WickraStrategyCi/WickraStrategyCi.csproj")),
        ("bindings/r/DESCRIPTION", description_version("bindings/r/DESCRIPTION")),
    ]
    for platform in NPM_PLATFORMS:
        path = f"bindings/node/npm/{platform}/package.json"
        found.append((path, json_version(path)))
    return found


def optional_dependencies() -> list[tuple[str, str]]:
    """The main npm package pins each platform package by exact version."""
    with open(os.path.join(ROOT, "bindings/node/package.json"), encoding="utf-8") as handle:
        deps = json.load(handle).get("optionalDependencies", {})
    return sorted(deps.items())


def main() -> int:
    previous = None
    if "--previous" in sys.argv:
        previous = sys.argv[sys.argv.index("--previous") + 1]

    found = sources()
    unreadable = [name for name, value in found if value is None]
    if unreadable:
        print("could not read a version from:", file=sys.stderr)
        for name in unreadable:
            print(f"  {name}", file=sys.stderr)
        return 1

    versions = {value for _, value in found}
    canonical = toml_version("Cargo.toml", "[workspace.package]")

    for name, value in found:
        mark = "ok  " if value == canonical else "DIFF"
        print(f"  {mark} {value:10} {name}")

    if len(versions) != 1:
        print(f"\nversion drift: {sorted(versions)}", file=sys.stderr)
        return 1

    # The optional dependencies pin the platform packages exactly; a bump that
    # misses them installs a native binary from the previous release.
    bad = [(name, spec) for name, spec in optional_dependencies() if spec != canonical]
    if bad:
        print("\nnpm optionalDependencies do not pin this version:", file=sys.stderr)
        for name, spec in bad:
            print(f"  {name} = {spec} (expected {canonical})", file=sys.stderr)
        return 1
    print(f"  ok   {canonical:10} bindings/node optionalDependencies "
          f"({len(optional_dependencies())} pins)")

    if previous and canonical == previous:
        print(f"\nversion is still {previous}: the bump did not happen", file=sys.stderr)
        return 1

    print(f"\nall {len(found)} touchpoints agree on {canonical}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
