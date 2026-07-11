"""Wickra Strategy-CI — a test runner for trading strategies.

A ``Session`` drives the deterministic core over a JSON boundary: ``command``
takes a request envelope (``{"cmd": ...}``) and returns the response JSON, so the
result is byte-identical to every other Wickra Strategy-CI binding.
"""

from ._wickra_strategy_ci import Session, __version__

__all__ = ["Session", "__version__"]
