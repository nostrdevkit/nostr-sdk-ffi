package org.nostrdevkit.examples

import kotlinx.coroutines.delay
import org.nostrdevkit.sdk.*

suspend fun clientExample() {
    initLogger(LogLevel.INFO)

    val client = Client()
    for (url in listOf("wss://relay.damus.io", "wss://nostr.wine")) {
        client.addRelay(RelayUrl.parse(url))
    }
    client.connect()

    val keys = Keys.generate()
    val kind = Kind.fromStd(KindStandard.TEXT_NOTE)
    client.sendEvent(EventBuilder(kind, "Hello!").finalize(keys))

    println("Mining a POW text note...")
    val minedEvent = EventBuilder(kind, "Hello with POW!")
        .finalizeUnsigned(keys.publicKey())
        .mineAsync(SingleThreadPow(), 20u)
        .sign(keys)
    val output = client.sendEvent(minedEvent)
    println("Event sent:")
    println(" hex:    ${output.id.toHex()}")
    println(" bech32: ${output.id.toBech32()}")
    println(" Successfully sent to: ${output.success}")
    println(" Failed to send to: ${output.failed}")

    delay(2_000)

    println("Getting events from relays...")
    val filter = Filter().author(keys.publicKey())
    for (event in client.fetchEvents(ReqTarget.auto(listOf(filter)))) {
        println(event.asJson())
    }
}
