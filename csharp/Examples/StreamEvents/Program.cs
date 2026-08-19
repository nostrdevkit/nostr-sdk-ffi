using Nostr.Sdk;

NostrSdkMethods.InitLogger(LogLevel.Info);

using var client = new Client();
foreach (var value in new[] { "wss://relay.damus.io", "wss://nos.lol" })
{
    using var relayUrl = RelayUrl.Parse(value);
    await client.AddRelay(relayUrl);
}
await client.Connect();

Console.WriteLine("Streaming events from relays...");
using var kind = new Kind(0);
using var filter = new Filter().Kind(kind).Limit(5);
using var target = ReqTarget.Auto(new[] { filter });
using var stream = await client.StreamEvents(target, timeout: TimeSpan.FromSeconds(10));
while (await stream.Next() is { } item)
{
    using (item)
    {
        if (item.Event is not null)
        {
            Console.WriteLine(item.Event.AsJson());
        }
        else if (item.Error is not null)
        {
            Console.WriteLine($"Relay error from {item.RelayUrl}: {item.Error}");
        }
    }
}
Console.WriteLine("Stream terminated.");
