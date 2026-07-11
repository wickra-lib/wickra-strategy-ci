// A runnable Go example: golden-pin a strategy with `bless`, then re-run it and
// confirm it passes. The report is recomputed by the engine, so the golden is an
// exact, reproducible anchor.
//
//	cargo build --release -p wickra-strategy-ci-c
//	# stage the library under bindings/go/lib/<goos>_<goarch>/ (CI does this)
//	cd examples/go && go run .
package main

import (
	"encoding/json"
	"fmt"
	"math"

	wickra "github.com/wickra-lib/wickra-strategy-ci/bindings/go"
)

const symbol = "AAA"

const strategy = `{"symbol":"AAA","timeframe":"1h",` +
	`"indicators":{"fast":{"type":"Ema","params":[3]},"slow":{"type":"Ema","params":[8]}},` +
	`"entry":{"cross_above":["fast","slow"]},"exit":{"cross_below":["fast","slow"]},` +
	`"sizing":{"type":"fixed_fraction","fraction":0.95}}`

// A short V-shaped price path so the fast/slow EMA cross fires at least once.
var closes = []float64{120, 118, 116, 114, 112, 110, 108, 112, 116, 120, 124, 128}

func candles() []map[string]float64 {
	out := make([]map[string]float64, 0, len(closes))
	for i, c := range closes {
		o := c
		if i > 0 {
			o = closes[i-1]
		}
		out = append(out, map[string]float64{
			"time": float64(1700000000 + i*3600), "open": o,
			"high": math.Max(o, c) + 1, "low": math.Min(o, c) - 1, "close": c, "volume": 1000,
		})
	}
	return out
}

func main() {
	s := wickra.New()
	defer s.Close()

	test := map[string]any{
		"id": "ema_crossover", "strategy": json.RawMessage(strategy),
		"dataset_ref": symbol, "tolerances": map[string]any{"*": map[string]any{"kind": "rel", "value": 0.0001}},
		"property_checks": []map[string]any{{"kind": "no_nan"}},
	}
	data := map[string]any{symbol: candles()}

	fmt.Println("wickra-strategy-ci", wickra.Version())

	blessCmd, _ := json.Marshal(map[string]any{"cmd": "bless", "test": test, "data": data})
	blessedRaw, err := s.Command(string(blessCmd))
	if err != nil {
		panic(err)
	}
	rerunCmd, _ := json.Marshal(map[string]any{"cmd": "run_test", "test": json.RawMessage(blessedRaw), "data": data})
	rerunRaw, err := s.Command(string(rerunCmd))
	if err != nil {
		panic(err)
	}
	var result struct {
		Passed         bool              `json:"passed"`
		PropertyResult []json.RawMessage `json:"property_results"`
	}
	if err := json.Unmarshal([]byte(rerunRaw), &result); err != nil {
		panic(err)
	}
	if !result.Passed {
		panic("a freshly-blessed test must pass: " + rerunRaw)
	}
	fmt.Printf("blessed test: PASS (diff empty, %d checks)\n", len(result.PropertyResult))
}
