# A runnable R example: golden-pin a strategy with bless, then re-run it and
# confirm it passes. The report is recomputed by the engine, so the golden is an
# exact, reproducible anchor.
#
#   R CMD INSTALL bindings/r   # with WKSTRATEGYCI_INC / WKSTRATEGYCI_LIB set
#   Rscript examples/r/run.R

library(wickrastrategyci)

symbol <- "AAA"

strategy <- paste0(
  '{"symbol":"AAA","timeframe":"1h",',
  '"indicators":{"fast":{"type":"Ema","params":[3]},',
  '"slow":{"type":"Ema","params":[8]}},',
  '"entry":{"cross_above":["fast","slow"]},',
  '"exit":{"cross_below":["fast","slow"]},',
  '"sizing":{"type":"fixed_fraction","fraction":0.95}}'
)

closes <- c(120, 118, 116, 114, 112, 110, 108, 112, 116, 120, 124, 128)

candles <- function() {
  parts <- vapply(seq_along(closes), function(i) {
    close <- closes[i]
    open <- if (i == 1) close else closes[i - 1]
    paste0(
      '{"time":', 1700000000 + (i - 1) * 3600,
      ',"open":', open, ',"high":', max(open, close) + 1,
      ',"low":', min(open, close) - 1, ',"close":', close, ',"volume":1000}'
    )
  }, character(1))
  paste0("[", paste(parts, collapse = ","), "]")
}

data <- paste0('{"', symbol, '":', candles(), "}")
test <- paste0(
  '{"id":"ema_crossover","strategy":', strategy,
  ',"dataset_ref":"', symbol, '","property_checks":[{"kind":"no_nan"}]}'
)

session <- wkstrategyci_new()
cat("wickra-strategy-ci", wkstrategyci_version(), "\n")

blessed <- wkstrategyci_command(session, paste0('{"cmd":"bless","test":', test, ',"data":', data, "}"))
rerun <- wkstrategyci_command(
  session,
  paste0('{"cmd":"run_test","test":', blessed, ',"data":', data, "}")
)
stopifnot(grepl('"passed":true', rerun, fixed = TRUE))
cat("blessed test: PASS (golden pinned)\n")
