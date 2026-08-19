import {
  Client,
  EventBuilder,
  Filter,
  Keys,
  Kind,
  KindStandard,
  LogLevel,
  RelayUrl,
  ReqTarget,
  SingleThreadPow,
  initLogger,
} from "../dist/index.js";
import { runIfMain } from "./_main.mjs";

export async function main() {
  initLogger(LogLevel.Info);

  const client = new Client();
  for (const url of ["wss://relay.damus.io", "wss://nostr.wine"]) {
    await client.addRelay(RelayUrl.parse(url));
  }
  await client.connect();

  const keys = Keys.generate();
  const kind = Kind.fromStd(KindStandard.TextNote);
  const event = new EventBuilder(kind, "Hello!").finalize(keys);
  await client.sendEvent(event);

  console.log("Mining a POW text note...");
  const unsignedEvent = new EventBuilder(kind, "Hello with POW!")
    .finalizeUnsigned(keys.publicKey());
  const minedEvent = await unsignedEvent.mineAsync(new SingleThreadPow(), 20);
  const output = await client.sendEvent(minedEvent.sign(keys));
  console.log("Event sent:");
  console.log(` hex:    ${output.id.toHex()}`);
  console.log(` bech32: ${output.id.toBech32()}`);
  console.log(" Successfully sent to:", output.success);
  console.log(" Failed to send to:", output.failed);

  await new Promise((resolve) => setTimeout(resolve, 2_000));

  console.log("Getting events from relays...");
  const filter = new Filter().author(keys.publicKey());
  const events = await client.fetchEvents(ReqTarget.auto([filter]), 10_000);
  for (const receivedEvent of events) {
    console.log(receivedEvent.asJson());
  }
}

await runIfMain(import.meta.url, main);
