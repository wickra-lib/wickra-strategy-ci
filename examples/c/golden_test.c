/* The cross-language golden invariant, seen from C.
 *
 * Every other binding runs the golden corpus through its own client. C did not,
 * which left the hub itself -- the library the C++, C#, Go, Java and R bindings
 * all call through -- exercised only from Rust. This compiles against the
 * shipped header with a real C compiler and drives the library exactly as a
 * consumer would.
 *
 * It reads the JSON tests and CSV datasets from the corpus, builds the run_suite
 * command from them, and asserts the response equals golden/expected/suite.json
 * byte for byte: the same bytes the Rust core and every other binding produce.
 *
 * Built and run by examples/c/CMakeLists.txt (ctest target `golden`). The corpus
 * path comes from the WKSTRATEGYCI_GOLDEN_DIR compile definition, so the test
 * does not guess where the repository root is. */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "wickra_strategy_ci.h"

#ifndef WKSTRATEGYCI_GOLDEN_DIR
#error "WKSTRATEGYCI_GOLDEN_DIR must be defined by the build"
#endif

/* Grow-on-append string, so neither the command nor the response needs a
 * guessed upper bound. */
typedef struct {
    char *data;
    size_t len;
    size_t cap;
} Buf;

static void buf_init(Buf *b) {
    b->cap = 4096;
    b->len = 0;
    b->data = malloc(b->cap);
    if (b->data == NULL) {
        fprintf(stderr, "out of memory\n");
        exit(1);
    }
    b->data[0] = '\0';
}

static void buf_append(Buf *b, const char *text, size_t n) {
    if (b->len + n + 1 > b->cap) {
        while (b->len + n + 1 > b->cap) {
            b->cap *= 2;
        }
        b->data = realloc(b->data, b->cap);
        if (b->data == NULL) {
            fprintf(stderr, "out of memory\n");
            exit(1);
        }
    }
    memcpy(b->data + b->len, text, n);
    b->len += n;
    b->data[b->len] = '\0';
}

static void buf_str(Buf *b, const char *text) { buf_append(b, text, strlen(text)); }

/* Read a whole file, or exit. A missing corpus file is a broken test, not a
 * condition worth recovering from. */
static char *slurp(const char *path) {
    FILE *fh = fopen(path, "rb");
    if (fh == NULL) {
        fprintf(stderr, "cannot open %s\n", path);
        exit(1);
    }
    fseek(fh, 0, SEEK_END);
    long size = ftell(fh);
    fseek(fh, 0, SEEK_SET);
    char *text = malloc((size_t)size + 1);
    if (text == NULL || fread(text, 1, (size_t)size, fh) != (size_t)size) {
        fprintf(stderr, "cannot read %s\n", path);
        exit(1);
    }
    text[size] = '\0';
    fclose(fh);
    return text;
}

/* Trailing whitespace differs between checkouts; the JSON body does not. */
static void rstrip(char *text) {
    size_t n = strlen(text);
    while (n > 0 && (text[n - 1] == '\n' || text[n - 1] == '\r' ||
                     text[n - 1] == ' ' || text[n - 1] == '\t')) {
        text[--n] = '\0';
    }
}

/* The corpus is fixed, so the file lists are too: naming them keeps the test
 * free of directory-walking code that differs per platform. */
static const char *TESTS[] = {"crossover", "fuzz", "mean_reversion",
                              "momentum", "property_only"};
static const char *DATASETS[] = {"sym-01", "sym-02", "sym-03",
                                 "sym-04", "sym-05", "sym-06"};

static void append_csv_as_candles(Buf *cmd, const char *name) {
    char path[1024];
    snprintf(path, sizeof(path), "%s/data/%s.csv", WKSTRATEGYCI_GOLDEN_DIR, name);
    char *text = slurp(path);

    buf_str(cmd, "\"");
    buf_str(cmd, name);
    buf_str(cmd, "\":[");

    /* Walked by hand rather than with strtok: the line and field splits are
     * nested, and the re-entrant variant is spelled strtok_r on POSIX and
     * strtok_s on MSVC. */
    int first = 1;
    char *line = text;
    while (*line != '\0') {
        char *end = line;
        while (*end != '\0' && *end != '\n' && *end != '\r') {
            ++end;
        }
        char saved = *end;
        *end = '\0';

        /* A data row starts with a digit; the header and blank lines do not. */
        if (line[0] >= '0' && line[0] <= '9') {
            char *field[6];
            int n = 0;
            char *cursor = line;
            while (n < 6) {
                field[n++] = cursor;
                char *comma = strchr(cursor, ',');
                if (comma == NULL) {
                    break;
                }
                *comma = '\0';
                cursor = comma + 1;
            }
            if (n == 6) {
                if (!first) {
                    buf_str(cmd, ",");
                }
                first = 0;
                char row[512];
                snprintf(row, sizeof(row),
                         "{\"time\":%s,\"open\":%s,\"high\":%s,"
                         "\"low\":%s,\"close\":%s,\"volume\":%s}",
                         field[0], field[1], field[2], field[3], field[4], field[5]);
                buf_str(cmd, row);
            }
        }

        if (saved == '\0') {
            break;
        }
        line = end + 1;
        while (*line == '\n' || *line == '\r') {
            ++line;
        }
    }
    buf_str(cmd, "]");
    free(text);
}

