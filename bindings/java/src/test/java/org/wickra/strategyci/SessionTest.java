package org.wickra.strategyci;

import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertTrue;

import org.junit.jupiter.api.Test;

class SessionTest {
    static final String STRATEGY =
            "{\"symbol\":\"TEST\",\"timeframe\":\"1h\","
                    + "\"indicators\":{\"fast\":{\"type\":\"Sma\",\"params\":[2]},"
                    + "\"slow\":{\"type\":\"Sma\",\"params\":[5]}},"
                    + "\"entry\":{\"cross_above\":[\"fast\",\"slow\"]},"
                    + "\"exit\":{\"cross_below\":[\"fast\",\"slow\"]},"
                    + "\"sizing\":{\"type\":\"fixed_fraction\",\"fraction\":1.0}}";

    static String candles() {
        StringBuilder sb = new StringBuilder("[");
        for (int i = 0; i < 40; i++) {
            double px = 100.0 + 10.0 * Math.sin(i * 0.5);
            if (i > 0) {
                sb.append(',');
            }
            sb.append("{\"time\":").append(i)
                    .append(",\"open\":").append(px)
                    .append(",\"high\":").append(px + 1.0)
                    .append(",\"low\":").append(px - 1.0)
                    .append(",\"close\":").append(px)
                    .append(",\"volume\":100.0}");
        }
        return sb.append(']').toString();
    }

    static String testEnvelope(String cmd) {
        return "{\"cmd\":\"" + cmd + "\",\"test\":{"
                + "\"id\":\"momentum\",\"strategy\":" + STRATEGY
                + ",\"dataset_ref\":\"sym-01\",\"property_checks\":[{\"kind\":\"no_nan\"}]},"
                + "\"data\":{\"sym-01\":" + candles() + "}}";
    }

    @Test
    void versionIsNonEmpty() {
        assertFalse(Session.version().isEmpty());
    }

    @Test
    void runTestPasses() {
        try (Session session = new Session()) {
            String result = session.command(testEnvelope("run_test"));
            assertTrue(result.contains("\"id\":\"momentum\""), result);
            assertTrue(result.contains("\"passed\":true"), result);
            assertTrue(result.contains("\"diff\":[]"), result);
        }
    }

    @Test
    void unknownCommandIsInBandError() {
        try (Session session = new Session()) {
            // An unknown command is not a hard error: the ABI returns a length and
            // the error surfaces in-band as {"ok":false,...} JSON.
            String raw = session.command("{\"cmd\":\"nope\"}");
            assertTrue(raw.contains("\"ok\":false"), raw);
        }
    }
}
