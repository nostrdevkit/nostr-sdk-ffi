package org.nostrdevkit.examples

import org.nostrdevkit.sdk.*

private class WebOfTrust : AdmitPolicy {
    private val allowedPublicKeys = mutableSetOf<PublicKey>()

    fun allow(publicKey: PublicKey) {
        allowedPublicKeys += publicKey
    }

    override suspend fun admitConnection(relayUrl: RelayUrl): AdmitStatus = AdmitStatus.success()

    override suspend fun admitEvent(
        relayUrl: RelayUrl,
        subscriptionId: String,
        event: Event,
    ): AdmitStatus = if (event.author() in allowedPublicKeys) {
        AdmitStatus.success()
    } else {
        AdmitStatus.rejected()
    }
}

suspend fun whitelistExample() {
    initLogger(LogLevel.INFO)

    val allowed = PublicKey.parse("npub1l2vyh47mk2p0qlsku7hg0vn29faehy9hy34ygaclpn66ukqp3afqutajft")
    val other = PublicKey.parse("npub1xtscya34g58tk0z605fvr788k263gsu6cy9x0mhnm87echrgufzsevkk5s")
    val webOfTrust = WebOfTrust().apply { allow(allowed) }
    val client = ClientBuilder().admitPolicy(webOfTrust).build()

    client.addRelay(RelayUrl.parse("wss://relay.damus.io"))
    client.connect()

    val filter = Filter().authors(listOf(allowed, other)).kind(Kind(0u))
    val events = client.fetchEvents(ReqTarget.auto(listOf(filter)))
    println("Received ${events.size} events")
}
