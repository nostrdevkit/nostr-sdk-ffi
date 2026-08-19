import asyncio
from nostr_sdk import *
from nostr_sdk import uniffi_set_event_loop


async def main():
    uniffi_set_event_loop(asyncio.get_running_loop())  # type: ignore[arg-type]

    init_logger(LogLevel.INFO)

    keys = Keys.parse("nsec1ufnus6pju578ste3v90xd5m2decpuzpql2295m3sknqcjzyys9ls0qlc85")
    print(keys.public_key().to_bech32())

    # Create/open LMDB database
    lmdb = await NostrLmdb.open("nostr-lmdb")

    client = ClientBuilder().database(lmdb).build()

    database = client.database()
    print(f"Database backend: {database.backend()}")

    await client.add_relay(RelayUrl.parse("wss://relay.damus.io"))
    await client.connect()

    # Negentropy reconciliation
    f = Filter().author(keys.public_key())
    await client.sync(f)

    # Query events from database
    f = Filter().author(keys.public_key()).limit(10)
    events = await client.database().query(f)
    for event in events:
        print(event.as_json())

if __name__ == '__main__':
    asyncio.run(main())
