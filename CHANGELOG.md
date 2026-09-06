# Changelog

All notable changes to this project are documented here. The format is based on
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project
adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Fixed

- **The crate could not be published at all.** `strategy-ci-core` declared
  `proof-core` as an optional dependency taken from git. `cargo publish`
  resolves *every* dependency against the registry, optional ones included, so
  the very first step of the release — `cargo publish -p strategy-ci-core` —
  failed with `no matching package named proof-core found`, and with it every
  job gated behind it, up to and including the GitHub release. Nothing in CI
  could see this: CI builds from git, and the publish path only ever runs on a
  tag.

- **The engine under test was 130 commits stale.** `wickra-backtest-core` was
  consumed as a git dependency, whose lockfile rev sat at 26 July — five
  releases (0.1.0 → 0.1.4) behind the engine a user of this crate resolves,
  because the manifest declared `version = "0.1"` alongside the git source. So
  the goldens were blessed against one engine and shipped against another. A git
  rev never goes red; it only gets older. Every Wickra dependency now comes from
  crates.io at an exact patch (`0.1.4`), and the goldens are re-verified against
  it byte-for-byte. `deny.toml` no longer exempts the org from
  `unknown-git = "deny"`, so a git source coming back fails the build instead of
  ageing quietly.

### Removed

- **The optional `proof` feature**, and with it the `report_hash` field and its
  per-request flag. It hashed a report through `wickra-proof`'s `proof-core`,
  which has no crates.io release — that is what made this crate unpublishable.
  The feature was always a ROADMAP *Next* item rather than a 0.1 promise, and it
  is restored the moment wickra-proof ships. Removing it changes no bytes on the
  wire: the field was `skip_serializing_if = "Option::is_none"` and the default
  build never populated it, so every golden is unchanged. The guard test that
  the JSON boundary carries no build-dependent field stays, because the rule it
  protects outlives the field that broke it.

- **A build flag changed the wire format.** With the `proof` feature compiled in,
  every response carried a `report_hash`, so a library built with
  `--all-features` failed every binding's golden comparison against a corpus
  pinned without it. The C# and Java suites hit exactly that, twice, and the Rust
  golden test had been quietly stripping the field for the same reason.
  That contradicts the claim this boundary exists to make: the same command
  produces the same bytes in ten languages. The hash is now opt-in **per request**
  (`"report_hash": true`) rather than per build, so the default response is
  identical whatever the library was compiled with. Two tests pin it, and both
  pass with the feature on and off.


- **One broken test took the whole suite down.** `run_suite` collected its
  results with `?`, so a single test whose `dataset_ref` did not resolve — or
  whose spec the engine rejected — made the call return `Err`. The CLI printed
  one error line and exited, and you learned nothing about the other tests in the
  run. For a test runner that is the wrong shape of failure: a typo in one file
  hid the verdict on every other file. A test that cannot be run is now a
  **failing test** that carries the reason, and the suite still reports on
  everything else.
  `TestResult` gains an `error` field, skipped when absent, so a result for a
  test that ran serialises exactly as before and every committed golden is
  unchanged.
- **The release workflow kept its token in the runner's git config.** The
  `major-tag` job was the one checkout in the repository with
  `persist-credentials: true`, and it kept them for the whole job to run a single
  `git push`. Moving a ref needs no working tree and no git credentials — the
  API does it directly — so the checkout is gone with them. The API call also
  handles the first release of a major version, where there is no tag to move
  yet.

### Added

- **Every binding now checks that the batch path equals the per-test path.**
  `run_suite` fans the corpus out across rayon and sorts the results by id;
  `run_test` walks one test at a time. Those are two different engines reached
  through the same boundary, and only the Rust core
  (`tests/suite_eq_seq.rs`) ever tested that they agree — through eight
  different FFI surfaces, the parallel path is a separate claim each time. A
  regression would have shown up as a suite that passes while an individual run
  of the same test does not.
  The WASM case covers the other half: that build is compiled with
  `--no-default-features`, so its `run_suite` walks the tests sequentially where
  every native binding parallelises them. Its test also submits the tests out of
  order, since a suite that returned them in submission order would pass a
  same-order comparison.

