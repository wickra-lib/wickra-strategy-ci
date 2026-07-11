package wickra

// The cross-language golden invariant seen from Go: the same command yields
// byte-identical output across calls, and a blessed golden re-matches itself.
// The response bytes are what every other binding produces too, because the
// whole test runner lives once in the Rust core and this binding forwards its
// JSON verbatim.

import (
	"encoding/json"
	"testing"
)

func TestRunTestByteIdenticalAcrossCalls(t *testing.T) {
	a := New()
	defer a.Close()
	b := New()
	defer b.Close()

	ra, err := a.Command(runTestCmd())
	if err != nil {
		t.Fatal(err)
	}
	rb, err := b.Command(runTestCmd())
	if err != nil {
		t.Fatal(err)
	}
	if ra != rb {
		t.Fatalf("expected byte-identical output, got:\n a: %s\n b: %s", ra, rb)
	}
}

func TestBlessThenMatch(t *testing.T) {
	s := New()
	defer s.Close()

	blessCmd, _ := json.Marshal(map[string]any{
		"cmd": "bless",
		"test": map[string]any{
			"id":              "momentum",
			"strategy":        json.RawMessage(strategy),
			"dataset_ref":     "sym-01",
			"property_checks": []map[string]any{{"kind": "no_nan"}},
		},
		"data": map[string]any{"sym-01": candles()},
	})
	blessedRaw, err := s.Command(string(blessCmd))
	if err != nil {
		t.Fatal(err)
	}
	var blessed map[string]json.RawMessage
	if err := json.Unmarshal([]byte(blessedRaw), &blessed); err != nil {
		t.Fatal(err)
	}
	if _, ok := blessed["expected"]; !ok {
		t.Fatalf("expected a blessed test with an expected report, got %s", blessedRaw)
	}

	rerunCmd, _ := json.Marshal(map[string]any{
		"cmd":  "run_test",
		"test": json.RawMessage(blessedRaw),
		"data": map[string]any{"sym-01": candles()},
	})
	rerunRaw, err := s.Command(string(rerunCmd))
	if err != nil {
		t.Fatal(err)
	}
	var rerun struct {
		Passed bool              `json:"passed"`
		Diff   []json.RawMessage `json:"diff"`
	}
	if err := json.Unmarshal([]byte(rerunRaw), &rerun); err != nil {
		t.Fatal(err)
	}
	if !rerun.Passed || len(rerun.Diff) != 0 {
		t.Fatalf("expected a passing re-run with an empty diff, got %s", rerunRaw)
	}
}
