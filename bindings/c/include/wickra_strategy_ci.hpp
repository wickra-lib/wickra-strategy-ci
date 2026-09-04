// Optional C++ convenience layer over the wickra-strategy-ci C ABI
// (`wickra_strategy_ci.h`).
//
// The C ABI has two rough edges that every caller otherwise re-implements: a
// handle that must be freed exactly once, and a two-call buffer protocol for
// `command` (ask for the length, allocate, ask again). This header wraps both:
//
//     #include "wickra_strategy_ci.hpp"
//
//     wickra::strategy_ci::Session session;
//     std::string response = session.command(R"({"cmd":"run_suite", ...})");
//     // session is freed here
//
// Header-only, and adds no runtime cost beyond the C calls themselves.
//
// Errors: `command` throws `wickra::strategy_ci::Error` for the negative return
// codes, which signal a misuse of the ABI (null handle, non-UTF-8 command) or a
// caught panic. An error *in the request* -- an unknown command, a malformed
// test -- is not one of those: it comes back as a normal
// `{"ok":false,"error":...}` response string, because it is data about the
// request rather than a failure of the call.

#ifndef WICKRA_STRATEGY_CI_HPP
#define WICKRA_STRATEGY_CI_HPP

#include "wickra_strategy_ci.h"

#include <stdexcept>
#include <string>
#include <utility>

namespace wickra {
namespace strategy_ci {

/// Thrown when the C ABI returns a negative code. `code` is that value: -1 for a
/// null handle or command, -2 for a non-UTF-8 command, -3 for a caught panic.
class Error : public std::runtime_error {
public:
    explicit Error(int32_t code)
        : std::runtime_error("wickra-strategy-ci: command failed with code " +
                             std::to_string(code)),
          code_(code) {}

    /// The raw negative return code.
    int32_t code() const noexcept { return code_; }

private:
    int32_t code_;
};

/// Move-only RAII owner of a `WickraStrategyCi *`.
class Session {
public:
    /// Create a session. Throws `std::bad_alloc` if the allocation fails, which
    /// is the only way `wickra_strategy_ci_new` returns null.
    Session() : handle_(wickra_strategy_ci_new()) {
        if (handle_ == nullptr) {
            throw std::bad_alloc();
        }
    }

    ~Session() {
        if (handle_ != nullptr) {
            wickra_strategy_ci_free(handle_);
        }
    }

    Session(const Session &) = delete;
    Session &operator=(const Session &) = delete;

    Session(Session &&other) noexcept : handle_(std::exchange(other.handle_, nullptr)) {}

    Session &operator=(Session &&other) noexcept {
        if (this != &other) {
            if (handle_ != nullptr) {
                wickra_strategy_ci_free(handle_);
            }
            handle_ = std::exchange(other.handle_, nullptr);
        }
        return *this;
    }

    /// The underlying handle, for calling the C functions directly.
    WickraStrategyCi *get() const noexcept { return handle_; }

    /// Run a command envelope and return the response JSON.
    ///
    /// Performs the ABI's two-call protocol: the first call with a null buffer
    /// asks for the length, the second fills it. The response is deterministic,
    /// so the second call cannot return a different length than the first.
    std::string command(const std::string &cmd) const {
        const int32_t len = wickra_strategy_ci_command(handle_, cmd.c_str(), nullptr, 0);
        if (len < 0) {
            throw Error(len);
        }
        if (len == 0) {
            return std::string();
        }
        std::string out(static_cast<size_t>(len), '\0');
        // The ABI writes `len` bytes plus a NUL, so it is handed `len + 1` of
        // capacity. Writing into the string's buffer is well defined since
        // C++11: its storage is contiguous and NUL-terminated at `[size()]`.
        const int32_t wrote = wickra_strategy_ci_command(
            handle_, cmd.c_str(), &out[0], static_cast<uintptr_t>(len) + 1);
        if (wrote < 0) {
            throw Error(wrote);
        }
        return out;
    }

private:
    WickraStrategyCi *handle_;
};

/// The library version.
inline std::string version() { return std::string(wickra_strategy_ci_version()); }

}  // namespace strategy_ci
}  // namespace wickra

#endif  // WICKRA_STRATEGY_CI_HPP
