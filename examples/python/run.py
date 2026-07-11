"""A runnable Python example: golden-pin a strategy with `bless`, then re-run it
and confirm it passes — the report is recomputed by the engine, so the golden is
an exact, reproducible anchor.

    pip install wickra-strategy-ci
    python examples/python/run.py
"""

import json

from wickra_strategy_ci import Session

SYMBOL = "AAA"

TEST = {
    "id": "ema_crossover",
    "strategy": {
        "symbol": SYMBOL,
        "timeframe": "1h",
        "indicators": {
            "fast": {"type": "Ema", "params": [3]},
            "slow": {"type": "Ema", "params": [8]},
        },
        "entry": {"cross_above": ["fast", "slow"]},
        "exit": {"cross_below": ["fast", "slow"]},
        "sizing": {"type": "fixed_fraction", "fraction": 0.95},
    },
    "dataset_ref": SYMBOL,
    "tolerances": {"*": {"kind": "rel", "value": 0.0001}},
    "property_checks": [{"kind": "no_nan"}],
}

# A short V-shaped price path so the fast/slow EMA cross fires at least once.
CLOSES = [120, 118, 116, 114, 112, 110, 108, 112, 116, 120, 124, 128]


def _candles() -> list[dict]:
    out = []
    for i, close in enumerate(CLOSES):
        open_ = close if i == 0 else CLOSES[i - 1]
        out.append(
            {
                "time": 1_700_000_000 + i * 3600,
                "open": open_,
                "high": max(open_, close) + 1,
                "low": min(open_, close) - 1,
                "close": close,
                "volume": 1000,
            }
        )
    return out


def main() -> None:
    session = Session()
    data = {SYMBOL: _candles()}
    print("wickra-strategy-ci", Session.version())

    blessed = json.loads(session.command(json.dumps({"cmd": "bless", "test": TEST, "data": data})))
    result = json.loads(
        session.command(json.dumps({"cmd": "run_test", "test": blessed, "data": data}))
    )
    assert result["passed"], result
    print(f"blessed test: PASS (diff empty, {len(result['property_results'])} checks)")


if __name__ == "__main__":
    main()
