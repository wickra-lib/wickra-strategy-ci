// A runnable C++ example against the wickra-strategy-ci C ABI: run a golden test
// and confirm it passes. Uses the same length-out protocol as the C example, but
// reads the response into a std::string.
#include <cstdint>
#include <iostream>
#include <string>

extern "C" {
#include "wickra_strategy_ci.h"
}

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
    std::cout << "wickra-strategy-ci " << wickra_strategy_ci_version() << "\n";

    WickraStrategyCi *session = wickra_strategy_ci_new();
    if (session == nullptr) {
        std::cerr << "failed to create session\n";
        return 1;
    }

    int32_t len = wickra_strategy_ci_command(session, kCmd, nullptr, 0);
    if (len < 0) {
        std::cerr << "command failed (code " << len << ")\n";
        wickra_strategy_ci_free(session);
        return 1;
    }
    std::string buf(static_cast<size_t>(len) + 1, '\0');
    wickra_strategy_ci_command(session, kCmd, &buf[0], buf.size());

    bool ok = buf.find("\"expected\"") != std::string::npos;
    std::cout << "blessed test: " << (ok ? "PASS (golden pinned)" : "FAIL") << "\n";

    wickra_strategy_ci_free(session);
    return ok ? 0 : 1;
}
