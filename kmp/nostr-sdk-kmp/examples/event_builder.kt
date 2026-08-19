package org.nostrdevkit.examples

import org.nostrdevkit.sdk.*

fun eventBuilderExample(): List<String> {
    val keys = Keys.generate()
    val kind = Kind.fromStd(KindStandard.TEXT_NOTE)

    val textNote = EventBuilder(kind, "Note from rust-nostr Kotlin bindings").finalize(keys)
    val customBuilder = EventBuilder(Kind(1234u), "My custom content")
    val customEvent = customBuilder.finalize(keys)
    val powEvent = customBuilder.finalizeUnsigned(keys.publicKey())
        .mine(SingleThreadPow(), 8u)
        .sign(keys)
    val unsignedEvent = customBuilder.finalizeUnsigned(keys.publicKey())

    return listOf(
        textNote.asJson(),
        customEvent.asJson(),
        powEvent.asJson(),
        unsignedEvent.asJson(),
    )
}
