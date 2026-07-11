"""Pin the public surface of the Session class across bindings."""

from wickra_strategy_ci import Session

EXPECTED_METHODS = {"command", "version"}


def test_expected_methods_present() -> None:
    for name in EXPECTED_METHODS:
        assert hasattr(Session, name), f"missing method: {name}"


def test_no_unexpected_public_methods() -> None:
    public = {name for name in dir(Session) if not name.startswith("_")}
    assert public == EXPECTED_METHODS
