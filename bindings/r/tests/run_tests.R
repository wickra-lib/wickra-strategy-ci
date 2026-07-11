## Plain-R tests for the wickra-strategy-ci R binding (no testthat dependency).
## Mirrors the Rust/Python/Node/Go/C#/Java tests and doubles as the completeness
## guard: it exercises the full public surface (version + new + command).

library(wickrastrategyci)

strategy <- paste0(
  '{"symbol":"TEST","timeframe":"1h",',
  '"indicators":{"fast":{"type":"Sma","params":[2]},',
  '"slow":{"type":"Sma","params":[5]}},',
  '"entry":{"cross_above":["fast","slow"]},',
  '"exit":{"cross_below":["fast","slow"]},',
  '"sizing":{"type":"fixed_fraction","fraction":1.0}}'
)

candles <- function() {
  parts <- vapply(0:39, function(i) {
    px <- 100.0 + 10.0 * sin(i * 0.5)
    paste0(
      '{"time":', i,
      ',"open":', px, ',"high":', px + 1.0, ',"low":', px - 1.0,
      ',"close":', px, ',"volume":100.0}'
    )
  }, character(1))
  paste0("[", paste(parts, collapse = ","), "]")
}

test_envelope <- function(cmd) {
  paste0(
    '{"cmd":"', cmd, '","test":{',
    '"id":"momentum","strategy":', strategy,
    ',"dataset_ref":"sym-01","property_checks":[{"kind":"no_nan"}]},',
    '"data":{"sym-01":', candles(), '}}'
  )
}

## version
stopifnot(nzchar(wkstrategyci_version()))

## a freshly-blessed test passes with an empty diff
session <- wkstrategyci_new()
result <- wkstrategyci_command(session, test_envelope("run_test"))
stopifnot(grepl('"id":"momentum"', result, fixed = TRUE))
stopifnot(grepl('"passed":true', result, fixed = TRUE))
stopifnot(grepl('"diff":[]', result, fixed = TRUE))

## run_test is byte-identical across sessions (the cross-language golden core)
session2 <- wkstrategyci_new()
result2 <- wkstrategyci_command(session2, test_envelope("run_test"))
stopifnot(identical(result, result2))

## bless then re-run matches with an empty diff
blessed <- wkstrategyci_command(session, test_envelope("bless"))
stopifnot(grepl('"expected"', blessed, fixed = TRUE))
rerun_cmd <- paste0(
  '{"cmd":"run_test","test":', blessed,
  ',"data":{"sym-01":', candles(), '}}'
)
rerun <- wkstrategyci_command(session, rerun_cmd)
stopifnot(grepl('"passed":true', rerun, fixed = TRUE))
stopifnot(grepl('"diff":[]', rerun, fixed = TRUE))

## an unknown command is an in-band error, not a hard error
inband <- wkstrategyci_command(session, '{"cmd":"nope"}')
stopifnot(grepl('"ok":false', inband, fixed = TRUE))

## cross-language golden: build the run_suite command from the committed
## golden/{tests,data} corpus and assert the response equals
## golden/expected/suite.json byte-for-byte — the exact SuiteResult the Rust
## core and every other binding produce. The corpus lives at the repo root;
## walk up from the working directory to find it, skipping cleanly if absent.
golden_dir <- function() {
  d <- normalizePath(getwd(), mustWork = FALSE)
  for (i in seq_len(10)) {
    g <- file.path(d, "golden")
    if (dir.exists(file.path(g, "tests"))) {
      return(g)
    }
    d <- dirname(d)
  }
  NULL
}

load_golden_data <- function(g) {
  parts <- character(0)
  for (csv in sort(list.files(file.path(g, "data"), pattern = "\\.csv$", full.names = TRUE))) {
    rows <- character(0)
    lines <- readLines(csv, warn = FALSE)
    for (idx in seq_along(lines)) {
      line <- trimws(lines[idx])
      if (!nzchar(line)) next
      cols <- trimws(strsplit(line, ",")[[1]])
      t <- suppressWarnings(as.integer(cols[1]))
      if (is.na(t)) next
      rows <- c(rows, paste0(
        '{"time":', cols[1], ',"open":', cols[2], ',"high":', cols[3],
        ',"low":', cols[4], ',"close":', cols[5], ',"volume":', cols[6], '}'
      ))
    }
    name <- sub("\\.csv$", "", basename(csv))
    parts <- c(parts, paste0('"', name, '":[', paste(rows, collapse = ","), "]"))
  }
  paste0("{", paste(parts, collapse = ","), "}")
}

g <- golden_dir()
if (!is.null(g)) {
  tests <- vapply(
    sort(list.files(file.path(g, "tests"), pattern = "\\.json$", full.names = TRUE)),
    function(p) trimws(paste(readLines(p, warn = FALSE), collapse = "\n")),
    character(1)
  )
  suite_cmd <- paste0(
    '{"cmd":"run_suite","tests":[', paste(tests, collapse = ","),
    '],"data":', load_golden_data(g), "}"
  )
  got <- wkstrategyci_command(session, suite_cmd)
  want <- trimws(paste(
    readLines(file.path(g, "expected", "suite.json"), warn = FALSE), collapse = "\n"
  ))
  stopifnot(identical(trimws(got), want))
}

cat("wickra-strategy-ci R tests passed\n")
