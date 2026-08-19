import {
  ClientBuilder,
  LogLevel,
  Monitor,
  RelayUrl,
  initLogger,
} from "../dist/index.js";
import { runIfMain } from "./_main.mjs";

class MonitorHandler {
  async relayStatusChanged(relayUrl, status) {
    console.log(`Relay ${relayUrl} status changed to ${status}`);
  }
}

export async function main() {
  initLogger(LogLevel.Debug);

  const monitor = new Monitor();
  const client = new ClientBuilder().monitor(monitor).build();
  for (const url of [
    "wss://relay.damus.io",
    "wss://nostr.mom",
    "wss://nostr.oxtr.dev",
  ]) {
    await client.addRelay(RelayUrl.parse(url));
  }
  await client.connect();
  await monitor.handleNotifications(new MonitorHandler());
}

await runIfMain(import.meta.url, main);
