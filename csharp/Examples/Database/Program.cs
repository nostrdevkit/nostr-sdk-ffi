using Nostr.Sdk;

NostrSdkMethods.InitLogger(LogLevel.Info);

using var keys = Keys.Parse("nsec1ufnus6pju578ste3v90xd5m2decpuzpql2295m3sknqcjzyys9ls0qlc85");
using var publicKey = keys.PublicKey();
Console.WriteLine(publicKey.ToBech32());

using var lmdb = await NostrLmdb.Open("nostr-lmdb");
using var clientBuilder = new ClientBuilder();
using var configuredBuilder = clientBuilder.Database(lmdb);
using var client = configuredBuilder.Build();
Console.WriteLine($"Database backend: {client.Database().Backend()}");

using var relayUrl = RelayUrl.Parse("wss://relay.damus.io");
await client.AddRelay(relayUrl);
await client.Connect();

using var syncFilter = new Filter().Author(publicKey);
await client.Sync(syncFilter);

using var queryFilter = new Filter().Author(publicKey).Limit(10);
foreach (var nostrEvent in await client.Database().Query(queryFilter))
{
    using (nostrEvent)
    {
        Console.WriteLine(nostrEvent.AsJson());
    }
}
