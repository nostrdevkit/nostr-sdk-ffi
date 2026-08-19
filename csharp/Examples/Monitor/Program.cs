using Nostr.Sdk;

NostrSdkMethods.InitLogger(LogLevel.Debug);

using var monitor = new Nostr.Sdk.Monitor();
using var clientBuilder = new ClientBuilder();
using var configuredBuilder = clientBuilder.Monitor(monitor);
using var client = configuredBuilder.Build();
foreach (var value in new[] {
    "wss://relay.damus.io",
    "wss://nostr.mom",
    "wss://nostr.oxtr.dev",
})
{
    using var relayUrl = RelayUrl.Parse(value);
    await client.AddRelay(relayUrl);
}
await client.Connect();
await monitor.HandleNotifications(new MonitorHandler());

sealed class MonitorHandler : HandleMonitorNotification
{
    public Task RelayStatusChanged(RelayUrl relayUrl, RelayStatus status)
    {
        Console.WriteLine($"Relay {relayUrl} status changed to {status}");
        return Task.CompletedTask;
    }
}
