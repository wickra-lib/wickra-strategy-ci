"use strict";

// Golden test over the wasm-pack (nodejs target) output: the WebAssembly build
// runs a strategy test and blesses a golden, byte-identically to the native run.
// Skips cleanly when `pkg/` has not been built yet
// (`wasm-pack build --target nodejs`).

const { test } = require("node:test");
const assert = require("node:assert");
const path = require("node:path");

let wasm = null;
try {
  wasm = require(path.resolve(__dirname, "..", "pkg", "wickra_strategy_ci_wasm.js"));
} catch {
  wasm = null;
}

const STRATEGY = {
  symbol: "TEST",
  timeframe: "1h",
  indicators: {
    fast: { type: "Sma", params: [2] },
    slow: { type: "Sma", params: [5] },
  },
  entry: { cross_above: ["fast", "slow"] },
  exit: { cross_below: ["fast", "slow"] },
  sizing: { type: "fixed_fraction", fraction: 1.0 },
};

function candles() {
  const out = [];
  for (let i = 0; i < 40; i += 1) {
    const close = 100.0 + 10.0 * Math.sin(i * 0.5);
    out.push({ time: i, open: close, high: close + 1, low: close - 1, close, volume: 100 });
  }
  return out;
}

function runTestCmd() {
  return JSON.stringify({
    cmd: "run_test",
    test: {
      id: "momentum",
      strategy: STRATEGY,
      dataset_ref: "sym-01",
      property_checks: [{ kind: "no_nan" }],
    },
    data: { "sym-01": candles() },
  });
}

test("wasm build present or skipped", (t) => {
  if (!wasm) t.skip("run `wasm-pack build --target nodejs` first");
});

if (wasm) {
  test("wasm run_test is byte-identical across calls", () => {
    const a = new wasm.Session().command(runTestCmd());
    const b = new wasm.Session().command(runTestCmd());
    assert.strictEqual(a, b);
  });

  test("wasm bless then re-run matches with an empty diff", () => {
    const session = new wasm.Session();
    const blessed = JSON.parse(
      session.command(
        JSON.stringify({
          cmd: "bless",
          test: {
            id: "momentum",
            strategy: STRATEGY,
            dataset_ref: "sym-01",
            property_checks: [{ kind: "no_nan" }],
          },
          data: { "sym-01": candles() },
        }),
      ),
    );
    assert.ok(blessed.expected);

    const rerun = JSON.parse(
      new wasm.Session().command(
        JSON.stringify({ cmd: "run_test", test: blessed, data: { "sym-01": candles() } }),
      ),
    );
    assert.deepStrictEqual(rerun.diff, []);
    assert.strictEqual(rerun.passed, true);
  });

  test("wasm version matches the module export", () => {
    assert.strictEqual(new wasm.Session().version(), wasm.version());
  });

  test("wasm reports an unknown command in-band", () => {
    const response = JSON.parse(new wasm.Session().command('{"cmd":"nope"}'));
    assert.strictEqual(response.ok, false);
  });
}
