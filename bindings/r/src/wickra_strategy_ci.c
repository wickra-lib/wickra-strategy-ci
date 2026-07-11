/* R .Call glue for the wickra-strategy-ci C ABI hub. */
#include <R.h>
#include <Rinternals.h>
#include <R_ext/Rdynload.h>
#include <stddef.h>
#include "wickra_strategy_ci.h"

/* --- handle lifetime ----------------------------------------------------- */

static void wkstrategyci_finalize(SEXP ext) {
    WickraStrategyCi *h = (WickraStrategyCi *)R_ExternalPtrAddr(ext);
    if (h) {
        wickra_strategy_ci_free(h);
    }
    R_ClearExternalPtr(ext);
}

static WickraStrategyCi *handle_of(SEXP ext) {
    WickraStrategyCi *h = (WickraStrategyCi *)R_ExternalPtrAddr(ext);
    if (!h) {
        Rf_error("wickra-strategy-ci: handle is closed");
    }
    return h;
}

/* --- exported .Call entries ---------------------------------------------- */

SEXP wkstrategyci_version(void) {
    return Rf_mkString(wickra_strategy_ci_version());
}

SEXP wkstrategyci_new(void) {
    WickraStrategyCi *h = wickra_strategy_ci_new();
    if (!h) {
        Rf_error("wickra-strategy-ci: failed to create a session");
    }
    SEXP ext = PROTECT(R_MakeExternalPtr(h, R_NilValue, R_NilValue));
    R_RegisterCFinalizerEx(ext, wkstrategyci_finalize, TRUE);
    UNPROTECT(1);
    return ext;
}

SEXP wkstrategyci_command(SEXP ext, SEXP cmd_json) {
    WickraStrategyCi *h = handle_of(ext);
    const char *cmd = CHAR(STRING_ELT(cmd_json, 0));

    /* Length-out protocol: learn the length, then read into a caller buffer.
       Domain errors come back in-band as {"ok":false,...} JSON, not a negative
       code; only unusable arguments / a caught panic return < 0. */
    int len = wickra_strategy_ci_command(h, cmd, NULL, 0);
    if (len < 0) {
        Rf_error("wickra-strategy-ci: command failed (code %d)", len);
    }
    char *buf = (char *)R_alloc((size_t)len + 1, 1);
    wickra_strategy_ci_command(h, cmd, buf, (size_t)len + 1);
    return Rf_mkString(buf);
}

/* --- registration -------------------------------------------------------- */

static const R_CallMethodDef CallEntries[] = {
    {"wkstrategyci_version", (DL_FUNC)&wkstrategyci_version, 0},
    {"wkstrategyci_new", (DL_FUNC)&wkstrategyci_new, 0},
    {"wkstrategyci_command", (DL_FUNC)&wkstrategyci_command, 2},
    {NULL, NULL, 0}};

void R_init_wickrastrategyci(DllInfo *dll) {
    R_registerRoutines(dll, NULL, CallEntries, NULL, NULL);
    R_useDynamicSymbols(dll, FALSE);
}
