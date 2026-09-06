//! Session configuration and the crate version.

/// The crate version, surfaced through the `version` command and
/// [`crate::Session::version`].
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Per-session configuration. The runner is stateless with respect to tests and
/// data, so the session holds only configuration; this is the extension point for
/// future runtime knobs (report-hash policy, output formatting, …).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Config {}