/* Run one command through a session, returning a malloc'd response. The ABI's
 * two-call protocol lives here so the checks below read as what they assert. */
static char *run(WickraStrategyCi *session, const char *cmd) {
    int32_t len = wickra_strategy_ci_command(session, cmd, NULL, 0);
    if (len < 0) {
        fprintf(stderr, "command failed with code %d\n", len);
        exit(1);
    }
    char *out = malloc((size_t)len + 1);
    if (out == NULL) {
        fprintf(stderr, "out of memory\n");
        exit(1);
    }
    wickra_strategy_ci_command(session, cmd, out, (uintptr_t)len + 1);
    return out;
}

/* The batch path against the per-test path. run_suite fans the corpus out across
 * rayon and sorts the results by id; run_test walks one test at a time. Those
 * are two different engines reached through the same ABI, and only the Rust core
 * tested that they agree -- through the C boundary the parallel path is a
 * separate claim.
 *
 * Compared as text: each golden file is named after the id it carries, so the
 * sorted file order is the sorted id order run_suite emits, and the per-test
 * responses concatenated are exactly the suite's results array. */
static int check_batch_equals_per_test(WickraStrategyCi *session, const char *suite,
                                       const char *data) {
    const char *marker = "\"results\":[";
    const char *from = strstr(suite, marker);
    const char *to = strstr(suite, "],\"passed\":");
    if (from == NULL || to == NULL || to <= from) {
        fprintf(stderr, "the suite response has no results array\n");
        return 0;
    }
    from += strlen(marker);

    Buf joined;
    buf_init(&joined);
    for (size_t i = 0; i < sizeof(TESTS) / sizeof(TESTS[0]); ++i) {
        char path[1024];
        snprintf(path, sizeof(path), "%s/tests/%s.json", WKSTRATEGYCI_GOLDEN_DIR, TESTS[i]);
        char *test = slurp(path);
        rstrip(test);

        Buf one;
        buf_init(&one);
        buf_str(&one, "{\"cmd\":\"run_test\",\"test\":");
        buf_str(&one, test);
        buf_str(&one, ",\"data\":");
        buf_str(&one, data);
        buf_str(&one, "}");
        free(test);

        char *response = run(session, one.data);
        free(one.data);
        if (i > 0) {
            buf_str(&joined, ",");
        }
        buf_str(&joined, response);
        free(response);
    }

    size_t batch_len = (size_t)(to - from);
    int same = joined.len == batch_len && memcmp(joined.data, from, batch_len) == 0;
    if (!same) {
        fprintf(stderr, "the batch path does not equal the per-test path\n");
        fprintf(stderr, " batch: %.*s\n", (int)batch_len, from);
        fprintf(stderr, "  each: %s\n", joined.data);
    } else {
        printf("C parity: the batch path equals the per-test path\n");
    }
    free(joined.data);
    return same;
}

int main(void) {
    Buf cmd;
    buf_init(&cmd);
    buf_str(&cmd, "{\"cmd\":\"run_suite\",\"tests\":[");
    for (size_t i = 0; i < sizeof(TESTS) / sizeof(TESTS[0]); ++i) {
        char path[1024];
        snprintf(path, sizeof(path), "%s/tests/%s.json", WKSTRATEGYCI_GOLDEN_DIR, TESTS[i]);
        char *text = slurp(path);
        rstrip(text);
        if (i > 0) {
            buf_str(&cmd, ",");
        }
        buf_str(&cmd, text);
        free(text);
    }
    Buf data;
    buf_init(&data);
    buf_str(&data, "{");
    for (size_t i = 0; i < sizeof(DATASETS) / sizeof(DATASETS[0]); ++i) {
        if (i > 0) {
            buf_str(&data, ",");
        }
        append_csv_as_candles(&data, DATASETS[i]);
    }
    buf_str(&data, "}");

    buf_str(&cmd, "],\"data\":");
    buf_str(&cmd, data.data);
    buf_str(&cmd, "}");

    WickraStrategyCi *session = wickra_strategy_ci_new();
    if (session == NULL) {
        fprintf(stderr, "failed to create a session\n");
        return 1;
    }

    char *response = run(session, cmd.data);
    int parity = check_batch_equals_per_test(session, response, data.data);
    wickra_strategy_ci_free(session);
    free(cmd.data);
    free(data.data);

    char expected_path[1024];
    snprintf(expected_path, sizeof(expected_path), "%s/expected/suite.json",
             WKSTRATEGYCI_GOLDEN_DIR);
    char *expected = slurp(expected_path);
    rstrip(expected);
    rstrip(response);

    int same = strcmp(response, expected) == 0 && parity;
    if (!same) {
        fprintf(stderr, "the C ABI does not reproduce the golden suite\n");
        fprintf(stderr, "expected: %s\n", expected);
        fprintf(stderr, "actual:   %s\n", response);
    } else {
        printf("C golden: the suite matches golden/expected/suite.json byte for byte\n");
    }

    free(response);
    free(expected);
    return same ? 0 : 1;
}
