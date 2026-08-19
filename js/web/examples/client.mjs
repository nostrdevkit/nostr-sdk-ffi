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

export async function clientExample() {
  initLogger(LogLevel.Info);

  const client = new Client();
  for (const url of ["wss://relay.damus.io", "wss://nostr.wine"]) {
    await client.addRelay(RelayUrl.parse(url));
  }
  await client.connect();

  const keys = Keys.generate();
  const kind = Kind.fromStd(KindStandard.TextNote);
  await client.sendEvent(new EventBuilder(kind, "Hello!").finalize(keys));

  const unsignedEvent = new EventBuilder(kind, "Hello with POW!")
    .finalizeUnsigned(keys.publicKey());
  const minedEvent = await unsignedEvent.mineAsync(new SingleThreadPow(), 20);
  const output = await client.sendEvent(minedEvent.sign(keys));
  console.log("Event sent:", output.id.toBech32(), output.success, output.failed);

  await new Promise((resolve) => setTimeout(resolve, 2_000));
  const filter = new Filter().author(keys.publicKey());
  const events = await client.fetchEvents(ReqTarget.auto([filter]), 10_000);
  events.forEach((event) => console.log(event.asJson()));
}
