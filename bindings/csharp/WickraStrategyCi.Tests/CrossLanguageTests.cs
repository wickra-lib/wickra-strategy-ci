using System.Text.Json;
using Wickra.StrategyCi;
using Xunit;

namespace WickraStrategyCi.Tests;

// Cross-language golden: build the run_suite command from the committed
// golden/{tests,data} corpus, run it through the binding, and assert the response
// equals golden/expected/suite.json byte-for-byte — the exact SuiteResult the
// Rust core and every other binding produce.
public class CrossLanguageTests
{
    private static string GoldenDir()
    {
        string? dir = AppContext.BaseDirectory;
        for (int i = 0; i < 12 && dir is not null; i++)
        {
            string g = Path.Combine(dir, "golden");
            if (Directory.Exists(Path.Combine(g, "tests")))
            {
                return g;
            }
            dir = Path.GetDirectoryName(dir);
        }
        throw new DirectoryNotFoundException("golden/ not found");
    }

    private static Dictionary<string, List<Dictionary<string, double>>> LoadData(string golden)
    {
        var data = new Dictionary<string, List<Dictionary<string, double>>>();
        foreach (string csv in Directory.EnumerateFiles(Path.Combine(golden, "data"), "*.csv").OrderBy(p => p))
        {
            var rows = new List<Dictionary<string, double>>();
            string[] lines = File.ReadAllLines(csv);
            for (int idx = 0; idx < lines.Length; idx++)
            {
                string line = lines[idx].Trim();
                if (line.Length == 0)
                {
                    continue;
                }
                string[] c = line.Split(',');
                if (!long.TryParse(c[0].Trim(), out long t))
                {
                    continue; // header
                }
                double F(int i) => double.Parse(c[i].Trim());
                rows.Add(new Dictionary<string, double>
                {
                    ["time"] = t, ["open"] = F(1), ["high"] = F(2), ["low"] = F(3), ["close"] = F(4), ["volume"] = F(5),
                });
            }
            data[Path.GetFileNameWithoutExtension(csv)] = rows;
        }
        return data;
    }

    [Fact]
    public void RunSuite_MatchesGolden()
    {
        string golden = GoldenDir();
        var tests = Directory
            .EnumerateFiles(Path.Combine(golden, "tests"), "*.json")
            .OrderBy(p => p)
            .Select(p => JsonSerializer.Deserialize<JsonElement>(File.ReadAllText(p)))
            .ToList();

        string cmd = JsonSerializer.Serialize(new { cmd = "run_suite", tests, data = LoadData(golden) });
        using var session = new Session();
        string got = session.Command(cmd);
        string want = File.ReadAllText(Path.Combine(golden, "expected", "suite.json")).Trim();

        Assert.Equal(want, got);
    }

    // The batch path against the per-test path. run_suite fans the corpus out
    // across rayon and sorts the results by id; run_test walks one test at a
    // time. Those are two different engines reached through the same P/Invoke
    // boundary, and only the Rust core tested that they agree -- from a binding,
    // the parallel path crossing the boundary is a separate claim. A regression
    // here would show up as a suite that passes while an individual run of the
    // same test does not.
    [Fact]
    public void RunSuite_AgreesWithIndividualRuns()
    {
        string golden = GoldenDir();
        var data = LoadData(golden);
        var tests = Directory
            .EnumerateFiles(Path.Combine(golden, "tests"), "*.json")
            .OrderBy(p => p)
            .Select(p => JsonSerializer.Deserialize<JsonElement>(File.ReadAllText(p)))
            .ToList();

        using var session = new Session();

        string suiteRaw = session.Command(
            JsonSerializer.Serialize(new { cmd = "run_suite", tests, data }));
        var batch = JsonSerializer.Deserialize<JsonElement>(suiteRaw)
            .GetProperty("results")
            .EnumerateArray()
            .Select(r => r.GetRawText())
            .ToList();

        var individual = tests
            .Select(t => session.Command(
                JsonSerializer.Serialize(new { cmd = "run_test", test = t, data })))
            .Select(r => JsonSerializer.Deserialize<JsonElement>(r))
            .OrderBy(r => r.GetProperty("id").GetString(), StringComparer.Ordinal)
            .Select(r => r.GetRawText())
            .ToList();

        Assert.Equal(batch, individual);
    }
}
