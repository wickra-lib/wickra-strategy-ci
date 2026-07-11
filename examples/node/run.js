// A runnable Node.js example: golden-pin a strategy with `bless`, then re-run it
// and confirm it passes. The report is recomputed by the engine, so the golden
// is an exact, reproducible anchor.
//
//   npm install wickra-strategy-ci
//   node examples/node/run.js

"use strict";

const { Session } = require("wickra-strategy-ci");

const SYMBOL = "AAA";

const TEST = {
  id: "ema_crossover",
  strategy: {
    symbol: SYMBOL,
    timeframe: "1h",
    indicators: {
      fast: { type: "Ema", params: [3] },
      slow: { type: "Ema", params: [8] },
    },
    entry: { cross_above: ["fast", "slow"] },
    exit: { cross_below: ["fast", "slow"] },
    sizing: { type: "fixed_fraction", fraction: 0.95 },
  },
  dataset_ref: SYMBOL,
  tolerances: { "*": { kind: "rel", value: 0.0001 } },
  property_checks: [{ kind: "no_nan" }],
};

// A short V-shaped price path so the fast/slow EMA cross fires at least once.
const CLOSES = [120, 118, 116, 114, 112, 110, 108, 112, 116, 120, 124, 128];

function candles() {
  return CLOSES.map((close, i) => {
    const open = i === 0 ? close : CLOSES[i - 1];
    return {
      time: 1_700_000_000 + i * 3600,
      open,
      high: Math.max(open, close) + 1,
      low: Math.min(open, close) - 1,
      close,
      volume: 1000,
    };
  });
}

function main() {
  const session = new Session();
  const data = { [SYMBOL]: candles() };
  console.log("wickra-strategy-ci", session.version());

  const blessed = JSON.parse(session.command(JSON.stringify({ cmd: "bless", test: TEST, data })));
  const result = JSON.parse(
    session.command(JSON.stringify({ cmd: "run_test", test: blessed, data })),
  );
  console.assert(result.passed, "a freshly-blessed test must pass");
  console.log(`blessed test: PASS (diff empty, ${result.property_results.length} checks)`);
}

main();
