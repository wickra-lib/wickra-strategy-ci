#' The wickra-strategy-ci library version.
#' @return A version string.
#' @export
wkstrategyci_version <- function() {
  .Call(C_wkstrategyci_version)
}

#' Create a strategy-test session.
#' @return A `wickra_strategy_ci` handle (an external pointer).
#' @export
wkstrategyci_new <- function() {
  .Call(C_wkstrategyci_new)
}

#' Apply a command JSON and return the resulting response JSON.
#' @param session A session handle from [wkstrategyci_new()].
#' @param cmd_json A command JSON string.
#' @return The response as a JSON string.
#' @export
wkstrategyci_command <- function(session, cmd_json) {
  .Call(C_wkstrategyci_command, session, cmd_json)
}
