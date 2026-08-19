import {
  ClientBuilder,
  CustomProxy,
  EventBuilder,
  Keys,
  Kind,
  KindStandard,
  LogLevel,
  Proxy,
  RelayUrl,
  RelayUrlLike,
  SignerAuthenticator,
  SocketAddr,
  SocketAddrLike,
  initLogger,
} from "../src/generated/nostr_sdk";

class TorProxy implements CustomProxy {
  custom(relayUrl: RelayUrlLike): SocketAddrLike | undefined {
    return relayUrl.isOnion()
      ? SocketAddr.parse("127.0.0.1:9050")
      : undefined;
  }
}

export async function main(): Promise<void> {
  initLogger(LogLevel.Info);
  const keys = Keys.generate();
  const client = new ClientBuilder()
    .authenticator(new SignerAuthenticator(keys))
    .proxy(Proxy.custom(new TorProxy()))
    .build();
  for (const url of [
    "wss://relay.damus.io",
    "ws://oxtrdevav64z64yb7x6rjg4ntzqjhedm5b5zjqulugknhzr46ny2qbad.onion",
    "ws://2jsnlhfnelig5acq6iacydmzdbdmg7xwunm4xl6qwbvzacw4lwrjmlyd.onion",
  ]) {
    await client.addRelay(RelayUrl.parse(url), undefined, false, undefined);
  }
  await client.connect(undefined);

  const event = new EventBuilder(
    Kind.fromStd(KindStandard.TextNote),
    "Hello from rust-nostr React Native bindings!",
  ).finalize(keys);
  const output = await client.sendEvent(
    event, undefined, undefined, undefined, undefined,
  );
  console.log("Event sent:", output.id.toBech32(), output.success, output.failed);
}
