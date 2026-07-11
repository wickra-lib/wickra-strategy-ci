// A runnable C# example: golden-pin a strategy with `bless`, then re-run it and
// confirm it passes. The report is recomputed by the engine, so the golden is an
// exact, reproducible anchor.
using System.Text.Json;
using Wickra.StrategyCi;

const string symbol = "AAA";

object Strategy() => new
{
    symbol,
    timeframe = "1h",
    indicators = new
    {
        fast = new { type = "Ema", @params = new[] { 3 } },
        slow = new { type = "Ema", @params = new[] { 8 } },
    },
    entry = new { cross_above = new[] { "fast", "slow" } },
    exit = new { cross_below = new[] { "fast", "slow" } },
    sizing = new { type = "fixed_fraction", fraction = 0.95 },
};

int[] closes = { 120, 118, 116, 114, 112, 110, 108, 112, 116, 120, 124, 128 };
object[] Candles()
{
    var list = new List<object>();
    for (int i = 0; i < closes.Length; i++)
    {
        int open = i == 0 ? closes[i] : closes[i - 1];
        int close = closes[i];
        list.Add(new { time = 1_700_000_000 + i * 3600, open, high = Math.Max(open, close) + 1, low = Math.Min(open, close) - 1, close, volume = 1000 });
    }
    return list.ToArray();
}

var data = new Dictionary<string, object[]> { [symbol] = Candles() };
using var session = new Session();
Console.WriteLine($"wickra-strategy-ci {Session.Version()}");

var test = new { id = "ema_crossover", strategy = Strategy(), dataset_ref = symbol, property_checks = new[] { new { kind = "no_nan" } } };
var blessed = JsonSerializer.Deserialize<JsonElement>(
    session.Command(JsonSerializer.Serialize(new { cmd = "bless", test, data })));

var rerun = JsonSerializer.Deserialize<JsonElement>(
    session.Command(JsonSerializer.Serialize(new { cmd = "run_test", test = blessed, data })));

bool passed = rerun.GetProperty("passed").GetBoolean();
if (!passed)
{
    Console.Error.WriteLine("a freshly-blessed test must pass");
    Environment.Exit(1);
}
Console.WriteLine($"blessed test: PASS (diff empty, {rerun.GetProperty("property_results").GetArrayLength()} checks)");
