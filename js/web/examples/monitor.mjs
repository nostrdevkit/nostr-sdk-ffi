import {
  ClientBuilder,
  LogLevel,
  Monitor,
  RelayUrl,
  initLogger,
} from "../dist/index.js";

class MonitorHandler {
  async relayStatusChanged(relayUrl, status) {
    console.log(`Relay ${relayUrl} status changed to ${status}`);
  }
}

export async function monitorExample() {
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
