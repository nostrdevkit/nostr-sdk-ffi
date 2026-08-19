import {
  AdmitStatus,
  ClientBuilder,
  Filter,
  Kind,
  LogLevel,
  PublicKey,
  RelayUrl,
  ReqTarget,
  initLogger,
} from "../dist/index.js";
import { runIfMain } from "./_main.mjs";

class Filtering {
  #mutedPublicKeys = new Set();

  mute(publicKey) {
    this.#mutedPublicKeys.add(publicKey.toHex());
  }

  async admitConnection() {
    return AdmitStatus.success();
  }

  async admitEvent(_relayUrl, _subscriptionId, event) {
    return this.#mutedPublicKeys.has(event.author().toHex())
      ? AdmitStatus.rejected()
      : AdmitStatus.success();
  }
}

export async function main() {
  initLogger(LogLevel.Info);

  const mutedPublicKey = PublicKey.parse(
    "npub1l2vyh47mk2p0qlsku7hg0vn29faehy9hy34ygaclpn66ukqp3afqutajft",
  );
  const otherPublicKey = PublicKey.parse(
    "npub1xtscya34g58tk0z605fvr788k263gsu6cy9x0mhnm87echrgufzsevkk5s",
  );
  const filtering = new Filtering();
  filtering.mute(mutedPublicKey);

  const client = new ClientBuilder().admitPolicy(filtering).build();
  await client.addRelay(RelayUrl.parse("wss://relay.damus.io"));
  await client.connect();

  const filter = new Filter()
    .authors([mutedPublicKey, otherPublicKey])
    .kind(new Kind(0));
  const events = await client.fetchEvents(ReqTarget.auto([filter]), 10_000);
  console.log(`Received ${events.length} events`);
}

await runIfMain(import.meta.url, main);
