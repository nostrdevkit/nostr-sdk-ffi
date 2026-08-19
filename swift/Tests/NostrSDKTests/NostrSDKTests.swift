import Foundation
import XCTest
@testable import NostrSDK

final class NostrSDKTests: XCTestCase {
    func testKeysImplementSigner() throws {
        let keys = Keys.generate()
        let builder = EventBuilder(
            kind: Kind.fromStd(e: .textNote),
            content: "NostrSigner interface test"
        )

        let event = try builder.finalize(signer: keys)
        XCTAssertTrue(event.verifyId())
    }

    func testKeysImplementAsyncSigner() async throws {
        let keys = Keys.generate()
        let builder = EventBuilder(
            kind: Kind.fromStd(e: .textNote),
            content: "AsyncNostrSigner interface test"
        )

        let event = try await builder.finalizeAsync(signer: keys)
        XCTAssertTrue(event.verifyId())
    }

    func testSignerAuthenticatorImplementsAuthenticator() async throws {
        let keys = Keys.generate()
        let authenticator: Authenticator = SignerAuthenticator(signer: keys)
        let relayUrl = try RelayUrl.parse(url: "wss://relay.example.com")

        let event = try await authenticator.makeAuthEvent(
            relayUrl: relayUrl,
            challenge: "challenge"
        )

        XCTAssertTrue(event?.verifyId() == true)
    }

    func testPowImplementationsImplementAdapters() async throws {
        let keys = Keys.generate()
        let unsignedEvent = EventBuilder(
            kind: Kind.fromStd(e: .textNote),
            content: "PowAdapter interface test"
        ).finalizeUnsigned(publicKey: keys.publicKey())

        let singleThreadPow = SingleThreadPow()
        let singleThreadEvent = try unsignedEvent.mine(
            adapter: singleThreadPow,
            difficulty: 1
        )
        let singleThreadAsyncEvent = try await unsignedEvent.mineAsync(
            adapter: singleThreadPow,
            difficulty: 1
        )
        let multiThreadPow = MultiThreadPow()
        let multiThreadEvent = try unsignedEvent.mine(
            adapter: multiThreadPow,
            difficulty: 1
        )
        let multiThreadAsyncEvent = try await unsignedEvent.mineAsync(
            adapter: multiThreadPow,
            difficulty: 1
        )

        XCTAssertNotNil(singleThreadEvent.id())
        XCTAssertNotNil(singleThreadAsyncEvent.id())
        XCTAssertNotNil(multiThreadEvent.id())
        XCTAssertNotNil(multiThreadAsyncEvent.id())
    }

    func testNostrLmdbImplementsDatabase() async throws {
        let databaseUrl = FileManager.default.temporaryDirectory
            .appendingPathComponent("nostr-sdk-ffi-\(UUID().uuidString)")
        defer { try? FileManager.default.removeItem(at: databaseUrl) }

        let database = try await NostrLmdb.open(path: databaseUrl.path)
        let client = ClientBuilder().database(database: database).build()

        XCTAssertFalse(client.database().backend().isEmpty)
    }
}

private func asAsyncSigner(_ signer: NostrConnect) -> AsyncNostrSigner {
    signer
}
