import Foundation
import NostrSDK

func clientExample() async throws {
    initLogger(level: .info)

    let client = Client()
    for value in ["wss://relay.damus.io", "wss://nostr.wine"] {
        _ = try await client.addRelay(url: RelayUrl.parse(url: value))
    }
    await client.connect()

    let keys = Keys.generate()
    let kind = Kind.fromStd(e: .textNote)
    let event = try EventBuilder(kind: kind, content: "Hello!").finalize(signer: keys)
    _ = try await client.sendEvent(event: event)

    print("Mining a POW text note...")
    let unsignedEvent = EventBuilder(kind: kind, content: "Hello with POW!")
        .finalizeUnsigned(publicKey: keys.publicKey())
    let minedEvent = try await unsignedEvent.mineAsync(
        adapter: SingleThreadPow(),
        difficulty: 20
    )
    let output = try await client.sendEvent(event: minedEvent.sign(signer: keys))
    print("Event sent:")
    print(" hex:    \(output.id.toHex())")
    print(" bech32: \(try output.id.toBech32())")
    print(" Successfully sent to: \(output.success)")
    print(" Failed to send to: \(output.failed)")

    try await Task.sleep(nanoseconds: 2_000_000_000)

    print("Getting events from relays...")
    let filter = Filter().author(author: keys.publicKey())
    let events = try await client.fetchEvents(target: ReqTarget.auto(filters: [filter]))
    for receivedEvent in events {
        print(try receivedEvent.asJson())
    }
}
