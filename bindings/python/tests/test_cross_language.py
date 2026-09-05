"""Cross-language golden: build the run_suite command from the committed
golden/{tests,data} corpus, run it through the binding, and assert the response
equals golden/expected/suite.json byte-for-byte — the exact SuiteResult the Rust
core and every other binding produce."""

import json
import math
import pathlib

from wickra_strategy_ci import Session

GOLDEN = pathlib.Path(__file__).resolve().parents[3] / "golden"


def _load_data() -> dict:
    data = {}
    for csv in sorted((GOLDEN / "data").glob("*.csv")):
        rows = []
        for idx, line in enumerate(csv.read_text().splitlines()):
            line = line.strip()
            if not line:
                continue
            cols = [c.strip() for c in line.split(",")]
            try:
                t = int(cols[0])
            except ValueError:
                if idx == 0:
                    continue  # header
                raise
            rows.append(
                {
                    "time": t,
                    "open": float(cols[1]),
                    "high": float(cols[2]),
                    "low": float(cols[3]),
                    "close": float(cols[4]),
                    "volume": float(cols[5]),
                }
            )
        data[csv.stem] = rows
    return data


def test_run_suite_matches_golden() -> None:
    tests = [
        json.loads(p.read_text()) for p in sorted((GOLDEN / "tests").glob("*.json"))
    ]
    cmd = json.dumps({"cmd": "run_suite", "tests": tests, "data": _load_data()})
    got = Session().command(cmd)
    want = (GOLDEN / "expected" / "suite.json").read_text().strip()
    assert got == want, "SuiteResult must be byte-identical to the Rust golden"
    # Sanity: the corpus is all-green.
    assert json.loads(got)["failed"] == 0
    assert not math.isnan(json.loads(got)["passed"])


def test_run_suite_agrees_with_individual_runs() -> None:
    """The batch path against the per-test path.

    ``run_suite`` fans the corpus out across rayon and sorts the results by id;
    ``run_test`` walks one test at a time. Those are two different engines
    reached through the same boundary, and only the Rust core tested that they
    agree -- from a binding, the parallel path crossing the boundary is a
    separate claim. A regression here would show up as a suite that passes while
    an individual run of the same test does not.
    """
    data = _load_data()
    tests = [
        json.loads(p.read_text()) for p in sorted((GOLDEN / "tests").glob("*.json"))
    ]

    session = Session()
    batch = json.loads(
        session.command(json.dumps({"cmd": "run_suite", "tests": tests, "data": data}))
    )["results"]

    individual = sorted(
        (
            json.loads(
                session.command(
                    json.dumps({"cmd": "run_test", "test": t, "data": data})
                )
            )
            for t in tests
        ),
        key=lambda r: r["id"],
    )

    assert batch == individual, "the batch path must equal the per-test path"
