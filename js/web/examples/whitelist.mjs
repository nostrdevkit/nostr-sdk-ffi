import {
  AdmitStatus,
  ClientBuilder,
  Filter,
  Kind,
  PublicKey,
  RelayUrl,
  ReqTarget,
} from "../dist/index.js";

class WebOfTrust {
  allowedPublicKeys = new Set();

  allow(publicKey) {
    this.allowedPublicKeys.add(publicKey.toHex());
  }

  async admitConnection() {
    return AdmitStatus.success();
  }

  async admitEvent(_relayUrl, _subscriptionId, event) {
    return this.allowedPublicKeys.has(event.author().toHex())
      ? AdmitStatus.success()
      : AdmitStatus.rejected();
  }
}

export async function whitelistExample() {
  const allowed = PublicKey.parse(
    "npub1l2vyh47mk2p0qlsku7hg0vn29faehy9hy34ygaclpn66ukqp3afqutajft",
  );
  const other = PublicKey.parse(
    "npub1xtscya34g58tk0z605fvr788k263gsu6cy9x0mhnm87echrgufzsevkk5s",
  );
  const webOfTrust = new WebOfTrust();
  webOfTrust.allow(allowed);
  const client = new ClientBuilder().admitPolicy(webOfTrust).build();
  await client.addRelay(RelayUrl.parse("wss://relay.damus.io"));
  await client.connect();
  const filter = new Filter().authors([allowed, other]).kind(new Kind(0));
  const events = await client.fetchEvents(ReqTarget.auto([filter]), 10_000);
  console.log(`Received ${events.length} events`);
}
