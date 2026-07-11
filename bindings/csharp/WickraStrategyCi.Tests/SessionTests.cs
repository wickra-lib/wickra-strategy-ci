using System.Text.Json;
using Wickra.StrategyCi;
using Xunit;

namespace WickraStrategyCi.Tests;

public class SessionTests
{
    private static object Strategy() => new
    {
        symbol = "TEST",
        timeframe = "1h",
        indicators = new
        {
            fast = new { type = "Sma", @params = new[] { 2 } },
            slow = new { type = "Sma", @params = new[] { 5 } },
        },
        entry = new { cross_above = new[] { "fast", "slow" } },
        exit = new { cross_below = new[] { "fast", "slow" } },
        sizing = new { type = "fixed_fraction", fraction = 1.0 },
    };

    private static object[] Candles()
    {
        var list = new List<object>();
        for (int i = 0; i < 40; i++)
        {
            double px = 100.0 + 10.0 * Math.Sin(i * 0.5);
            list.Add(new { time = i, open = px, high = px + 1, low = px - 1, close = px, volume = 100 });
        }
        return [.. list];
    }

    internal static string RunTestCmd() => JsonSerializer.Serialize(new
    {
        cmd = "run_test",
        test = new
        {
            id = "momentum",
            strategy = Strategy(),
            dataset_ref = "sym-01",
            property_checks = new[] { new { kind = "no_nan" } },
        },
        data = new Dictionary<string, object[]> { ["sym-01"] = Candles() },
    });

    internal static string BlessCmd() => JsonSerializer.Serialize(new
    {
        cmd = "bless",
        test = new
        {
            id = "momentum",
            strategy = Strategy(),
            dataset_ref = "sym-01",
            property_checks = new[] { new { kind = "no_nan" } },
        },
        data = new Dictionary<string, object[]> { ["sym-01"] = Candles() },
    });

    [Fact]
    public void Version_IsNonEmpty()
    {
        Assert.False(string.IsNullOrEmpty(Session.Version()));
    }

    [Fact]
    public void RunTest_Passes()
    {
        using var session = new Session();
        JsonElement result = JsonDocument.Parse(session.Command(RunTestCmd())).RootElement;
        Assert.Equal("momentum", result.GetProperty("id").GetString());
        Assert.True(result.GetProperty("passed").GetBoolean());
        Assert.Empty(result.GetProperty("diff").EnumerateArray());
    }

    [Fact]
    public void UnknownCommand_IsInBandError()
    {
        using var session = new Session();
        // An unknown command is not a hard error: the ABI returns a length and the
        // error surfaces in-band as {"ok":false,...} JSON.
        string raw = session.Command("{\"cmd\":\"nope\"}");
        Assert.Contains("\"ok\":false", raw);
    }
}
