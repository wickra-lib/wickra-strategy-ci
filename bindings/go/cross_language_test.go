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

func loadGoldenTests(t *testing.T) []json.RawMessage {
	t.Helper()
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
	return tests
}

func TestRunSuiteMatchesGolden(t *testing.T) {
	tests := loadGoldenTests(t)

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

// The batch path against the per-test path. run_suite fans the corpus out
// across rayon and sorts the results by id; run_test walks one test at a time.
// Those are two different engines reached through the same cgo boundary, and
// only the Rust core tested that they agree -- from a binding, the parallel path
// crossing the boundary is a separate claim. A regression here would show up as
// a suite that passes while an individual run of the same test does not.
func TestRunSuiteAgreesWithIndividualRuns(t *testing.T) {
	data := loadGoldenData(t)
	tests := loadGoldenTests(t)

	s := New()
	defer s.Close()

	suiteCmd, err := json.Marshal(map[string]any{
		"cmd": "run_suite", "tests": tests, "data": data,
	})
	if err != nil {
		t.Fatal(err)
	}
	suiteRaw, err := s.Command(string(suiteCmd))
	if err != nil {
		t.Fatal(err)
	}
	var suite struct {
		Results []json.RawMessage `json:"results"`
	}
	if err := json.Unmarshal([]byte(suiteRaw), &suite); err != nil {
		t.Fatal(err)
	}

	individual := make([]json.RawMessage, 0, len(tests))
	for _, test := range tests {
		cmd, err := json.Marshal(map[string]any{
			"cmd": "run_test", "test": test, "data": data,
		})
		if err != nil {
			t.Fatal(err)
		}
		one, err := s.Command(string(cmd))
		if err != nil {
			t.Fatal(err)
		}
		individual = append(individual, json.RawMessage(one))
	}
	// run_suite sorts by id; sort the per-test results the same way.
	sort.Slice(individual, func(i, j int) bool {
		return resultID(t, individual[i]) < resultID(t, individual[j])
	})

	if len(suite.Results) != len(individual) {
		t.Fatalf("suite has %d results, per-test has %d", len(suite.Results), len(individual))
	}
	for i := range suite.Results {
		if string(suite.Results[i]) != string(individual[i]) {
			t.Fatalf("result %d differs:\n batch: %s\n  each: %s",
				i, suite.Results[i], individual[i])
		}
	}
}

func resultID(t *testing.T, raw json.RawMessage) string {
	t.Helper()
	var r struct {
		ID string `json:"id"`
	}
	if err := json.Unmarshal(raw, &r); err != nil {
		t.Fatal(err)
	}
	return r.ID
}
