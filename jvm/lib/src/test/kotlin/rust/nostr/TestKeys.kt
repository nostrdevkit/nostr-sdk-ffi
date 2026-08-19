package org.nostrdevkit

import org.nostrdevkit.sdk.Keys
import org.nostrdevkit.sdk.SecretKey
import org.nostrdevkit.examples.keysExample
import kotlin.test.Test

class TestKeys {
    @Test
    fun testKeys() {
        val keys: Keys = Keys.generate()

        val secretKey: SecretKey = keys.secretKey()

        // Serialize secret key to hex
        val hex: String = secretKey.toHex()

        // Parse hex
        val parsedKeys = Keys.parse(hex)

        assert(keys == parsedKeys) {
            "Keys doesn't match"
        }
    }

    @Test
    fun testKeysExample() {
        assert(keysExample().startsWith("npub1"))
    }
}
