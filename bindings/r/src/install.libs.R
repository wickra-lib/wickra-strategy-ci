# Install the compiled package shared object plus the bundled C ABI library so
# the package is self-contained: on Windows wickra_strategy_ci.dll (matched by
# the *.dll glob); on Linux libwickra_strategy_ci.so (matched by the *.so
# SHLIB_EXT glob); on macOS libwickra_strategy_ci.dylib, added explicitly since
# R package objects use the .so extension there too. The Unix rpath baked by
# configure ($ORIGIN / @loader_path) resolves it from this libs directory.
files <- unique(c(Sys.glob(paste0("*", SHLIB_EXT)), Sys.glob("libwickra_strategy_ci.dylib")))
dest <- file.path(R_PACKAGE_DIR, paste0("libs", R_ARCH))
dir.create(dest, recursive = TRUE, showWarnings = FALSE)
file.copy(files, dest, overwrite = TRUE)
if (file.exists("symbols.rds")) {
  file.copy("symbols.rds", dest, overwrite = TRUE)
}
