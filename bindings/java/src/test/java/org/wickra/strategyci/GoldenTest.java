package org.wickra.strategyci;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertTrue;

import org.junit.jupiter.api.Test;

// The cross-language golden invariant seen from Java: the same command yields
// byte-identical output across calls, and a blessed golden re-matches itself. The
// response bytes are what every other binding produces too, because the whole
// test runner lives once in the Rust core and this binding forwards its JSON
// verbatim.
class GoldenTest {
    @Test
    void runTestIsByteIdenticalAcrossCalls() {
        try (Session a = new Session(); Session b = new Session()) {
            String cmd = SessionTest.testEnvelope("run_test");
            assertEquals(a.command(cmd), b.command(cmd));
        }
    }

    @Test
    void blessThenReRunMatchesWithEmptyDiff() {
        try (Session session = new Session()) {
            String blessed = session.command(SessionTest.testEnvelope("bless"));
            assertTrue(blessed.contains("\"expected\""), blessed);

            String rerunCmd = "{\"cmd\":\"run_test\",\"test\":" + blessed
                    + ",\"data\":{\"sym-01\":" + SessionTest.candles() + "}}";
            String rerun = session.command(rerunCmd);
            assertTrue(rerun.contains("\"passed\":true"), rerun);
            assertTrue(rerun.contains("\"diff\":[]"), rerun);
        }
    }
}
