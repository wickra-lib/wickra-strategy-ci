package wickra

import (
	"encoding/json"
	"math"
	"strings"
	"testing"
)

const strategy = `{"symbol":"TEST","timeframe":"1h",` +
	`"indicators":{"fast":{"type":"Sma","params":[2]},"slow":{"type":"Sma","params":[5]}},` +
	`"entry":{"cross_above":["fast","slow"]},"exit":{"cross_below":["fast","slow"]},` +
	`"sizing":{"type":"fixed_fraction","fraction":1.0}}`

func candles() []map[string]float64 {
	out := make([]map[string]float64, 0, 40)
	for i := 0; i < 40; i++ {
		px := 100.0 + 10.0*math.Sin(float64(i)*0.5)
		out = append(out, map[string]float64{
			"time": float64(i), "open": px,
			"high": px + 1, "low": px - 1, "close": px, "volume": 100,
		})
	}
	return out
}

func runTestCmd() string {
	test := map[string]any{
		"id":              "momentum",
		"strategy":        json.RawMessage(strategy),
		"dataset_ref":     "sym-01",
		"property_checks": []map[string]any{{"kind": "no_nan"}},
	}
	cmd, _ := json.Marshal(map[string]any{
		"cmd":  "run_test",
		"test": test,
		"data": map[string]any{"sym-01": candles()},
	})
	return string(cmd)
}

func TestVersion(t *testing.T) {
	if Version() == "" {
		t.Fatal("empty version")
	}
}

func TestRunTestPasses(t *testing.T) {
	s := New()
	defer s.Close()
	raw, err := s.Command(runTestCmd())
	if err != nil {
		t.Fatal(err)
	}
	var result struct {
		ID     string            `json:"id"`
		Passed bool              `json:"passed"`
		Diff   []json.RawMessage `json:"diff"`
	}
	if err := json.Unmarshal([]byte(raw), &result); err != nil {
		t.Fatal(err)
	}
	if result.ID != "momentum" {
		t.Fatalf("expected id momentum, got %s", raw)
	}
	if !result.Passed {
		t.Fatalf("expected the freshly-blessed test to pass, got %s", raw)
	}
	if len(result.Diff) != 0 {
		t.Fatalf("expected an empty diff, got %s", raw)
	}
}

func TestUnknownCommandIsInBandError(t *testing.T) {
	s := New()
	defer s.Close()
	raw, err := s.Command(`{"cmd":"nope"}`)
	if err != nil {
		t.Fatalf("unexpected hard error: %v", err)
	}
	if !strings.Contains(raw, `"ok":false`) {
		t.Fatalf("expected an in-band error, got: %s", raw)
	}
}
