package org.nostrdevkit.examples

import kotlinx.coroutines.awaitCancellation
import org.nostrdevkit.sdk.*

private class RejectEmptyEvents : WritePolicy {
    override suspend fun admitEvent(event: Event, socketAddr: String): WritePolicyResult =
        if (event.content().isNotEmpty()) {
            WritePolicyResult.Accept
        } else {
            WritePolicyResult.Reject("empty content")
        }
}

private class LocalQueriesOnly : QueryPolicy {
    override suspend fun admitQuery(query: Filter, socketAddr: String): QueryPolicyResult =
        if (socketAddr.startsWith("127.0.0.1:") || socketAddr.startsWith("[::1]:")) {
            QueryPolicyResult.Accept
        } else {
            QueryPolicyResult.Reject("local queries only")
        }
}

suspend fun relayBuilderExample() {
    val relay = LocalRelayBuilder()
        .port(7676u)
        .writePolicy(RejectEmptyEvents())
        .queryPolicy(LocalQueriesOnly())
        .build()

    relay.run()
    println("Relay URL: ${relay.url()}")
    awaitCancellation()
}
