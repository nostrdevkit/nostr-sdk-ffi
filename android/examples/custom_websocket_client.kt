package org.nostrdevkit.examples

import kotlinx.coroutines.delay
import org.nostrdevkit.sdk.*

interface ExampleWebSocketConnection {
    suspend fun send(message: WebSocketMessage)
    suspend fun receive(): WebSocketMessage?
    suspend fun close()
}

private class ExampleWebSocketAdapter(
    private val connection: ExampleWebSocketConnection,
) : WebSocketAdapter {
    override suspend fun send(msg: WebSocketMessage) = connection.send(msg)
    override suspend fun recv(): WebSocketMessage? = connection.receive()
    override suspend fun closeConnection() = connection.close()
}

private class ExampleWebSocketTransport(
    private val connect: suspend (String) -> ExampleWebSocketConnection,
) : CustomWebSocketTransport {
    override fun supportPing(): Boolean = false

    override suspend fun connect(url: String, proxy: SocketAddr?): WebSocketAdapterWrapper? {
        require(proxy == null) { "This example transport does not support proxies" }
        return WebSocketAdapterWrapper(ExampleWebSocketAdapter(connect(url)))
    }
}

suspend fun customWebSocketClientExample(
    connect: suspend (String) -> ExampleWebSocketConnection,
) {
    initLogger(LogLevel.TRACE)

    val keys = Keys.parse("nsec1ufnus6pju578ste3v90xd5m2decpuzpql2295m3sknqcjzyys9ls0qlc85")
    val client = ClientBuilder()
        .authenticator(SignerAuthenticator(keys))
        .websocketTransport(ExampleWebSocketTransport(connect))
        .build()
    client.addRelay(RelayUrl.parse("ws://127.0.0.1:7777"))
    client.connect()

    val event = EventBuilder(
        Kind.fromStd(KindStandard.TEXT_NOTE),
        "Test from nostrdevkit Kotlin bindings!",
    ).finalize(keys)
    val output = client.sendEvent(event)
    println("Event sent: ${output.id.toBech32()}")

    delay(2_000)
    val filter = Filter().author(keys.publicKey())
    for (receivedEvent in client.fetchEvents(ReqTarget.auto(listOf(filter)))) {
        println(receivedEvent.asPrettyJson())
    }
}
