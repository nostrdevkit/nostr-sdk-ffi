package org.nostrdevkit.examples

import org.nostrdevkit.sdk.*

private class MonitorHandler : HandleMonitorNotification {
    override suspend fun relayStatusChanged(relayUrl: RelayUrl, status: RelayStatus) {
        println("Relay $relayUrl status changed to $status")
    }
}

suspend fun monitorExample() {
    initLogger(LogLevel.DEBUG)

    val monitor = Monitor()
    val client = ClientBuilder().monitor(monitor).build()
    for (url in listOf("wss://relay.damus.io", "wss://nostr.mom", "wss://nostr.oxtr.dev")) {
        client.addRelay(RelayUrl.parse(url))
    }
    client.connect()
    monitor.handleNotifications(MonitorHandler())
}
