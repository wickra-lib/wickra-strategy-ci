"""Smoke test: construct a session, run a strategy test, parse the result."""

import json
import math

from wickra_strategy_ci import Session, __version__

STRATEGY = {
    "symbol": "TEST",
    "timeframe": "1h",
    "indicators": {
        "fast": {"type": "Sma", "params": [2]},
        "slow": {"type": "Sma", "params": [5]},
    },
    "entry": {"cross_above": ["fast", "slow"]},
    "exit": {"cross_below": ["fast", "slow"]},
    "sizing": {"type": "fixed_fraction", "fraction": 1.0},
}


def _candles() -> list[dict]:
    out = []
    for i in range(40):
        close = 100.0 + 10.0 * math.sin(i * 0.5)
        out.append(
            {
                "time": i,
                "open": close,
                "high": close + 1.0,
                "low": close - 1.0,
                "close": close,
                "volume": 100.0,
            }
        )
    return out


def _run_test_cmd() -> str:
    return json.dumps(
        {
            "cmd": "run_test",
            "test": {
                "id": "momentum",
                "strategy": STRATEGY,
                "dataset_ref": "sym-01",
                "property_checks": [{"kind": "no_nan"}],
            },
            "data": {"sym-01": _candles()},
        }
    )


def test_run_test_roundtrip() -> None:
    session = Session()
    result = json.loads(session.command(_run_test_cmd()))
    assert result["id"] == "momentum"
    assert result["passed"] is True
    assert result["diff"] == []


def test_version_matches_module() -> None:
    assert Session.version() == __version__


def test_unknown_command_returns_error_json() -> None:
    session = Session()
    response = json.loads(session.command(json.dumps({"cmd": "nope"})))
    assert response["ok"] is False
