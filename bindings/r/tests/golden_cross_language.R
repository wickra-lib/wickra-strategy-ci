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

cat("wickra-strategy-ci R cross-language golden matches\n")
