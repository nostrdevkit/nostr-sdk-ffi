using Nostr.Sdk;

NostrSdkMethods.InitLogger(LogLevel.Info);

using var keys = Keys.Generate();
using var publicKey = keys.PublicKey();
Console.WriteLine(publicKey.ToBech32());

using var authenticator = new SignerAuthenticator(keys);
using var proxy = Proxy.Custom(new TorProxy());
using var clientBuilder = new ClientBuilder();
using var authBuilder = clientBuilder.Authenticator(authenticator);
using var proxyBuilder = authBuilder.Proxy(proxy);
using var client = proxyBuilder.Build();
foreach (var value in new[] {
    "wss://relay.damus.io",
    "ws://oxtrdevav64z64yb7x6rjg4ntzqjhedm5b5zjqulugknhzr46ny2qbad.onion",
    "ws://2jsnlhfnelig5acq6iacydmzdbdmg7xwunm4xl6qwbvzacw4lwrjmlyd.onion",
})
{
    using var relayUrl = RelayUrl.Parse(value);
    await client.AddRelay(relayUrl);
}
await client.Connect();

using var kind = Kind.FromStd(KindStandard.TextNote);
using var builder = new EventBuilder(kind, "Hello from rust-nostr C# bindings!");
using var nostrEvent = builder.Finalize(keys);
var output = await client.SendEvent(nostrEvent);
Console.WriteLine("Event sent:");
Console.WriteLine($" hex:    {output.Id.ToHex()}");
Console.WriteLine($" bech32: {output.Id.ToBech32()}");
Console.WriteLine($" Successfully sent to: {string.Join(", ", output.Success.Select(url => url.ToString()))}");
Console.WriteLine($" Failed to send to: {string.Join(", ", output.Failed.Select(item => $"{item.Key}: {item.Value}"))}");

sealed class TorProxy : CustomProxy
{
    public SocketAddr? Custom(RelayUrl relayUrl) =>
        relayUrl.IsOnion() ? SocketAddr.Parse("127.0.0.1:9050") : null;
}
