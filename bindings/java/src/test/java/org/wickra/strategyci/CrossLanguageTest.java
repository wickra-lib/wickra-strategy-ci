package org.wickra.strategyci;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertTrue;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.List;
import java.util.TreeMap;
import java.util.stream.Stream;
import org.junit.jupiter.api.Test;

// Cross-language golden: build the run_suite command from the committed
// golden/{tests,data} corpus, run it through the binding, and assert the response
// equals golden/expected/suite.json byte-for-byte — the exact SuiteResult the
// Rust core and every other binding produce.
class CrossLanguageTest {
    private static Path goldenDir() {
        Path dir = Paths.get("").toAbsolutePath();
        for (int i = 0; i < 10 && dir != null; i++) {
            Path g = dir.resolve("golden");
            if (Files.isDirectory(g.resolve("tests"))) {
                return g;
            }
            dir = dir.getParent();
        }
        throw new IllegalStateException("golden/ not found");
    }

    private static String loadData(Path golden) throws IOException {
        var symbols = new TreeMap<String, String>();
        try (Stream<Path> files = Files.list(golden.resolve("data"))) {
            for (Path csv : files.filter(p -> p.toString().endsWith(".csv")).sorted().toList()) {
                List<String> rows = new ArrayList<>();
                List<String> lines = Files.readAllLines(csv);
                for (int idx = 0; idx < lines.size(); idx++) {
                    String line = lines.get(idx).trim();
                    if (line.isEmpty()) {
                        continue;
                    }
                    String[] c = line.split(",");
                    long t;
                    try {
                        t = Long.parseLong(c[0].trim());
                    } catch (NumberFormatException e) {
                        continue; // header
                    }
                    rows.add("{\"time\":" + t + ",\"open\":" + c[1].trim() + ",\"high\":" + c[2].trim()
                            + ",\"low\":" + c[3].trim() + ",\"close\":" + c[4].trim()
                            + ",\"volume\":" + c[5].trim() + "}");
                }
                String name = csv.getFileName().toString().replaceFirst("\\.csv$", "");
                symbols.put(name, "[" + String.join(",", rows) + "]");
            }
        }
        var parts = new ArrayList<String>();
        symbols.forEach((k, v) -> parts.add("\"" + k + "\":" + v));
        return "{" + String.join(",", parts) + "}";
    }

    @Test
    void runSuiteMatchesGolden() throws IOException {
        Path golden = goldenDir();
        var tests = new ArrayList<String>();
        try (Stream<Path> files = Files.list(golden.resolve("tests"))) {
            for (Path p : files.filter(x -> x.toString().endsWith(".json")).sorted().toList()) {
                tests.add(Files.readString(p).trim());
            }
        }
        String cmd = "{\"cmd\":\"run_suite\",\"tests\":[" + String.join(",", tests)
                + "],\"data\":" + loadData(golden) + "}";
        String want = Files.readString(golden.resolve("expected").resolve("suite.json")).trim();

        try (Session session = new Session()) {
            assertEquals(want, session.command(cmd));
        }
    }

    // The batch path against the per-test path. run_suite fans the corpus out
    // across rayon and sorts the results by id; run_test walks one test at a
    // time. Those are two different engines reached through the same FFM
    // boundary, and only the Rust core tested that they agree -- from a binding,
    // the parallel path crossing the boundary is a separate claim. A regression
    // here would show up as a suite that passes while an individual run of the
    // same test does not.
    //
    // Compared as text rather than through a JSON model: each golden file is
    // named after the id it carries, so the sorted file order is the sorted id
    // order run_suite emits, and the per-test responses concatenated are exactly
    // the suite's results array.
    @Test
    void runSuiteAgreesWithIndividualRuns() throws IOException {
        Path golden = goldenDir();
        String data = loadData(golden);
        var tests = new ArrayList<String>();
        try (Stream<Path> files = Files.list(golden.resolve("tests"))) {
            for (Path p : files.filter(x -> x.toString().endsWith(".json")).sorted().toList()) {
                tests.add(Files.readString(p).trim());
            }
        }

        try (Session session = new Session()) {
            String suite = session.command("{\"cmd\":\"run_suite\",\"tests\":["
                    + String.join(",", tests) + "],\"data\":" + data + "}");

            int from = suite.indexOf("\"results\":[");
            assertTrue(from >= 0, "the suite response carries no results array");
            from += "\"results\":[".length();
            int to = suite.lastIndexOf("],\"passed\":");
            assertTrue(to > from, "the suite response carries no passed count");
            String batch = suite.substring(from, to);

            var individual = new ArrayList<String>();
            for (String test : tests) {
                individual.add(session.command(
                        "{\"cmd\":\"run_test\",\"test\":" + test + ",\"data\":" + data + "}"));
            }

            assertEquals(String.join(",", individual), batch,
                    "the batch path must equal the per-test path");
        }
    }
}
