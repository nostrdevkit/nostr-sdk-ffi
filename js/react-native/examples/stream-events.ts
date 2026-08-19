import {
  Client,
  Filter,
  Kind,
  LogLevel,
  RelayUrl,
  ReqTarget,
  initLogger,
} from "../src/generated/nostr_sdk";

export async function main(): Promise<void> {
  initLogger(LogLevel.Info);

  const client = new Client();
  await client.addRelay(
    RelayUrl.parse("wss://relay.damus.io"), undefined, false, undefined,
  );
  await client.addRelay(RelayUrl.parse("wss://nos.lol"), undefined, false, undefined);
  await client.connect(undefined);

  const filter = new Filter().kind(new Kind(0)).limit(5n);
  const stream = await client.streamEvents(
    ReqTarget.auto([filter]), undefined, 10_000, undefined,
  );
  while (true) {
    const item = await stream.next();
    if (item === undefined) break;
    if (item.event !== undefined) {
      console.log(item.event.asJson());
    } else if (item.error !== undefined) {
      console.log(`Relay error from ${item.relayUrl}: ${item.error}`);
    }
  }
}