### Fixed


- **The WASM npm package would have published without its licence texts.**
  `wickra-strategy-ci-wasm` declares `MIT OR Apache-2.0` and its generated
  manifest listed only the three build outputs; the release staged no copies for
  it. The other seven npm packages were fixed by naming the texts in a manifest
  under review, which is not possible here because wasm-pack generates this one —
  so the release now stages the copies, adds them to the generated `files`, and
  proves with `npm pack --dry-run` that they are in the tarball before
  publishing. wasm-pack had been saying so on every build: "License key is set in
  Cargo.toml but no LICENSE file(s) were found".
- **`.gitignore` and `.Rbuildignore` disagreed about the R build artefacts.**
  `configure`/`configure.win` stage a downloaded header and generate an import
  library, a `.def` and `Makevars`; `.Rbuildignore` lists them all so they stay
  out of the package tarball, but `.gitignore` covered only `*.o`, `*.so` and
  `*.dll` — and `*.dll` does not match `*.dll.a`. A `git add -A` after an R build
  would have committed a downloaded header and a generated import library.


- **`Cargo.lock` did not match the manifests, so `cargo build --locked` failed.**
  When the benchmark crate moved to `codspeed-criterion-compat`, the lockfile was
  not regenerated with it: `codspeed`, `codspeed-criterion-compat` and their
  transitive dependencies were absent entirely. Every `--locked` build would have
  refused to start, and the CodSpeed job could not have built its benchmarks. The
  lockfile is regenerated and `cargo build --locked --workspace --all-features`
  now succeeds.


- The `semver` and `examples-smoke` jobs pinned `Swatinem/rust-cache` with a bare
  `# v2` comment, too coarse for Dependabot to act on; both now name v2.9.2, the
  version the rest of the organisation pins.
- `scripts/update-lockfiles.sh` pinned uv 0.12.7; it now pins 0.12.10, with the
  checksums taken from the release's own `.sha256` files and confirmed by
  downloading and hashing the archive rather than copied on trust.
- The one checkout that keeps its credentials -- `major-tag`, which pushes a tag
  and cannot work without them -- now says so where a reader meets it.
- The README's Requirements table gained R's lower bound, which `DESCRIPTION`
  declares but the table did not repeat.


<<<<<<< HEAD
<<<<<<< HEAD
- **JSON float parsing was not bit-exact, in a project whose claim is
  byte-identical results.** `serde_json`'s default float parser is fast but can
  land one ULP from the value the text names, so `parse(serialize(x)) != x` for
  some inputs. The `test_parse` fuzz target found it on its first run after the
  new workflows landed: `1888888888888288888888855` parsed to
  `1.8888888888882886e24`, serialized, and came back as `1.888888888888289e24`.
  A golden is text that is re-parsed on every run, so an inexact parser
  undermines both reproducibility between runs and identity across the ten
  bindings. The workspace and the fuzz crate now enable `serde_json`'s
  `float_roundtrip` feature, with a unit test pinning the property, since nothing
  else in the build would notice the feature being dropped.
- **`js-yaml` 4.3.0 carried a high-severity advisory** (GHSA-5p4m-2wfm-xmqj,
  CVSS 7.5), reachable as a transitive development dependency of
  `@napi-rs/cli`. Found by the osv-scanner job the moment it was wired up;
  `bindings/node/package-lock.json` now resolves 4.3.2.
=======
>>>>>>> 1113e5e (fix: complete the binding inventory and stop the R test reaching outside its package)
=======
- **Every packaged binding README linked its licence relatively.** `../../LICENSE-MIT`
  resolves on GitHub and nowhere else: on PyPI, npm and pkg.go.dev the link is
  simply dead, and nothing said so because the file it points at does exist in
  the repository. Found by the new `check_readme_links.py`, which now guards it.

