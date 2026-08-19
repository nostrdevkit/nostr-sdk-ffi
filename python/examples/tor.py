import asyncio
from typing import Optional

from nostr_sdk import (
    ClientBuilder,
    CustomProxy,
    EventBuilder,
    Keys,
    Kind,
    KindStandard,
    LogLevel,
    Proxy,
    RelayUrl,
    SignerAuthenticator,
    SocketAddr,
    init_logger,
)


class TorProxy(CustomProxy):
    def custom(self, relay_url: RelayUrl) -> Optional[SocketAddr]:
        if relay_url.is_onion():
            return SocketAddr.parse("127.0.0.1:9050")
        return None


async def main():
    init_logger(LogLevel.INFO)

    keys = Keys.generate()
    print(keys.public_key().to_bech32())

    # Configure client to use a Tor proxy for `.onion` relays
    authenticator = SignerAuthenticator(keys)
    proxy = Proxy.custom(TorProxy())
    client = ClientBuilder().authenticator(authenticator).proxy(proxy).build()

    await client.add_relay(RelayUrl.parse("wss://relay.damus.io"))
    await client.add_relay(RelayUrl.parse("ws://oxtrdevav64z64yb7x6rjg4ntzqjhedm5b5zjqulugknhzr46ny2qbad.onion"))
    await client.add_relay(RelayUrl.parse("ws://2jsnlhfnelig5acq6iacydmzdbdmg7xwunm4xl6qwbvzacw4lwrjmlyd.onion"))
    await client.connect()

    event = EventBuilder(Kind.from_std(KindStandard.TEXT_NOTE), "Hello from rust-nostr Python bindings!").finalize(keys)
    output = await client.send_event(event)
    print("Event sent:")
    print(f" hex:    {output.id.to_hex()}")
    print(f" bech32: {output.id.to_bech32()}")
    print(f" Successfully sent to:    {output.success}")
    print(f" Failed to send to: {output.failed}")


if __name__ == '__main__':
    asyncio.run(main())
