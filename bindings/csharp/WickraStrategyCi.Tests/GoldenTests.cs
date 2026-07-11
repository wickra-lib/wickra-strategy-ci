using System.Text.Json;
using Wickra.StrategyCi;
using Xunit;

namespace WickraStrategyCi.Tests;

// The cross-language golden invariant seen from C#: the same command yields
// byte-identical output across calls, and a blessed golden re-matches itself. The
// response bytes are what every other binding produces too, because the whole
// test runner lives once in the Rust core and this binding forwards its JSON
// verbatim.
public class GoldenTests
{
    [Fact]
    public void RunTest_IsByteIdenticalAcrossCalls()
    {
        using var a = new Session();
        using var b = new Session();
        Assert.Equal(a.Command(SessionTests.RunTestCmd()), b.Command(SessionTests.RunTestCmd()));
    }

    [Fact]
    public void Bless_ThenReRun_MatchesWithEmptyDiff()
    {
        using var session = new Session();
        JsonElement blessed = JsonDocument.Parse(session.Command(SessionTests.BlessCmd())).RootElement;
        Assert.True(blessed.TryGetProperty("expected", out _));

        string rerunCmd = JsonSerializer.Serialize(new
        {
            cmd = "run_test",
            test = blessed,
            data = JsonSerializer.Deserialize<JsonElement>(SessionTests.BlessCmd()).GetProperty("data"),
        });
        JsonElement rerun = JsonDocument.Parse(session.Command(rerunCmd)).RootElement;
        Assert.True(rerun.GetProperty("passed").GetBoolean());
        Assert.Empty(rerun.GetProperty("diff").EnumerateArray());
    }
}
