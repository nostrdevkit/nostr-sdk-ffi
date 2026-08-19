package rust.nostr

import kotlinx.coroutines.runBlocking
import org.nostrdevkit.sdk.*
import java.nio.file.Files
import kotlin.test.Test
import kotlin.test.assertNotNull
import kotlin.test.assertTrue

class TestInterfaces {
    @Test
    fun testRustImplementationsCrossInterfaceBoundaries() = runBlocking {
        val keys = Keys.generate()
        val kind = Kind.fromStd(KindStandard.TEXT_NOTE)

        val event = EventBuilder(kind, "sync signer").finalize(keys)
        val asyncEvent = EventBuilder(kind, "async signer").finalizeAsync(keys)
        assertTrue(event.verifyId())
        assertTrue(asyncEvent.verifyId())

        val authenticator: Authenticator = SignerAuthenticator(keys)
        val authEvent = authenticator.makeAuthEvent(
            RelayUrl.parse("wss://relay.example.com"),
            "challenge",
        )
        assertTrue(authEvent?.verifyId() == true)

        val unsignedEvent = EventBuilder(kind, "proof of work")
            .finalizeUnsigned(keys.publicKey())
        val pow = SingleThreadPow()
        assertNotNull(unsignedEvent.mine(pow, 1u).id())
        assertNotNull(unsignedEvent.mineAsync(pow, 1u).id())
        Unit
    }

    @Test
    fun testDatabaseImplementationCrossesInterfaceBoundary() = runBlocking {
        val databasePath = Files.createTempDirectory("nostr-sdk-ffi-")
        try {
            val database: NostrDatabase = NostrLmdb.open(databasePath.toString())
            val client = ClientBuilder().database(database).build()
            assertTrue(client.database().backend().isNotEmpty())
        } finally {
            databasePath.toFile().deleteRecursively()
        }
    }
}