### Added

- **Five verification scripts, and the CI job that runs them.** Each covers a
  failure that is invisible from a build:
  - `check_binding_surface.py` — reads the C ABI header as the source of truth
    and holds all eight bindings to it. Nothing compared the bindings *to each
    other*, so a method going missing in one of them failed nowhere: its own
    tests simply stopped exercising it.
  - `check_version_sync.py` — the version lives in twelve declarations across
    six package managers; a bump that misses one ships a package pinning a
    native binary that was never published, which surfaces on a user's machine
    after the tag.
  - `check_license_copies.py` — cargo packages from git, so a licence copy that
    is untracked is missing from the `.crate` without a word.
  - `check_readme_links.py` — see above.
  - `check_r_abi_skew.py` — R is the one binding whose native half comes from a
    published release rather than this tree, so the pairing r-universe compiles
    is one our own R job never sees.
- **`examples-smoke`** — `examples/` carried nine runnable programs and CI ran
  none of them. An example that stopped compiling would have been found by a
  reader, which is the worst place to find out.
- **`semver`** — nothing checked that a patch release keeps the public API,
  across a surface ten bindings sit on top of.
- **`python-wheel-container-smoke`** — the Linux wheels are built inside
  manylinux and musllinux containers while the ordinary job builds on the
  runner, so a dependency needing a system library the container lacks would
  have failed during the release, after the tag.
- **`links`** in CI, non-blocking, alongside the authoritative weekly run: it
  catches a link the pull request itself broke, while that is still cheap.
- **A golden test for the C binding** (`examples/c/golden_test.c`). Every other
  binding ran the golden corpus through its own client; C did not, leaving the
  hub that the C++, C#, Go, Java and R bindings all call through exercised only
  from Rust. It compiles against the shipped header with a real C compiler and
  asserts the suite byte for byte.
- `scripts/update-lockfiles.sh` — the lockfiles had no regeneration script. uv is
  fetched only on request (`WKSTRATEGYCI_BOOTSTRAP_UV=1`) and its release is
  checksum-verified, rather than piping an installer into a shell.


>>>>>>> 1570892 (ci: add the five verification scripts and the jobs that run them)
- **The shipped R test reached outside the package.** `tests/run_tests.R` walked
  up to ten directories looking for `golden/`. `R CMD check` runs the shipped
  tests inside an unpacked tarball, where no repository lies above the package,
  so the cross-language comparison either skipped silently on every real check
  run or bound to an unrelated directory that happened to carry that name. It is
  now split: `run_tests.R` is self-contained, and the golden comparison lives in
  `tests/golden_cross_language.R`, which is `.Rbuildignore`d and run from CI
  against the checkout.
- **The npm platform packages would have published without their licence
  texts.** All seven manifests name `MIT OR Apache-2.0`, but an SPDX expression
  is a reference to two documents, not the documents. The `files` field now lists
  them, and a release step stages the copies and proves with `npm pack
  --dry-run` that they are in each tarball before anything is published.
- **The main npm package shipped the platform stubs inside itself.** Its `files`
  listed `npm` and `*.node`; the native binaries reach a consumer through the
  per-platform packages in `optionalDependencies`, so the tarball is now the JS
  loader, the types and the licences.
- **The Linux npm stubs did not declare `libc`.** Without it npm installs the
  gnu package on a musl system, where the binary cannot load. Both gnu stubs now
  declare `glibc`.
- `bindings/r/DESCRIPTION` declared no lower bound on R.

<<<<<<< HEAD
- **`wickra-data` was declared and used by nothing.** It sat in
  `[workspace.dependencies]` pinned at `0.9` while the published crate is at
  `1.0.4`, so it was simultaneously dead weight and stale. No crate imported it:
  the CLI reads its own CSV and `Candle` is re-exported from the engine. Adding
  it back would mean two crates defining the same row, so it is removed rather
  than bumped.
