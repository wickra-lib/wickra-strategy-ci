// A runnable C++ example against the wickra-strategy-ci C ABI: bless a test and
// confirm the golden is pinned.
//
// Where run.c does the two-call buffer protocol and the free by hand, this uses
// the header-only C++ hull (wickra_strategy_ci.hpp): the session frees itself at
// scope exit and `command` returns a std::string. Compiling this example is what
// keeps the hull honest -- it is the only thing that builds it.
#include <iostream>
#include <string>

#include "wickra_strategy_ci.hpp"

namespace {
const char *kCmd =
    "{\"cmd\":\"bless\",\"test\":{\"id\":\"ema_crossover\",\"strategy\":{"
    "\"symbol\":\"AAA\",\"timeframe\":\"1h\","
    "\"indicators\":{\"fast\":{\"type\":\"Ema\",\"params\":[3]},"
    "\"slow\":{\"type\":\"Ema\",\"params\":[8]}},"
    "\"entry\":{\"cross_above\":[\"fast\",\"slow\"]},"
    "\"exit\":{\"cross_below\":[\"fast\",\"slow\"]},"
    "\"sizing\":{\"type\":\"fixed_fraction\",\"fraction\":0.95}},"
    "\"dataset_ref\":\"AAA\",\"property_checks\":[{\"kind\":\"no_nan\"}]},"
    "\"data\":{\"AAA\":["
    "{\"time\":1700000000,\"open\":120,\"high\":121,\"low\":119,\"close\":120,\"volume\":1000},"
    "{\"time\":1700003600,\"open\":120,\"high\":121,\"low\":117,\"close\":118,\"volume\":1000},"
    "{\"time\":1700007200,\"open\":118,\"high\":119,\"low\":115,\"close\":116,\"volume\":1000},"
    "{\"time\":1700010800,\"open\":116,\"high\":117,\"low\":113,\"close\":114,\"volume\":1000},"
    "{\"time\":1700014400,\"open\":114,\"high\":115,\"low\":111,\"close\":112,\"volume\":1000},"
    "{\"time\":1700018000,\"open\":112,\"high\":113,\"low\":109,\"close\":110,\"volume\":1000},"
    "{\"time\":1700021600,\"open\":110,\"high\":111,\"low\":107,\"close\":108,\"volume\":1000},"
    "{\"time\":1700025200,\"open\":108,\"high\":113,\"low\":107,\"close\":112,\"volume\":1000},"
    "{\"time\":1700028800,\"open\":112,\"high\":117,\"low\":111,\"close\":116,\"volume\":1000},"
    "{\"time\":1700032400,\"open\":116,\"high\":121,\"low\":115,\"close\":120,\"volume\":1000},"
    "{\"time\":1700036000,\"open\":120,\"high\":125,\"low\":119,\"close\":124,\"volume\":1000},"
    "{\"time\":1700039600,\"open\":124,\"high\":129,\"low\":123,\"close\":128,\"volume\":1000}"
    "]}}";
}  // namespace

int main() {
    std::cout << "wickra-strategy-ci " << wickra::strategy_ci::version() << "\n";

    try {
        wickra::strategy_ci::Session session;
        const std::string response = session.command(kCmd);

        const bool ok = response.find("\"expected\"") != std::string::npos;
        std::cout << "blessed test: " << (ok ? "PASS (golden pinned)" : "FAIL") << "\n";
        return ok ? 0 : 1;
    } catch (const std::exception &e) {
        std::cerr << e.what() << "\n";
        return 1;
    }
}
