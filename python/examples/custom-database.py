import asyncio
from nostr_sdk import *
from nostr_sdk import uniffi_set_event_loop
from typing import Dict, List, Optional


async def main():
    init_logger(LogLevel.INFO)

    uniffi_set_event_loop(asyncio.get_running_loop())  # type: ignore[arg-type]

    # Example of custom in-memory database
    class MyDatabase(NostrDatabase):
        def __init__(self):
            self.events: Dict[EventId, Event] = {}

        def backend(self) -> str:
            return "my-in-memory-backend"

        def features(self) -> NostrDatabaseFeatures:
            return NostrDatabaseFeatures(
                persistent=False,
                event_expiration=False,
                full_text_search=False,
                request_to_vanish=False,
            )

        async def save_event(self, event: Event) -> Optional[SaveEventStatus]:
            self.events[event.id()] = event
            return SaveEventStatus.success()

        async def check_id(self, event_id: EventId) -> DatabaseEventStatus:
            if event_id in self.events:
                return DatabaseEventStatus.SAVED
            else:
                return DatabaseEventStatus.NOT_EXISTENT

        async def event_by_id(self, event_id: EventId) -> Optional[Event]:
            return self.events.get(event_id, None)

        async def count(self, filters: Filter) -> int:
            return len(self.events)

        async def query(self, filter: Filter) -> List[Event]:
            # Fake algorithm
            return list(self.events.values())[:10]

        async def delete_events(self, filter: Filter) -> None:
            self.events.clear()

        async def wipe(self) -> None:
            self.events.clear()

    my_db = MyDatabase()
    client = ClientBuilder().database(my_db).build()

    await client.add_relay(RelayUrl.parse("wss://relay.damus.io"))
    await client.connect()

    keys = Keys.parse("nsec1ufnus6pju578ste3v90xd5m2decpuzpql2295m3sknqcjzyys9ls0qlc85")
    print(keys.public_key().to_bech32())

    # Negentropy reconciliation
    f = Filter().author(keys.public_key())
    opts = SyncOptions()
    await client.sync(f, opts)

    # Query events from database
    f = Filter().author(keys.public_key()).limit(10)
    events = await client.database().query(f)
    if len(events) == 0:
        print("Query not found any event")
    else:
        for event in events:
            print(event.as_json())


if __name__ == '__main__':
    asyncio.run(main())
