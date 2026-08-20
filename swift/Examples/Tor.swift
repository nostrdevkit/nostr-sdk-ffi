import NostrSDK

private final class TorProxy: CustomProxy {
    func custom(relayUrl: RelayUrl) -> SocketAddr? {
        relayUrl.isOnion() ? try? SocketAddr.parse(addr: "127.0.0.1:9050") : nil
    }
}

func torExample() async throws {
    initLogger(level: .info)

    let keys = Keys.generate()
    print(try keys.publicKey().toBech32())

    let client = ClientBuilder()
        .authenticator(authenticator: SignerAuthenticator(signer: keys))
        .proxy(proxy: Proxy.custom(custom: TorProxy()))
        .build()
    for value in [
        "wss://relay.damus.io",
        "ws://oxtrdevav64z64yb7x6rjg4ntzqjhedm5b5zjqulugknhzr46ny2qbad.onion",
        "ws://2jsnlhfnelig5acq6iacydmzdbdmg7xwunm4xl6qwbvzacw4lwrjmlyd.onion",
    ] {
        _ = try await client.addRelay(url: RelayUrl.parse(url: value))
    }
    await client.connect()

    let event = try EventBuilder(
        kind: Kind.fromStd(e: .textNote),
        content: "Hello from rust-nostr Swift bindings!"
    ).finalize(signer: keys)
    let output = try await client.sendEvent(event: event)
    print("Event sent:")
    print(" hex:    \(output.id.toHex())")
    print(" bech32: \(try output.id.toBech32())")
    print(" Successfully sent to: \(output.success)")
    print(" Failed to send to: \(output.failed)")
}
