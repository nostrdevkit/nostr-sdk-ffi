using Nostr.Sdk;

_ = (Func<NostrConnect, AsyncNostrSigner>)AsAsyncSigner;

using var keys = Keys.Generate();
using var kind = Kind.FromStd(KindStandard.TextNote);
using var builder = new EventBuilder(kind, "NostrSigner interface example");
using var nostrEvent = builder.Finalize(keys);
using var eventId = nostrEvent.Id();

Console.WriteLine(eventId.ToHex());

using var asyncBuilder = new EventBuilder(kind, "AsyncNostrSigner interface example");
using var asyncEvent = await asyncBuilder.FinalizeAsync(keys);
using var asyncEventId = asyncEvent.Id();

Console.WriteLine(asyncEventId.ToHex());

using var signerAuthenticator = new SignerAuthenticator(keys);
Authenticator authenticator = signerAuthenticator;
using var relayUrl = RelayUrl.Parse("wss://relay.example.com");
using var authEvent = await authenticator.MakeAuthEvent(relayUrl, "challenge")
    ?? throw new InvalidOperationException("The authenticator returned no event");
using var authEventId = authEvent.Id();

Console.WriteLine(authEventId.ToHex());

using var publicKey = keys.PublicKey();
using var powBuilder = new EventBuilder(kind, "PowAdapter interface example");
using var unsignedEvent = powBuilder.FinalizeUnsigned(publicKey);
using var singleThreadPow = new SingleThreadPow();
using var singleThreadEvent = unsignedEvent.Mine(singleThreadPow, 1);
using var singleThreadAsyncEvent = await unsignedEvent.MineAsync(singleThreadPow, 1);
using var multiThreadPow = new MultiThreadPow();
using var multiThreadEvent = unsignedEvent.Mine(multiThreadPow, 1);
using var multiThreadAsyncEvent = await unsignedEvent.MineAsync(multiThreadPow, 1);

var databasePath = Path.Combine(Path.GetTempPath(), $"nostr-sdk-ffi-{Guid.NewGuid()}");

try
{
    using var database = await NostrLmdb.Open(databasePath);
    using var clientBuilder = new ClientBuilder();
    using var configuredBuilder = clientBuilder.Database(database);
    using var client = configuredBuilder.Build();

    Console.WriteLine(client.Database().Backend());
}
finally
{
    if (Directory.Exists(databasePath))
    {
        Directory.Delete(databasePath, recursive: true);
    }
}

static AsyncNostrSigner AsAsyncSigner(NostrConnect signer) => signer;
