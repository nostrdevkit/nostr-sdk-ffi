import {
  Client,
  EventBuilder,
  Filter,
  Keys,
  Kind,
  KindStandard,
  RelayUrl,
  ReqTarget,
} from "../dist/index.js";

export async function clientExample() {
  const client = new Client();
  for (const url of ["wss://relay.damus.io", "wss://nostr.wine"]) {
    await client.addRelay(RelayUrl.parse(url));
  }
  await client.connect();

  const keys = Keys.generate();
  const kind = Kind.fromStd(KindStandard.TextNote);
  const event = new EventBuilder(kind, "Hello!").finalize(keys);
  const output = await client.sendEvent(event);
  console.log("Event sent:", output.id.toBech32(), output.success, output.failed);

  await new Promise((resolve) => setTimeout(resolve, 2_000));
  const filter = new Filter().author(keys.publicKey());
  const events = await client.fetchEvents(ReqTarget.auto([filter]), 10_000);
  events.forEach((event) => console.log(event.asJson()));
}
