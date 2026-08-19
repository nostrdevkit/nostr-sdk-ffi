import NostrSDK

func keysExample() throws -> String {
    try Keys.generate().publicKey().toBech32()
}
