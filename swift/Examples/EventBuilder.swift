import NostrSDK

func eventBuilderExample() throws -> [String] {
    let keys = Keys.generate()
    let kind = Kind.fromStd(e: .textNote)

    let textNote = try EventBuilder(
        kind: kind,
        content: "Note from rust-nostr Swift bindings"
    ).finalize(signer: keys)

    let customBuilder = EventBuilder(
        kind: Kind(kind: 1234),
        content: "My custom content"
    )
    let customEvent = try customBuilder.finalize(signer: keys)
    let powEvent = try customBuilder
        .finalizeUnsigned(publicKey: keys.publicKey())
        .mine(adapter: SingleThreadPow(), difficulty: 8)
        .sign(signer: keys)
    let unsignedEvent = customBuilder.finalizeUnsigned(publicKey: keys.publicKey())

    return [
        textNote.asJson(),
        customEvent.asJson(),
        powEvent.asJson(),
        unsignedEvent.asJson(),
    ]
}
