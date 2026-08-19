using Nostr.Sdk;

NostrSdkMethods.InitLogger(LogLevel.Info);

using var client = new Client();
foreach (var value in new[] { "wss://relay.damus.io", "wss://nostr.wine" })
{
    using var relayUrl = RelayUrl.Parse(value);
    await client.AddRelay(relayUrl);
}
await client.Connect();

using var keys = Keys.Generate();
using var kind = Kind.FromStd(KindStandard.TextNote);
using var builder = new EventBuilder(kind, "Hello!");
using var textNote = builder.Finalize(keys);
await client.SendEvent(textNote);

Console.WriteLine("Mining a POW text note...");
using var powBuilder = new EventBuilder(kind, "Hello with POW!");
using var publicKey = keys.PublicKey();
using var unsignedEvent = powBuilder.FinalizeUnsigned(publicKey);
using var pow = new SingleThreadPow();
using var minedEvent = await unsignedEvent.MineAsync(pow, 20);
using var powEvent = minedEvent.Sign(keys);
var output = await client.SendEvent(powEvent);
Console.WriteLine("Event sent:");
Console.WriteLine($" hex:    {output.Id.ToHex()}");
Console.WriteLine($" bech32: {output.Id.ToBech32()}");
Console.WriteLine($" Successfully sent to: {string.Join(", ", output.Success.Select(url => url.ToString()))}");
Console.WriteLine($" Failed to send to: {string.Join(", ", output.Failed.Select(item => $"{item.Key}: {item.Value}"))}");

await Task.Delay(TimeSpan.FromSeconds(2));

Console.WriteLine("Getting events from relays...");
using var filter = new Filter().Author(publicKey);
using var target = ReqTarget.Auto(new[] { filter });
foreach (var nostrEvent in await client.FetchEvents(target, TimeSpan.FromSeconds(10)))
{
    using (nostrEvent)
    {
        Console.WriteLine(nostrEvent.AsJson());
    }
}
