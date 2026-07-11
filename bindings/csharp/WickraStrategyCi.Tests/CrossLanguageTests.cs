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
}
