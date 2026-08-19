import asyncio
from nostr_sdk import *


async def main():
    relay = LocalRelayBuilder().port(7676).build()

    await relay.run()

    print(f"Relay url: {await relay.url()}")

    while True:
        await asyncio.sleep(60)


if __name__ == '__main__':
    asyncio.run(main())
