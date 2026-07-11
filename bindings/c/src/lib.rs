//! C ABI for wickra-strategy-ci — the hub every C-capable language links against.
//!
//! One opaque handle plus a `command` entry point that takes a request JSON and
//! writes a response JSON into a caller-owned buffer. The buffer protocol lets a
//! caller size the response with a first (short) call, then read it with a second.

#![allow(unsafe_code)]

use std::ffi::{c_char, CStr};
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::ptr;

use strategy_ci_core::Session;

/// Opaque handle wrapping a [`Session`].
pub struct WickraStrategyCi(Session);

/// Create a handle. Returns null only on allocation failure.
#[no_mangle]
pub extern "C" fn wickra_strategy_ci_new() -> *mut WickraStrategyCi {
    catch_unwind(|| Box::into_raw(Box::new(WickraStrategyCi(Session::new()))))
        .unwrap_or(ptr::null_mut())
}

/// Free a handle. Null-safe.
///
/// # Safety
/// `handle` must be a pointer returned by [`wickra_strategy_ci_new`] and not
/// already freed, or null.
#[no_mangle]
pub unsafe extern "C" fn wickra_strategy_ci_free(handle: *mut WickraStrategyCi) {
    if !handle.is_null() {
        drop(Box::from_raw(handle));
    }
}

/// Run a command envelope and write the response JSON into `out`.
///
/// Returns the response length in bytes (excluding the NUL). If that length is
/// `< cap`, the response plus a NUL terminator is written to `out`; otherwise
/// `out` is left untouched and the caller should retry with a `len + 1` buffer
/// (the response is deterministic, so the second call is the same length).
/// Returns `-1` for a null handle or command, `-2` for a non-UTF-8 command, and
/// `-3` if a panic was caught (which never happens in normal operation).
/// Internal errors are returned as an `{"ok":false,"error":...}` response with a
/// non-negative length, not as a negative code.
///
/// # Safety
/// `handle` must come from [`wickra_strategy_ci_new`]; `cmd` must be a valid
/// NUL-terminated string; and `out` must point to at least `cap` writable bytes
/// (or be null when `cap` is 0 for a length query).
#[no_mangle]
pub unsafe extern "C" fn wickra_strategy_ci_command(
    handle: *mut WickraStrategyCi,
    cmd: *const c_char,
    out: *mut c_char,
    cap: usize,
) -> i32 {
    if handle.is_null() || cmd.is_null() {
        return -1;
    }
    let Ok(cmd) = CStr::from_ptr(cmd).to_str() else {
        return -2;
    };
    let response = catch_unwind(AssertUnwindSafe(|| {
        let session = &mut (*handle).0;
        session.command_json(cmd).unwrap_or_else(|e| {
            format!(
                "{{\"ok\":false,\"error\":{}}}",
                serde_json::to_string(&e.to_string()).unwrap_or_else(|_| "\"error\"".to_string())
            )
        })
    }));
    let Ok(response) = response else {
        return -3;
    };
    let bytes = response.as_bytes();
    let len = bytes.len();
    if len < cap && !out.is_null() {
        ptr::copy_nonoverlapping(bytes.as_ptr(), out.cast::<u8>(), len);
        *out.add(len) = 0;
    }
    i32::try_from(len).unwrap_or(i32::MAX)
}

/// The crate version as a static NUL-terminated string.
#[no_mangle]
pub extern "C" fn wickra_strategy_ci_version() -> *const c_char {
    concat!(env!("CARGO_PKG_VERSION"), "\0").as_ptr().cast()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    fn read_buf(buf: &[u8]) -> String {
        CStr::from_bytes_until_nul(buf)
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
    }

    /// Run one command, allocating exactly the buffer the length query asks for.
    unsafe fn command(handle: *mut WickraStrategyCi, cmd: &str) -> String {
        let cmd = CString::new(cmd).unwrap();
        let len = wickra_strategy_ci_command(handle, cmd.as_ptr(), ptr::null_mut(), 0);
        assert!(len > 0);
        let mut buf = vec![0u8; usize::try_from(len).unwrap() + 1];
        let len2 = wickra_strategy_ci_command(
            handle,
            cmd.as_ptr(),
            buf.as_mut_ptr().cast::<c_char>(),
            buf.len(),
        );
        assert_eq!(len2, len);
        read_buf(&buf)
    }

    #[test]
    fn new_command_free_round_trip() {
        let handle = wickra_strategy_ci_new();
        assert!(!handle.is_null());
        unsafe {
            assert!(command(handle, r#"{"cmd":"version"}"#).contains("\"version\""));
            wickra_strategy_ci_free(handle);
        }
    }

    #[test]
    fn too_small_buffer_leaves_out_untouched() {
        let handle = wickra_strategy_ci_new();
        let cmd = CString::new(r#"{"cmd":"version"}"#).unwrap();
        let mut buf = vec![0xAAu8; 4]; // deliberately too small
        let len = unsafe {
            wickra_strategy_ci_command(
                handle,
                cmd.as_ptr(),
                buf.as_mut_ptr().cast::<c_char>(),
                buf.len(),
            )
        };
        assert!(usize::try_from(len).unwrap() >= buf.len());
        assert!(buf.iter().all(|&b| b == 0xAA)); // untouched
        unsafe { wickra_strategy_ci_free(handle) };
    }

    #[test]
    fn null_args_return_negative_one() {
        let cmd = CString::new(r#"{"cmd":"version"}"#).unwrap();
        let handle = wickra_strategy_ci_new();
        unsafe {
            assert_eq!(
                wickra_strategy_ci_command(ptr::null_mut(), cmd.as_ptr(), ptr::null_mut(), 0),
                -1
            );
            assert_eq!(
                wickra_strategy_ci_command(handle, ptr::null(), ptr::null_mut(), 0),
                -1
            );
            wickra_strategy_ci_free(handle);
        }
    }

    #[test]
    fn unknown_command_returns_error_json() {
        let handle = wickra_strategy_ci_new();
        unsafe {
            assert!(command(handle, r#"{"cmd":"nope"}"#).contains("\"ok\":false"));
            wickra_strategy_ci_free(handle);
        }
    }

    #[test]
    fn version_string_is_valid() {
        let version = unsafe { CStr::from_ptr(wickra_strategy_ci_version()) }
            .to_str()
            .unwrap();
        assert_eq!(version, strategy_ci_core::VERSION);
    }

    #[test]
    fn free_null_is_safe() {
        unsafe { wickra_strategy_ci_free(ptr::null_mut()) };
    }
}
