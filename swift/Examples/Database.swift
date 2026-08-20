import NostrSDK

func databaseExample(path: String) async throws {
    initLogger(level: .info)

    let keys = try Keys.parse(secretKey: "nsec1ufnus6pju578ste3v90xd5m2decpuzpql2295m3sknqcjzyys9ls0qlc85")
    print(try keys.publicKey().toBech32())

    let database = try await NostrLmdb.open(path: path)
    let client = ClientBuilder().database(database: database).build()
    print("Database backend: \(client.database().backend())")

    _ = try await client.addRelay(url: RelayUrl.parse(url: "wss://relay.damus.io"))
    await client.connect()
    _ = try await client.sync(filter: Filter().author(author: keys.publicKey()))

    let filter = Filter().author(author: keys.publicKey()).limit(limit: 10)
    for event in try await client.database().query(filter: filter) {
        print(try event.asJson())
    }
}