- Neither published crate declared `[package.metadata.docs.rs] all-features`, so
  docs.rs would have rendered the default feature set only, leaving the optional
  `proof` API and the rayon-backed `parallel` path invisible to a reader.
- Neither published crate carried the licence texts it names. `cargo` decides
  what to package from git, so an untracked copy makes `cargo publish` refuse the
  tree and a gitignored one is dropped from the tarball; the copies are committed
  beside each manifest.

- **The fuzz axis fed the engine candles no market could print.** `jitter`
  scaled `open`, `high`, `low` and `close` by four independent draws, so
  whenever the jitter exceeded a bar's own range the high landed below the low;
  on a bar with a 0.1% range and a 5% jitter that happened in 84 of 200 runs.
  `gap_shock` moved `close` without regard to the range it had to sit inside,
  putting it outside `[low, high]`. `wickra-backtest` does not validate its
  input, so those bars were accepted silently and any property that failed on
  them said nothing about the strategy — the axis was reporting artefacts of its
  own generator. Perturbed candles are now repaired to satisfy the OHLC ordering
  (and non-negative volume) before they reach the engine. The repair keeps every
  price the PRNG drew and only re-assigns which is the high and which the low,
  so determinism is unchanged and it is a no-op on a bar that was already valid;
  the committed goldens are unaffected. `docs/FUZZING.md` had already promised
  "perturbed but well-formed data".

- **The release pipeline could not have published the CLI.** Five steps referred
  to a crate `wickra-strategy-ci-cli`; the package is named `wickra-strategy-ci`
  (only its directory is `crates/strategy-ci-cli`). `cargo publish -p
  wickra-strategy-ci-cli` answers "package ID specification did not match any
  packages", so both the `cli-binaries` build and the crates.io publish would
  have failed on the first tag. The SBOM step also copied from a directory that
  does not exist, and the README's crates.io badge pointed at a crate that will
  never exist.
- **A `workflow_dispatch` from any branch would have published.** `release.yml`
  accepts a manual start so a failed publish can be retried without moving the
  tag, and the `release` environment carries no branch policy, so nothing stopped
  a dispatch from `main` publishing whatever main contained — and tagging the Go
  mirror `vmain`. The gate now requires `refs/tags/v*`, testing the ref rather
  than the ref name, since a branch called `v1` would pass a name check.
- **A failed build stopped one registry, not the release.** Each publish job
  depended only on the artefacts it consumed, so a broken wheel build stopped
  PyPI while crates.io, npm and NuGet published anyway — and a version burned on
  four registries out of seven cannot be rolled back. Every publish now sits
  behind a gate that opens only once all builds succeed.
- **The gate reads a still-running CI as an undecided one, not a failed one.**
  `ci.yml` does not fire on a tag push, so the run that matters is the one from
  when the commit was on `main`; waiting for a decision keeps a correct tag from
  being refused for a run that had simply not finished.
- **`github-release` did not wait for three of its producers.** `csharp-publish`,
  `java-publish` and `go-mirror` could fail while the release went out reporting
  success.
- **The Go mirror was pushed and tagged without ever being built.** A Go tag is
  immutable on the module proxy, so a mirror that does not compile is permanent.
  It is now built and vetted exactly as a consumer would, from the module
  directory with no repository above it. Its `*_test.go` files are also no longer
  shipped: they read the `golden/` corpus from above the module, so `go test
  ./...` failed for anyone depending on the mirror.
