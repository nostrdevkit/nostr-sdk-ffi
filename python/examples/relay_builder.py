import asyncio
from typing import cast

from nostr_sdk import *


class RejectEmptyEvents(WritePolicy):
    async def admit_event(self, event: Event, socket_addr: str) -> WritePolicyResult:
        if event.content():
            return cast(WritePolicyResult, WritePolicyResult.ACCEPT())
        return cast(WritePolicyResult, WritePolicyResult.REJECT(message="empty content"))


class LocalQueriesOnly(QueryPolicy):
    async def admit_query(self, query: Filter, socket_addr: str) -> QueryPolicyResult:
        if socket_addr.startswith("127.0.0.1:") or socket_addr.startswith("[::1]:"):
            return cast(QueryPolicyResult, QueryPolicyResult.ACCEPT())
        return cast(QueryPolicyResult, QueryPolicyResult.REJECT(message="local queries only"))


async def main():
    relay = (
        LocalRelayBuilder()
        .port(7676)
        .write_policy(RejectEmptyEvents())
        .query_policy(LocalQueriesOnly())
        .build()
    )

    await relay.run()

    print(f"Relay url: {await relay.url()}")

    while True:
        await asyncio.sleep(60)


if __name__ == '__main__':
    asyncio.run(main())
