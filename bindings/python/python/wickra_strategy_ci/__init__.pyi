__version__: str

class Session:
    """A stateless strategy-test session driven over a JSON command boundary."""

    def __init__(self) -> None: ...
    def command(self, cmd_json: str) -> str:
        """Run a command envelope (`{"cmd": ...}`) and return the response JSON."""
        ...

    @staticmethod
    def version() -> str:
        """Return the crate version."""
        ...

__all__ = ["Session", "__version__"]
