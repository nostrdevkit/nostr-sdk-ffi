import {
  ClientBuilder,
  HandleMonitorNotification,
  LogLevel,
  Monitor,
  RelayStatus,
  RelayUrl,
  initLogger,
} from "../src/generated/nostr_sdk";

class MonitorHandler implements HandleMonitorNotification {
  async relayStatusChanged(relayUrl: RelayUrl, status: RelayStatus): Promise<void> {
    console.log(`Relay ${relayUrl} status changed to ${status}`);
  }
}

export async function main(): Promise<void> {
  initLogger(LogLevel.Debug);
  const monitor = new Monitor();
  const client = new ClientBuilder().monitor(monitor).build();
  for (const url of [
    "wss://relay.damus.io",
    "wss://nostr.mom",
    "wss://nostr.oxtr.dev",
  ]) {
    await client.addRelay(RelayUrl.parse(url), undefined, false, undefined);
  }
  await client.connect(undefined);
  await monitor.handleNotifications(new MonitorHandler());
}
