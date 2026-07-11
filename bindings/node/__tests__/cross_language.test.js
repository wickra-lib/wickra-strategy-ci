"use strict";

// Cross-language golden: build the run_suite command from the committed
// golden/{tests,data} corpus, run it through the binding, and assert the
// response equals golden/expected/suite.json byte-for-byte — the exact
// SuiteResult the Rust core and every other binding produce.

const { test } = require("node:test");
const assert = require("node:assert");
const fs = require("node:fs");
const path = require("node:path");
const { Session } = require("../index.js");

const GOLDEN = path.resolve(__dirname, "..", "..", "..", "golden");

function loadData() {
  const data = {};
  for (const file of fs.readdirSync(path.join(GOLDEN, "data")).sort()) {
    if (!file.endsWith(".csv")) continue;
    const rows = [];
    const lines = fs.readFileSync(path.join(GOLDEN, "data", file), "utf8").split("\n");
    lines.forEach((raw, idx) => {
      const line = raw.trim();
      if (!line) return;
      const c = line.split(",").map((x) => x.trim());
      const t = Number.parseInt(c[0], 10);
      if (Number.isNaN(t)) {
        if (idx === 0) return; // header
        throw new Error(`bad ts: ${c[0]}`);
      }
      rows.push({
        time: t,
        open: Number(c[1]),
        high: Number(c[2]),
        low: Number(c[3]),
        close: Number(c[4]),
        volume: Number(c[5]),
      });
    });
    data[path.basename(file, ".csv")] = rows;
  }
  return data;
}

test("run_suite matches the Rust SuiteResult golden byte-for-byte", () => {
  const tests = fs
    .readdirSync(path.join(GOLDEN, "tests"))
    .filter((f) => f.endsWith(".json"))
    .sort()
    .map((f) => JSON.parse(fs.readFileSync(path.join(GOLDEN, "tests", f), "utf8")));

  const cmd = JSON.stringify({ cmd: "run_suite", tests, data: loadData() });
  const got = new Session().command(cmd);
  const want = fs.readFileSync(path.join(GOLDEN, "expected", "suite.json"), "utf8").trim();

  assert.strictEqual(got, want, "SuiteResult must be byte-identical to the Rust golden");
  assert.strictEqual(JSON.parse(got).failed, 0);
});
