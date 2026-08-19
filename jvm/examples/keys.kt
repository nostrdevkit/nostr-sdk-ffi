package org.nostrdevkit.examples

import org.nostrdevkit.sdk.Keys

fun keysExample(): String {
    val keys = Keys.generate()
    return keys.publicKey().toBech32()
}

fun main() {
    println(keysExample())
}
