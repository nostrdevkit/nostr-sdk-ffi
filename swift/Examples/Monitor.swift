import NostrSDK

private final class MonitorHandler: HandleMonitorNotification {
    func relayStatusChanged(relayUrl: RelayUrl, status: RelayStatus) async {
        print("Relay \(relayUrl) status changed to \(status)")
    }
}

func monitorExample() async throws {
    initLogger(level: .debug)

    let monitor = Monitor()
    let client = ClientBuilder().monitor(monitor: monitor).build()
    for value in [
        "wss://relay.damus.io",
        "wss://nostr.mom",
        "wss://nostr.oxtr.dev",
    ] {
        _ = try await client.addRelay(url: RelayUrl.parse(url: value))
    }
    await client.connect()
    try await monitor.handleNotifications(handler: MonitorHandler())
}
