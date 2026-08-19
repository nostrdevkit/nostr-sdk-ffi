package org.nostrdevkit.examples

import org.nostrdevkit.sdk.*

suspend fun streamEventsExample() {
    initLogger(LogLevel.INFO)

    val client = Client()
    client.addRelay(RelayUrl.parse("wss://relay.damus.io"))
    client.addRelay(RelayUrl.parse("wss://nos.lol"))
    client.connect()

    println("Streaming events from relays...")
    val filter = Filter().kind(Kind(0u)).limit(5uL)
    val stream = client.streamEvents(ReqTarget.auto(listOf(filter)))
    while (true) {
        val item = stream.next() ?: break
        when {
            item.event != null -> println(item.event?.asJson())
            item.error != null -> println("Relay error from ${item.relayUrl}: ${item.error}")
        }
    }
    println("Stream terminated.")
}
