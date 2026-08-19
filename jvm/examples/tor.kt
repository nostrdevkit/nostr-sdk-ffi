package org.nostrdevkit.examples

import org.nostrdevkit.sdk.*

private class TorProxy : CustomProxy {
    override fun custom(relayUrl: RelayUrl): SocketAddr? =
        if (relayUrl.isOnion()) SocketAddr.parse("127.0.0.1:9050") else null
}

suspend fun torExample() {
    initLogger(LogLevel.INFO)

    val keys = Keys.generate()
    println(keys.publicKey().toBech32())

    val client = ClientBuilder()
        .authenticator(SignerAuthenticator(keys))
        .proxy(Proxy.custom(TorProxy()))
        .build()
    for (url in listOf(
        "wss://relay.damus.io",
        "ws://oxtrdevav64z64yb7x6rjg4ntzqjhedm5b5zjqulugknhzr46ny2qbad.onion",
        "ws://2jsnlhfnelig5acq6iacydmzdbdmg7xwunm4xl6qwbvzacw4lwrjmlyd.onion",
    )) {
        client.addRelay(RelayUrl.parse(url))
    }
    client.connect()

    val event = EventBuilder(
        Kind.fromStd(KindStandard.TEXT_NOTE),
        "Hello from rust-nostr Kotlin bindings!",
    ).finalize(keys)
    val output = client.sendEvent(event)
    println("Event sent:")
    println(" hex:    ${output.id.toHex()}")
    println(" bech32: ${output.id.toBech32()}")
    println(" Successfully sent to: ${output.success}")
    println(" Failed to send to: ${output.failed}")
}
