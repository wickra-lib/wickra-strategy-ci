## Cross-language golden check for the R binding.
##
## NOT shipped in the package tarball (see .Rbuildignore): it reads the
## repository's golden/ corpus, which exists in a checkout and nowhere else. CI
## runs it from the repository root after the shipped tests pass.
##
## It builds the run_suite command from golden/{tests,data} and asserts the
## response equals golden/expected/suite.json byte-for-byte -- the exact
## SuiteResult the Rust core and every other binding produce.

library(wickrastrategyci)

golden_dir <- function() {
  d <- normalizePath(getwd(), mustWork = FALSE)
  for (i in seq_len(10)) {
    g <- file.path(d, "golden")
    if (dir.exists(file.path(g, "tests"))) {
      return(g)
    }
    d <- dirname(d)
  }
  stop("golden/ not found: run this from the repository checkout")
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
session <- wkstrategyci_new()
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

## The batch path against the per-test path. run_suite fans the corpus out across
## rayon and sorts the results by id; run_test walks one test at a time. Those are
## two different engines reached through the same C ABI boundary, and only the
## Rust core tested that they agree -- from a binding, the parallel path crossing
## the boundary is a separate claim.
##
## Compared as text: each golden file is named after the id it carries, so the
## sorted file order is the sorted id order run_suite emits, and the per-test
## responses concatenated are exactly the suite's results array.
marker <- '"results":['
from <- regexpr(marker, got, fixed = TRUE)
stopifnot(from > 0)
batch <- substring(got, from + nchar(marker))
to <- regexpr('],"passed":', batch, fixed = TRUE)
stopifnot(to > 0)
batch <- substring(batch, 1, to - 1)

data_json <- load_golden_data(g)
individual <- vapply(tests, function(test) {
  wkstrategyci_command(
    session,
    paste0('{"cmd":"run_test","test":', test, ',"data":', data_json, "}")
  )
}, character(1))

stopifnot(identical(paste(individual, collapse = ","), batch))

cat("wickra-strategy-ci R cross-language golden matches\n")
cat("wickra-strategy-ci R batch path equals the per-test path\n")
