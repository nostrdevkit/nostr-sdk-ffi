import Foundation
import NostrSDK

let keys = Keys.generate()
print(try keys.publicKey().toBech32())

let kind = Kind.fromStd(e: .textNote)
let builder = EventBuilder(kind: kind, content: "NostrSigner interface example")
let event = try builder.finalize(signer: keys)
print(event.id().toHex())

let asyncEvent = try await builder.finalizeAsync(signer: keys)
print(asyncEvent.id().toHex())

let authenticator: Authenticator = SignerAuthenticator(signer: keys)
let relayUrl = try RelayUrl.parse(url: "wss://relay.example.com")
let authEvent = try await authenticator.makeAuthEvent(
    relayUrl: relayUrl,
    challenge: "challenge"
)
print(authEvent?.id().toHex() ?? "")

let unsignedEvent = EventBuilder(
    kind: kind,
    content: "PowAdapter interface example"
).finalizeUnsigned(publicKey: keys.publicKey())
let singleThreadPow = SingleThreadPow()
let singleThreadEvent = try unsignedEvent.mine(adapter: singleThreadPow, difficulty: 1)
let singleThreadAsyncEvent = try await unsignedEvent.mineAsync(
    adapter: singleThreadPow,
    difficulty: 1
)
let multiThreadPow = MultiThreadPow()
let multiThreadEvent = try unsignedEvent.mine(adapter: multiThreadPow, difficulty: 1)
let multiThreadAsyncEvent = try await unsignedEvent.mineAsync(
    adapter: multiThreadPow,
    difficulty: 1
)
print(singleThreadEvent.id()?.toHex() ?? "")
print(singleThreadAsyncEvent.id()?.toHex() ?? "")
print(multiThreadEvent.id()?.toHex() ?? "")
print(multiThreadAsyncEvent.id()?.toHex() ?? "")

let databaseUrl = FileManager.default.temporaryDirectory
    .appendingPathComponent("nostr-sdk-ffi-\(UUID().uuidString)")
defer { try? FileManager.default.removeItem(at: databaseUrl) }

let database = try await NostrLmdb.open(path: databaseUrl.path)
let client = ClientBuilder().database(database: database).build()
print(client.database().backend())

private func asAsyncSigner(_ signer: NostrConnect) -> AsyncNostrSigner {
    signer
}
