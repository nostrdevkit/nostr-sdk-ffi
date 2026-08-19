using Nostr.Sdk;

NostrSdkMethods.InitLogger(LogLevel.Info);

using var muted = PublicKey.Parse("npub1l2vyh47mk2p0qlsku7hg0vn29faehy9hy34ygaclpn66ukqp3afqutajft");
using var other = PublicKey.Parse("npub1xtscya34g58tk0z605fvr788k263gsu6cy9x0mhnm87echrgufzsevkk5s");
var filtering = new Filtering();
filtering.Mute(muted);

using var clientBuilder = new ClientBuilder();
using var configuredBuilder = clientBuilder.AdmitPolicy(filtering);
using var client = configuredBuilder.Build();
using var relayUrl = RelayUrl.Parse("wss://relay.damus.io");
await client.AddRelay(relayUrl);
await client.Connect();

using var kind = new Kind(0);
using var filter = new Filter().Authors(new[] { muted, other }).Kind(kind);
using var target = ReqTarget.Auto(new[] { filter });
var events = await client.FetchEvents(target, TimeSpan.FromSeconds(10));
Console.WriteLine($"Received {events.Length} events");

sealed class Filtering : AdmitPolicy
{
    private readonly HashSet<string> mutedPublicKeys = new();

    public void Mute(PublicKey publicKey) => mutedPublicKeys.Add(publicKey.ToHex());

    public Task<AdmitStatus?> AdmitConnection(RelayUrl relayUrl) =>
        Task.FromResult<AdmitStatus?>(AdmitStatus.Success());

    public Task<AdmitStatus?> AdmitEvent(
        RelayUrl relayUrl,
        string subscriptionId,
        Event nostrEvent) => Task.FromResult<AdmitStatus?>(
            mutedPublicKeys.Contains(nostrEvent.Author().ToHex())
                ? AdmitStatus.Rejected()
                : AdmitStatus.Success());
}
