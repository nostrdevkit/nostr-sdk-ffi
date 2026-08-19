import asyncio

from datetime import timedelta
from typing import Optional

from aiohttp import ClientSession, ClientWebSocketResponse, WSMsgType
from nostr_sdk import *
from nostr_sdk import uniffi_set_event_loop


class MyAdapter(WebSocketAdapter):
    def __init__(self, session: ClientSession, ws: ClientWebSocketResponse):
        self.session = session
        self.websocket = ws

    async def send(self, msg: WebSocketMessage):
        if isinstance(msg, WebSocketMessage.TEXT):
            await self.websocket.send_str(msg.text)
        elif isinstance(msg, WebSocketMessage.BINARY):
            await self.websocket.send_bytes(msg.bytes)
        elif isinstance(msg, WebSocketMessage.PING):
            await self.websocket.ping(msg.bytes)
        elif isinstance(msg, WebSocketMessage.PONG):
            await self.websocket.pong(msg.bytes)
        elif isinstance(msg, WebSocketMessage.CLOSE):
            await self.websocket.close()

    async def recv(self) -> Optional[WebSocketMessage]:
        raw_msg = await self.websocket.receive()

        if raw_msg.type == WSMsgType.TEXT:
            return WebSocketMessage.TEXT(raw_msg.data)  # type: ignore[return-value]
        if raw_msg.type == WSMsgType.BINARY:
            return WebSocketMessage.BINARY(raw_msg.data)  # type: ignore[return-value]
        if raw_msg.type == WSMsgType.PING:
            return WebSocketMessage.PING(raw_msg.data)  # type: ignore[return-value]
        if raw_msg.type == WSMsgType.PONG:
            return WebSocketMessage.PONG(raw_msg.data)  # type: ignore[return-value]
        return None

    async def close_connection(self):
        await self.websocket.close()
        await self.session.close()

class MyWebSocketClient(CustomWebSocketTransport):
    def support_ping(self) -> bool:
        return False

    async def connect(self, url: str, proxy: Optional[SocketAddr]) -> Optional[WebSocketAdapterWrapper]:
        session = ClientSession()
        ws = await session.ws_connect(url)
        return WebSocketAdapterWrapper(MyAdapter(session, ws))


async def main():
    uniffi_set_event_loop(asyncio.get_running_loop())  # type: ignore[arg-type]

    # Init logger
    init_logger(LogLevel.TRACE)

    # Initialize client without signer
    # client = Client()

    # Or, initialize with Keys signer
    keys = Keys.parse("nsec1ufnus6pju578ste3v90xd5m2decpuzpql2295m3sknqcjzyys9ls0qlc85")
    authenticator = SignerAuthenticator(keys)
    client = ClientBuilder().authenticator(authenticator).websocket_transport(MyWebSocketClient()).build()
    #client = ClientBuilder().authenticator(authenticator).build()

    # Add relays and connect
    await client.add_relay(RelayUrl.parse("ws://127.0.0.1:7777"))
    await client.connect()

    # Send an event using the Nostr Signer
    event = EventBuilder(Kind.from_std(KindStandard.TEXT_NOTE), "Test from nostrdevkit Python bindings!").finalize(keys)
    output = await client.send_event(event)

    print("Event sent:")
    print(f" hex:    {output.id.to_hex()}")
    print(f" bech32: {output.id.to_bech32()}")
    print(f" Successfully sent to:    {output.success}")
    print(f" Failed to send to: {output.failed}")

    await asyncio.sleep(2.0)

    # Get events from relays
    print("Getting events from relays...")
    f = Filter().authors([keys.public_key()])
    events = await client.fetch_events(ReqTarget.auto([f]), timedelta(seconds=10))
    for event in events:
        print(event.as_pretty_json())


if __name__ == '__main__':
    asyncio.run(main())
