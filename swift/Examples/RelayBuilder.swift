import Foundation
import NostrSDK

private final class RejectEmptyEvents: WritePolicy {
    func admitEvent(event: Event, socketAddr: String) async -> WritePolicyResult {
        event.content().isEmpty ? .reject(message: "empty content") : .accept
    }
}

private final class LocalQueriesOnly: QueryPolicy {
    func admitQuery(query: Filter, socketAddr: String) async -> QueryPolicyResult {
        let isLocal = socketAddr.hasPrefix("127.0.0.1:") || socketAddr.hasPrefix("[::1]:")
        return isLocal ? .accept : .reject(message: "local queries only")
    }
}

func relayBuilderExample() async throws {
    let relay = LocalRelayBuilder()
        .port(port: 7676)
        .writePolicy(policy: RejectEmptyEvents())
        .queryPolicy(policy: LocalQueriesOnly())
        .build()

    try await relay.run()
    print("Relay URL: \(await relay.url())")
    try await Task.sleep(nanoseconds: UInt64.max)
}