- **Maven Central would have rejected the deployment.** The POM declared no
  `<scm>` ("SCM URL is not defined") and no `<developers>` ("Developers
  information is missing"), and named a single licence `MIT OR Apache-2.0` with
  no URL. `mvn -Prelease` matched no profile at all, so Maven only warned and
  deployed bare: no sources JAR, no javadoc JAR, no signatures, and no publishing
  plugin to send them with. The profile now exists, with
  `waitUntil=published` so a green job means published rather than accepted.
- **Provenance covered the crates and wheels only** — not the NuGet package, the
  Maven JAR or the six C ABI libraries, which are precisely the artefacts that
  are native binaries rather than source. The Maven JAR also never reached the
  GitHub Release at all.

=======
>>>>>>> 1113e5e (fix: complete the binding inventory and stop the R test reaching outside its package)
### Added

- **`bindings/r/configure` and `configure.win`.** The R binding had neither, so
  it could only ever build against a C ABI already present in the environment —
  which is what CI provides and what a user installing from r-universe does not
  have. Both now download the `wickra-strategy-ci-c-<triple>.tar.gz` asset
  matching `DESCRIPTION: Version`, stage it into `src/`, and bake an rpath
  ($ORIGIN / @loader_path) so the bundled library is found after install.
  `src/Makevars.in` and `src/install.libs.R` come with them; `src/Makevars` is
  now generated rather than committed.
- **`bindings/c/include/wickra_strategy_ci.hpp`** — the C++ hull. The C ABI hands
  out a handle that must be freed exactly once and a two-call buffer protocol for
  `command`, and every C++ caller re-implemented both by hand. `Session` is a
  move-only RAII owner and `command` returns a `std::string`. `examples/c/run.cpp`
  now uses it, which is what keeps it compiling.
- `bindings/csharp/README.md` — the binding-level landing page. The one beside
  the `.csproj` is packaged into the `.nupkg` and is a different document.
- `examples/wasm/run.mjs` — WASM was the only binding without a runnable example.

<<<<<<< HEAD
### Changed

- The release notes move to `.github/release-notes.md`, rendered with `envsubst`
  and passed as `body_path`. Inlined in the workflow, their `uses:` usage example
  was indistinguishable from a real unpinned step to anything auditing for them.


=======
>>>>>>> 1113e5e (fix: complete the binding inventory and stop the R test reaching outside its package)

- **Governance and community docs described a different product.** `SECURITY.md`,
  `SUPPORT.md`, `GOVERNANCE.md`, `CONTRIBUTING.md`, the bug-report issue template
  and the pull-request template referred to a `ScanSpec`, a sample "universe" and
  an optional `live` feature — none of which exist in this repository. They now
  describe the `StrategyTest` model, recorded datasets, and the actual `parallel`
  and `proof` features.
- **`deny.toml` carried a dead licence exception.** The `CDLA-Permissive-2.0`
  allowance for `webpki-roots` was justified by a TLS stack that this crate never
  pulls in; `webpki-roots` is not in `Cargo.lock`. The exception list is now
  empty.
- `README.md` and `ARCHITECTURE.md` advertised four tolerance kinds ("exact,
  absolute, relative, or ULP"). The runner implements `abs` and `rel`, falling
  back to exact equality when a field names no tolerance; there is no ULP mode.
- `README.md` listed invariant properties that do not exist (`equity never
  negative`); it now names the seven that do.
- `CHANGELOG.md` credited the composite action with a `@v1` moving major tag; no
  tag has been published yet.
- `SECURITY.md` stated that security fixes target "the most recent published
  version" as though one existed.
- The feature-request issue template asked for "a new comparator, a cross-section
  or breadth metric" and described "the screen you can't express today" — text
  carried over from `wickra-screener`. It now asks about tolerances, properties
  and perturbations.


- `LICENSES/MIT.txt` and `LICENSES/Apache-2.0.txt` — the SPDX licence texts at
  their conventional paths, alongside the existing `LICENSE-MIT` /
  `LICENSE-APACHE`.
- `docs/README.md` — an index of the documentation kept beside the code, and a
  signpost to what deliberately lives elsewhere (`docs.wickra.org`, and the
  `StrategySpec` schema in `wickra-backtest`).
- `README.md` gained the sections the repository blueprint requires:
  `Benchmarks`, `Requirements`, `Project layout`, `Building everything from
  source`, `Testing` and `Ecosystem`, plus a runnable quickstart directly under
  the badges and a worked `StrategyTest` example.
- `README.md` and `ARCHITECTURE.md` now state that the golden diff compares
  numeric leaves only — strings, booleans and nulls are not pinned.
- Five issue templates the blueprint requires: `bug_report_detailed`,
  `feature_request_detailed`, `performance_regression`, `documentation` and
  `question`. The detailed bug template asks whether a defect reproduces in a
  second binding and on the sequential path, because a cross-language or
  parallel-only disagreement is the class of bug this project most needs
  reported precisely.
- `.github/PULL_REQUEST_TEMPLATE/detailed.md` — the long-form template, covering
  wire-format impact, binding parity across all ten languages, and determinism.
  GitHub offers no picker for it, so `PULL_REQUEST_TEMPLATE.md` now says how to
  reach it (`?template=detailed.md`).
- `.github/codeql/codeql-config.yml`, referenced from the CodeQL workflow, so the
  napi-derive glue in `bindings/node/src/lib.rs` stops producing an
  `access-invalid-pointer` finding per exported class while the rule stays active
  over `bindings/c/src`, where the real raw-pointer code lives.
- CodeQL now analyses **C# and Java**, the two bindings that reach the core
  across the C ABI. Both build with `build-mode: manual`; without a build no
  dependency resolves and GitHub reports the analysis as low quality.
- Dependabot now covers `fuzz/` (a detached workspace the root cargo entry cannot
  reach) and `examples/go` (its own module). Both would otherwise never see a
  dependency update.


- **`strategy-ci-core`** — the data-driven test runner: a serde `StrategyTest`
  model carrying an opaque `wickra-backtest` `StrategySpec`, golden-diff with
  per-field `abs`/`rel` tolerances, invariant property checks (`no_nan`,
  `monotone_equity`, `max_drawdown_le`, `min_trades_ge`, `sharpe_ge`, `pnl_ge`,
  `field_in_range`) with `metrics.<name>` path resolution, and a seeded
  `rand_pcg` fuzz axis (`jitter`/`dropout`/`gap_shock`). Optional `proof` feature
  surfaces a BLAKE3 report hash.
- **`wickra-strategy-ci` CLI** — `run` (non-zero exit on any failure — the CI
  gate), `bless`, `list`, `version`, text and JSON output.
- **Ten-language bindings** over one `Session`/`command_json` boundary: Rust,
  Python (PyO3), Node.js (napi-rs) and WASM (wasm-bindgen) natively, plus C, C++,
  C#, Go, Java and R over a C ABI hub — byte-identical results across all.
- **Composite GitHub Action** (`action.yml`) that installs the released CLI and
  runs a strategy-test directory, failing the workflow on any failing test, with
  a dogfooding self-test. It builds the CLI from git until the first release
  ships a binary asset.
- **Golden corpus** (`golden/`) run by every binding for cross-language
  byte-identity, four integration test suites, four cargo-fuzz targets, and a
  criterion benchmark.
- Full CI across all ten languages, CodeQL, OpenSSF Scorecard, zizmor, a nightly
  benchmark, a weekly link check, metadata-drift audit, and a tag-gated release
  pipeline to crates.io, PyPI, npm, NuGet, Maven Central, r-universe and Go.
- Project scaffolding: the workspace manifest, dual `MIT OR Apache-2.0` license,
  supply-chain and link config (`deny.toml`, `lychee.toml`, `osv-scanner.toml`,
  `repo-metadata.toml`), and the governance and community docs.

[Unreleased]: https://github.com/wickra-lib/wickra-strategy-ci/commits/main
