"use strict";

// The cross-language golden invariant seen from Node: the same command yields
// byte-identical output across calls, and a blessed golden re-matches itself. The
// response bytes are what every other binding produces too.

const { test } = require("node:test");
const assert = require("node:assert");
const { Session } = require("../index.js");
const { STRATEGY, candles, runTestCmd } = require("./session.test.js");

test("run_test is byte-identical across calls", () => {
  const a = new Session().command(runTestCmd());
  const b = new Session().command(runTestCmd());
  assert.strictEqual(a, b);
});

test("bless then re-run matches with an empty diff", () => {
  const session = new Session();
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
    session.command(
      JSON.stringify({ cmd: "run_test", test: blessed, data: { "sym-01": candles() } }),
    ),
  );
  assert.deepStrictEqual(rerun.diff, []);
  assert.strictEqual(rerun.passed, true);
});
