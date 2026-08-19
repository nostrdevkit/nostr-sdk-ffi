package org.nostrdevkit.examples

import org.nostrdevkit.sdk.*

private class InMemoryDatabase : NostrDatabase {
    private val events = mutableMapOf<EventId, Event>()

    override fun backend(): String = "my-in-memory-backend"

    override fun features() = NostrDatabaseFeatures(
        persistent = false,
        eventExpiration = false,
        fullTextSearch = false,
        requestToVanish = false,
    )

    override suspend fun saveEvent(event: Event): SaveEventStatus {
        events[event.id()] = event
        return SaveEventStatus.success()
    }

    override suspend fun checkId(eventId: EventId): DatabaseEventStatus =
        if (eventId in events) DatabaseEventStatus.SAVED else DatabaseEventStatus.NOT_EXISTENT

    override suspend fun eventById(eventId: EventId): Event? = events[eventId]

    override suspend fun count(filters: Filter): ULong = events.size.toULong()

    override suspend fun query(filter: Filter): List<Event> = events.values.take(10)

    override suspend fun deleteEvents(filter: Filter) = events.clear()

    override suspend fun wipe() = events.clear()
}

suspend fun customDatabaseExample() {
    initLogger(LogLevel.INFO)

    val client = ClientBuilder().database(InMemoryDatabase()).build()
    client.addRelay(RelayUrl.parse("wss://relay.damus.io"))
    client.connect()

    val keys = Keys.parse("nsec1ufnus6pju578ste3v90xd5m2decpuzpql2295m3sknqcjzyys9ls0qlc85")
    println(keys.publicKey().toBech32())
    client.sync(Filter().author(keys.publicKey()))

    val events = client.database().query(Filter().author(keys.publicKey()).limit(10uL))
    if (events.isEmpty()) {
        println("Query did not find any event")
    } else {
        events.forEach { println(it.asJson()) }
    }
}
