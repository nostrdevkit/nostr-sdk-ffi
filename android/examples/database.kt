package org.nostrdevkit.examples

import org.nostrdevkit.sdk.*

suspend fun databaseExample(databasePath: String) {
    initLogger(LogLevel.INFO)

    val keys = Keys.parse("nsec1ufnus6pju578ste3v90xd5m2decpuzpql2295m3sknqcjzyys9ls0qlc85")
    println(keys.publicKey().toBech32())

    val client = ClientBuilder().database(NostrLmdb.open(databasePath)).build()
    println("Database backend: ${client.database().backend()}")

    client.addRelay(RelayUrl.parse("wss://relay.damus.io"))
    client.connect()
    client.sync(Filter().author(keys.publicKey()))

    val filter = Filter().author(keys.publicKey()).limit(10uL)
    for (event in client.database().query(filter)) {
        println(event.asJson())
    }
}
