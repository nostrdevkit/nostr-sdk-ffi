import Foundation
import NostrSDK

protocol ExampleWebSocketConnection: AnyObject, Sendable {
    func send(_ message: WebSocketMessage) async throws
    func receive() async throws -> WebSocketMessage?
    func close() async throws
}

private final class ExampleWebSocketAdapter: WebSocketAdapter, @unchecked Sendable {
    private let connection: ExampleWebSocketConnection

    init(connection: ExampleWebSocketConnection) {
        self.connection = connection
    }

    func send(msg: WebSocketMessage) async throws {
        try await connection.send(msg)
    }

    func recv() async throws -> WebSocketMessage? {
        try await connection.receive()
    }

    func closeConnection() async throws {
        try await connection.close()
    }
}

private final class ExampleWebSocketTransport: CustomWebSocketTransport, @unchecked Sendable {
    private let connectHandler: @Sendable (String) async throws -> ExampleWebSocketConnection

    init(connect: @escaping @Sendable (String) async throws -> ExampleWebSocketConnection) {
        connectHandler = connect
    }

    func supportPing() -> Bool {
        false
    }

    func connect(url: String, proxy: SocketAddr?) async throws -> WebSocketAdapterWrapper? {
        guard proxy == nil else {
            throw ExampleWebSocketError.proxyNotSupported
        }
        return WebSocketAdapterWrapper(
            adapter: ExampleWebSocketAdapter(connection: try await connectHandler(url))
        )
    }
}

private enum ExampleWebSocketError: Error {
    case proxyNotSupported
}

func customWebSocketClientExample(
    connect: @escaping @Sendable (String) async throws -> ExampleWebSocketConnection
) async throws {
    initLogger(level: .trace)

    let keys = try Keys.parse(secretKey: "nsec1ufnus6pju578ste3v90xd5m2decpuzpql2295m3sknqcjzyys9ls0qlc85")
    let client = ClientBuilder()
        .authenticator(authenticator: SignerAuthenticator(signer: keys))
        .websocketTransport(transport: ExampleWebSocketTransport(connect: connect))
        .build()
    try await client.addRelay(url: RelayUrl.parse(url: "ws://127.0.0.1:7777"))
    await client.connect()

    let event = try EventBuilder(
        kind: Kind.fromStd(e: .textNote),
        content: "Test from nostrdevkit Swift bindings!"
    ).finalize(signer: keys)
    let output = try await client.sendEvent(event: event)
    print("Event sent: \(try output.id.toBech32())")

    try await Task.sleep(nanoseconds: 2_000_000_000)
    let filter = Filter().author(author: keys.publicKey())
    for receivedEvent in try await client.fetchEvents(
        target: ReqTarget.auto(filters: [filter])
    ) {
        print(receivedEvent.asPrettyJson())
    }
}
