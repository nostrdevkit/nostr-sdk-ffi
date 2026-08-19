import asyncio
from datetime import timedelta
from nostr_sdk import *


async def main():
    # Init logger
    init_logger(LogLevel.INFO)

    client = Client()

    # Add relays and connect
    url = RelayUrl.parse("wss://relay.damus.io")
    await client.add_relay(url)

    url = RelayUrl.parse("wss://nos.lol")
    await client.add_relay(url)

    await client.connect()

    print("Streaming events from relays...")

    k = Kind(0)
    f = Filter().kind(k).limit(5)

    stream = await client.stream_events(ReqTarget.auto([f]), timeout=timedelta(seconds=10))

    while True:
        item = await stream.next()

        # Check if the stream is terminated
        if item is None:
            break

        if item.event is not None:
            print(item.event.as_json())
        elif item.error is not None:
            print(f"Relay error from {item.relay_url}: {item.error}")


    print("Stream terminated.")

if __name__ == '__main__':
    asyncio.run(main())
