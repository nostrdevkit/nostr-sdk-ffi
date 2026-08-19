package org.nostrdevkit.examples

import org.nostrdevkit.sdk.*

private class Filtering : AdmitPolicy {
    private val mutedPublicKeys = mutableSetOf<PublicKey>()

    fun mute(publicKey: PublicKey) {
        mutedPublicKeys += publicKey
    }

    override suspend fun admitConnection(relayUrl: RelayUrl): AdmitStatus = AdmitStatus.success()

    override suspend fun admitEvent(
        relayUrl: RelayUrl,
        subscriptionId: String,
        event: Event,
    ): AdmitStatus = if (event.author() in mutedPublicKeys) {
        AdmitStatus.rejected()
    } else {
        AdmitStatus.success()
    }
}

suspend fun blacklistExample() {
    initLogger(LogLevel.INFO)

    val muted = PublicKey.parse("npub1l2vyh47mk2p0qlsku7hg0vn29faehy9hy34ygaclpn66ukqp3afqutajft")
    val other = PublicKey.parse("npub1xtscya34g58tk0z605fvr788k263gsu6cy9x0mhnm87echrgufzsevkk5s")
    val filtering = Filtering().apply { mute(muted) }
    val client = ClientBuilder().admitPolicy(filtering).build()

    client.addRelay(RelayUrl.parse("wss://relay.damus.io"))
    client.connect()

    val filter = Filter().authors(listOf(muted, other)).kind(Kind(0u))
    val events = client.fetchEvents(ReqTarget.auto(listOf(filter)))
    println("Received ${events.size} events")
}
