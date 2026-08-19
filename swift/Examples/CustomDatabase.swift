import NostrSDK

private final class InMemoryDatabase: NostrDatabase, @unchecked Sendable {
    private var events: [String: Event] = [:]

    func backend() -> String {
        "my-in-memory-backend"
    }

    func features() -> NostrDatabaseFeatures {
        NostrDatabaseFeatures(
            persistent: false,
            eventExpiration: false,
            fullTextSearch: false,
            requestToVanish: false
        )
    }

    func saveEvent(event: Event) async throws -> SaveEventStatus? {
        events[event.id().toHex()] = event
        return SaveEventStatus.success()
    }

    func checkId(eventId: EventId) async throws -> DatabaseEventStatus {
        events[eventId.toHex()] == nil ? .notExistent : .saved
    }

    func eventById(eventId: EventId) async throws -> Event? {
        events[eventId.toHex()]
    }

    func count(filter: Filter) async throws -> UInt64 {
        UInt64(events.count)
    }

    func query(filter: Filter) async throws -> [Event] {
        Array(events.values.prefix(10))
    }

    func deleteEvents(filter: Filter) async throws {
        events.removeAll()
    }

    func wipe() async throws {
        events.removeAll()
    }
}

func customDatabaseExample() async throws {
    initLogger(level: .info)

    let client = ClientBuilder().database(database: InMemoryDatabase()).build()
    try await client.addRelay(url: RelayUrl.parse(url: "wss://relay.damus.io"))
    await client.connect()

    let keys = try Keys.parse(secretKey: "nsec1ufnus6pju578ste3v90xd5m2decpuzpql2295m3sknqcjzyys9ls0qlc85")
    print(try keys.publicKey().toBech32())
    _ = try await client.sync(filter: Filter().author(author: keys.publicKey()))

    let filter = Filter().author(author: keys.publicKey()).limit(limit: 10)
    let events = try await client.database().query(filter: filter)
    if events.isEmpty {
        print("Query did not find any event")
    } else {
        events.forEach { print($0.asJson()) }
    }
}
