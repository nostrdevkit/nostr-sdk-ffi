import NostrSDK

private final class Filtering: AdmitPolicy, @unchecked Sendable {
    private var mutedPublicKeys: Set<String> = []

    func mute(_ publicKey: PublicKey) {
        mutedPublicKeys.insert(publicKey.toHex())
    }

    func admitConnection(relayUrl: RelayUrl) async throws -> AdmitStatus? {
        AdmitStatus.success()
    }

    func admitEvent(
        relayUrl: RelayUrl,
        subscriptionId: String,
        event: Event
    ) async throws -> AdmitStatus? {
        mutedPublicKeys.contains(event.author().toHex())
            ? AdmitStatus.rejected()
            : AdmitStatus.success()
    }
}

func blacklistExample() async throws {
    initLogger(level: .info)

    let muted = try PublicKey.parse(
        publicKey: "npub1l2vyh47mk2p0qlsku7hg0vn29faehy9hy34ygaclpn66ukqp3afqutajft"
    )
    let other = try PublicKey.parse(
        publicKey: "npub1xtscya34g58tk0z605fvr788k263gsu6cy9x0mhnm87echrgufzsevkk5s"
    )
    let filtering = Filtering()
    filtering.mute(muted)
    let client = ClientBuilder().admitPolicy(policy: filtering).build()

    try await client.addRelay(url: RelayUrl.parse(url: "wss://relay.damus.io"))
    await client.connect()
    let filter = Filter().authors(authors: [muted, other]).kind(kind: Kind(kind: 0))
    let events = try await client.fetchEvents(target: ReqTarget.auto(filters: [filter]))
    print("Received \(events.count) events")
}
