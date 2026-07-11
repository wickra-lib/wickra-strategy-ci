// A runnable Java example: golden-pin a strategy with `bless`, then re-run it and
// confirm it passes. Compile against the binding classes and run with
// --enable-native-access; the report is recomputed by the engine, so the golden
// is an exact, reproducible anchor.
//
//   cargo build -p wickra-strategy-ci-c
//   javac -cp bindings/java/target/classes examples/java/Run.java -d examples/java/out
//   java --enable-native-access=ALL-UNNAMED \
//        -Dnative.lib.dir=target/debug \
//        -cp "bindings/java/target/classes:examples/java/out" Run

import org.wickra.strategyci.Session;

public class Run {
    private static final String SYMBOL = "AAA";

    private static final String STRATEGY =
            "{\"symbol\":\"AAA\",\"timeframe\":\"1h\","
                    + "\"indicators\":{\"fast\":{\"type\":\"Ema\",\"params\":[3]},"
                    + "\"slow\":{\"type\":\"Ema\",\"params\":[8]}},"
                    + "\"entry\":{\"cross_above\":[\"fast\",\"slow\"]},"
                    + "\"exit\":{\"cross_below\":[\"fast\",\"slow\"]},"
                    + "\"sizing\":{\"type\":\"fixed_fraction\",\"fraction\":0.95}}";

    private static final int[] CLOSES = {120, 118, 116, 114, 112, 110, 108, 112, 116, 120, 124, 128};

    private static String candles() {
        StringBuilder sb = new StringBuilder("[");
        for (int i = 0; i < CLOSES.length; i++) {
            int open = i == 0 ? CLOSES[i] : CLOSES[i - 1];
            int close = CLOSES[i];
            if (i > 0) {
                sb.append(',');
            }
            sb.append("{\"time\":").append(1_700_000_000L + i * 3600L)
                    .append(",\"open\":").append(open)
                    .append(",\"high\":").append(Math.max(open, close) + 1)
                    .append(",\"low\":").append(Math.min(open, close) - 1)
                    .append(",\"close\":").append(close)
                    .append(",\"volume\":1000}");
        }
        return sb.append(']').toString();
    }

    public static void main(String[] args) {
        String data = "{\"" + SYMBOL + "\":" + candles() + "}";
        String test = "{\"id\":\"ema_crossover\",\"strategy\":" + STRATEGY
                + ",\"dataset_ref\":\"" + SYMBOL + "\",\"property_checks\":[{\"kind\":\"no_nan\"}]}";

        try (Session session = new Session()) {
            System.out.println("wickra-strategy-ci " + Session.version());
            String blessed = session.command("{\"cmd\":\"bless\",\"test\":" + test + ",\"data\":" + data + "}");
            String rerun = session.command(
                    "{\"cmd\":\"run_test\",\"test\":" + blessed + ",\"data\":" + data + "}");
            if (!rerun.contains("\"passed\":true")) {
                System.err.println("a freshly-blessed test must pass: " + rerun);
                System.exit(1);
            }
            System.out.println("blessed test: PASS (golden pinned)");
        }
    }
}
