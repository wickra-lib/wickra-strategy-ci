package wickra

// Cross-language golden: build the run_suite command from the committed
// golden/{tests,data} corpus, run it through the binding, and assert the
// response equals golden/expected/suite.json byte-for-byte — the exact
// SuiteResult the Rust core and every other binding produce.

import (
	"encoding/json"
	"os"
	"path/filepath"
	"sort"
	"strconv"
	"strings"
	"testing"
)

func goldenDir() string {
	return filepath.Join("..", "..", "golden")
}

func loadGoldenData(t *testing.T) map[string][]map[string]float64 {
	t.Helper()
	dir := filepath.Join(goldenDir(), "data")
	entries, err := os.ReadDir(dir)
	if err != nil {
		t.Fatal(err)
	}
	data := map[string][]map[string]float64{}
	for _, e := range entries {
		if !strings.HasSuffix(e.Name(), ".csv") {
			continue
		}
		raw, err := os.ReadFile(filepath.Join(dir, e.Name()))
		if err != nil {
			t.Fatal(err)
		}
		var rows []map[string]float64
		for idx, line := range strings.Split(string(raw), "\n") {
			line = strings.TrimSpace(line)
			if line == "" {
				continue
			}
			cols := strings.Split(line, ",")
			ts, err := strconv.ParseInt(strings.TrimSpace(cols[0]), 10, 64)
			if err != nil {
				if idx == 0 {
					continue // header
				}
				t.Fatalf("bad ts %q", cols[0])
			}
			f := func(i int) float64 { v, _ := strconv.ParseFloat(strings.TrimSpace(cols[i]), 64); return v }
			rows = append(rows, map[string]float64{
				"time": float64(ts), "open": f(1), "high": f(2), "low": f(3), "close": f(4), "volume": f(5),
			})
		}
		data[strings.TrimSuffix(e.Name(), ".csv")] = rows
	}
	return data
}

func TestRunSuiteMatchesGolden(t *testing.T) {
	testsDir := filepath.Join(goldenDir(), "tests")
	entries, err := os.ReadDir(testsDir)
	if err != nil {
		t.Fatal(err)
	}
	var names []string
	for _, e := range entries {
		if strings.HasSuffix(e.Name(), ".json") {
			names = append(names, e.Name())
		}
	}
	sort.Strings(names)
	tests := make([]json.RawMessage, 0, len(names))
	for _, name := range names {
		raw, err := os.ReadFile(filepath.Join(testsDir, name))
		if err != nil {
			t.Fatal(err)
		}
		tests = append(tests, json.RawMessage(raw))
	}

	cmd, err := json.Marshal(map[string]any{
		"cmd":   "run_suite",
		"tests": tests,
		"data":  loadGoldenData(t),
	})
	if err != nil {
		t.Fatal(err)
	}

	s := New()
	defer s.Close()
	got, err := s.Command(string(cmd))
	if err != nil {
		t.Fatal(err)
	}
	wantRaw, err := os.ReadFile(filepath.Join(goldenDir(), "expected", "suite.json"))
	if err != nil {
		t.Fatal(err)
	}
	want := strings.TrimSpace(string(wantRaw))
	if got != want {
		t.Fatalf("SuiteResult mismatch:\n got: %s\nwant: %s", got, want)
	}
}
