import NostrSDK

private final class WebOfTrust: AdmitPolicy, @unchecked Sendable {
    private var allowedPublicKeys: Set<String> = []

    func allow(_ publicKey: PublicKey) {
        allowedPublicKeys.insert(publicKey.toHex())
    }

    func admitConnection(relayUrl: RelayUrl) async throws -> AdmitStatus? {
        AdmitStatus.success()
    }

    func admitEvent(
        relayUrl: RelayUrl,
        subscriptionId: String,
        event: Event
    ) async throws -> AdmitStatus? {
        allowedPublicKeys.contains(event.author().toHex())
            ? AdmitStatus.success()
            : AdmitStatus.rejected()
    }
}

func whitelistExample() async throws {
    initLogger(level: .info)

    let allowed = try PublicKey.parse(
        publicKey: "npub1l2vyh47mk2p0qlsku7hg0vn29faehy9hy34ygaclpn66ukqp3afqutajft"
    )
    let other = try PublicKey.parse(
        publicKey: "npub1xtscya34g58tk0z605fvr788k263gsu6cy9x0mhnm87echrgufzsevkk5s"
    )
    let webOfTrust = WebOfTrust()
    webOfTrust.allow(allowed)
    let client = ClientBuilder().admitPolicy(policy: webOfTrust).build()

    _ = try await client.addRelay(url: RelayUrl.parse(url: "wss://relay.damus.io"))
    await client.connect()
    let filter = Filter().authors(authors: [allowed, other]).kind(kind: Kind(kind: 0))
    let events = try await client.fetchEvents(target: ReqTarget.auto(filters: [filter]))
    print("Received \(events.count) events")
}
