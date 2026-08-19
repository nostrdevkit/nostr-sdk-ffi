using Nostr.Sdk;

NostrSdkMethods.InitLogger(LogLevel.Info);

using var clientBuilder = new ClientBuilder();
using var configuredBuilder = clientBuilder.Database(new InMemoryDatabase());
using var client = configuredBuilder.Build();
using var relayUrl = RelayUrl.Parse("wss://relay.damus.io");
await client.AddRelay(relayUrl);
await client.Connect();

using var keys = Keys.Parse("nsec1ufnus6pju578ste3v90xd5m2decpuzpql2295m3sknqcjzyys9ls0qlc85");
using var publicKey = keys.PublicKey();
Console.WriteLine(publicKey.ToBech32());

using var syncFilter = new Filter().Author(publicKey);
await client.Sync(syncFilter);

using var queryFilter = new Filter().Author(publicKey).Limit(10);
var events = await client.Database().Query(queryFilter);
if (events.Length == 0)
{
    Console.WriteLine("Query did not find any event");
}
else
{
    foreach (var nostrEvent in events)
    {
        using (nostrEvent)
        {
            Console.WriteLine(nostrEvent.AsJson());
        }
    }
}

sealed class InMemoryDatabase : NostrDatabase
{
    private readonly Dictionary<string, Event> events = new();

    public string Backend() => "my-in-memory-backend";

    public NostrDatabaseFeatures Features() => new(
        Persistent: false,
        EventExpiration: false,
        FullTextSearch: false,
        RequestToVanish: false);

    public Task<SaveEventStatus?> SaveEvent(Event nostrEvent)
    {
        events[nostrEvent.Id().ToHex()] = nostrEvent;
        return Task.FromResult<SaveEventStatus?>(SaveEventStatus.Success());
    }

    public Task<DatabaseEventStatus> CheckId(EventId eventId) => Task.FromResult(
        events.ContainsKey(eventId.ToHex())
            ? DatabaseEventStatus.Saved
            : DatabaseEventStatus.NotExistent);

    public Task<Event?> EventById(EventId eventId)
    {
        events.TryGetValue(eventId.ToHex(), out var nostrEvent);
        return Task.FromResult(nostrEvent);
    }

    public Task<ulong> Count(Filter filter) => Task.FromResult((ulong)events.Count);

    public Task<Event[]> Query(Filter filter) =>
        Task.FromResult(events.Values.Take(10).ToArray());

    public Task DeleteEvents(Filter filter)
    {
        events.Clear();
        return Task.CompletedTask;
    }

    public Task Wipe()
    {
        events.Clear();
        return Task.CompletedTask;
    }
}
