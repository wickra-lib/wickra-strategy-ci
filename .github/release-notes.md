<!--
Release-notes template. Rendered by the `github-release` job in
.github/workflows/release.yml, which substitutes ${TAG} and ${VERSION} with
envsubst and passes the result to the release action as `body_path`.

It lives outside the workflow on purpose. The YAML example below is
documentation, but inside a workflow file it reads as a real `uses:` step to
anything scanning for unpinned actions -- the repository's own audit flagged it
as exactly that. Here it is prose, and the workflow is shorter for it.

Only ${TAG} and ${VERSION} are substituted. Any other shell-looking text is
passed through literally, so `$` in an example needs no escaping.
-->
wickra-strategy-ci ${TAG} — Jest for trading strategies: golden-pin a strategy's backtest report, catch regressions in CI, and property/fuzz-test it against perturbed data. Native Rust, Python, Node.js, WASM plus a C ABI hub for C, C++, C#, Go, Java, R, and a composite GitHub Action.

### Install

```bash
cargo install wickra-strategy-ci
pip install wickra-strategy-ci
npm install wickra-strategy-ci
dotnet add package Wickra.StrategyCi
go get github.com/wickra-lib/wickra-strategy-ci/bindings/go
```

GitHub Action:

```yaml
- uses: wickra-lib/wickra-strategy-ci@v1
  with: { tests: tests, data: data }
```

Java (Maven Central):

```xml
<dependency>
  <groupId>org.wickra</groupId>
  <artifactId>wickra-strategy-ci</artifactId>
  <version>${VERSION}</version>
</dependency>
```

R (r-universe):

```r
install.packages("wickrastrategyci", repos = "https://wickra-lib.r-universe.dev")
```

### Attached assets

Pre-built artefacts for every supported platform — the same files this
workflow run published to crates.io, PyPI, and npm — plus standalone
CLI binaries and the C ABI libraries. C# ships to NuGet, Java to Maven
Central, the in-repo Go module is tagged `bindings/go/${TAG}`,
and R ships to r-universe via their own release jobs.

### Auto-generated changelog

See below; GitHub computes it from the commits since the previous tag.
