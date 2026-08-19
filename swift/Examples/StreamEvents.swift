import NostrSDK

func streamEventsExample() async throws {
    initLogger(level: .info)

    let client = Client()
    try await client.addRelay(url: RelayUrl.parse(url: "wss://relay.damus.io"))
    try await client.addRelay(url: RelayUrl.parse(url: "wss://nos.lol"))
    await client.connect()

    print("Streaming events from relays...")
    let filter = Filter().kind(kind: Kind(kind: 0)).limit(limit: 5)
    let stream = try await client.streamEvents(target: ReqTarget.auto(filters: [filter]))
    while let item = try await stream.next() {
        if let event = item.event {
            print(event.asJson())
        } else if let error = item.error {
            print("Relay error from \(item.relayUrl): \(error)")
        }
    }
    print("Stream terminated.")
}
