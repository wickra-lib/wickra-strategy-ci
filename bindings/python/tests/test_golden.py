"""Determinism: the same command yields byte-identical output across calls, and a
blessed golden re-matches itself. This is the cross-language golden invariant seen
from Python — the response bytes are what every other binding produces too."""

import json

from wickra_strategy_ci import Session

from test_smoke import _run_test_cmd, _candles, STRATEGY


def test_run_test_is_byte_identical_across_calls() -> None:
    a = Session().command(_run_test_cmd())
    b = Session().command(_run_test_cmd())
    assert a == b


def test_bless_then_match() -> None:
    session = Session()
    bless_cmd = json.dumps(
        {
            "cmd": "bless",
            "test": {
                "id": "momentum",
                "strategy": STRATEGY,
                "dataset_ref": "sym-01",
                "property_checks": [{"kind": "no_nan"}],
            },
            "data": {"sym-01": _candles()},
        }
    )
    blessed = json.loads(session.command(bless_cmd))
    assert "expected" in blessed

    rerun = json.loads(
        session.command(
            json.dumps(
                {"cmd": "run_test", "test": blessed, "data": {"sym-01": _candles()}}
            )
        )
    )
    assert rerun["diff"] == []
    assert rerun["passed"] is True
