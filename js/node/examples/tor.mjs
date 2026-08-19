import {
  ClientBuilder,
  EventBuilder,
  Keys,
  Kind,
  KindStandard,
  LogLevel,
  Proxy,
  RelayUrl,
  SignerAuthenticator,
  SocketAddr,
  initLogger,
} from "../dist/index.js";
import { runIfMain } from "./_main.mjs";

class TorProxy {
  custom(relayUrl) {
    return relayUrl.isOnion()
      ? SocketAddr.parse("127.0.0.1:9050")
      : undefined;
  }
}

export async function main() {
  initLogger(LogLevel.Info);

  const keys = Keys.generate();
  console.log(keys.publicKey().toBech32());

  const client = new ClientBuilder()
    .authenticator(new SignerAuthenticator(keys))
    .proxy(Proxy.custom(new TorProxy()))
    .build();

  for (const url of [
    "wss://relay.damus.io",
    "ws://oxtrdevav64z64yb7x6rjg4ntzqjhedm5b5zjqulugknhzr46ny2qbad.onion",
    "ws://2jsnlhfnelig5acq6iacydmzdbdmg7xwunm4xl6qwbvzacw4lwrjmlyd.onion",
  ]) {
    await client.addRelay(RelayUrl.parse(url));
  }
  await client.connect();

  const event = new EventBuilder(
    Kind.fromStd(KindStandard.TextNote),
    "Hello from rust-nostr JavaScript bindings!",
  ).finalize(keys);
  const output = await client.sendEvent(event);
  console.log("Event sent:");
  console.log(` hex:    ${output.id.toHex()}`);
  console.log(` bech32: ${output.id.toBech32()}`);
  console.log(" Successfully sent to:", output.success);
  console.log(" Failed to send to:", output.failed);
}

await runIfMain(import.meta.url, main);
