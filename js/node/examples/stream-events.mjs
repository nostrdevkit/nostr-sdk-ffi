import {
  Client,
  Filter,
  Kind,
  LogLevel,
  RelayUrl,
  ReqTarget,
  initLogger,
} from "../dist/index.js";
import { runIfMain } from "./_main.mjs";

export async function main() {
  initLogger(LogLevel.Info);

  const client = new Client();
  await client.addRelay(RelayUrl.parse("wss://relay.damus.io"));
  await client.addRelay(RelayUrl.parse("wss://nos.lol"));
  await client.connect();

  console.log("Streaming events from relays...");
  const filter = new Filter().kind(new Kind(0)).limit(5n);
  const stream = await client.streamEvents(ReqTarget.auto([filter]), undefined, 10_000);

  while (true) {
    const item = await stream.next();
    if (item === undefined) {
      break;
    }
    if (item.event !== undefined) {
      console.log(item.event.asJson());
    } else if (item.error !== undefined) {
      console.log(`Relay error from ${item.relayUrl}: ${item.error}`);
    }
  }
  console.log("Stream terminated.");
}

await runIfMain(import.meta.url, main);
