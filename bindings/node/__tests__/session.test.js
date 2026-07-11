"use strict";

const { test } = require("node:test");
const assert = require("node:assert");
const { Session } = require("../index.js");

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

test("run_test roundtrip passes and has an empty diff", () => {
  const session = new Session();
  const result = JSON.parse(session.command(runTestCmd()));
  assert.strictEqual(result.id, "momentum");
  assert.strictEqual(result.passed, true);
  assert.deepStrictEqual(result.diff, []);
});

test("an unknown command returns an error JSON, not a throw", () => {
  const session = new Session();
  const response = JSON.parse(session.command(JSON.stringify({ cmd: "nope" })));
  assert.strictEqual(response.ok, false);
});

test("version is a string", () => {
  const session = new Session();
  assert.strictEqual(typeof session.version(), "string");
});

module.exports = { STRATEGY, candles, runTestCmd };
