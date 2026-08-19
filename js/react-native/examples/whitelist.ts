import {
  AdmitPolicy,
  AdmitStatus,
  AdmitStatusLike,
  ClientBuilder,
  EventLike,
  Filter,
  Kind,
  LogLevel,
  PublicKey,
  PublicKeyLike,
  RelayUrl,
  RelayUrlLike,
  ReqTarget,
  initLogger,
} from "../src/generated/nostr_sdk";

class WebOfTrust implements AdmitPolicy {
  private readonly allowedPublicKeys = new Set<string>();

  allow(publicKey: PublicKeyLike): void {
    this.allowedPublicKeys.add(publicKey.toHex());
  }

  async admitConnection(_relayUrl: RelayUrlLike): Promise<AdmitStatusLike> {
    return AdmitStatus.success();
  }

  async admitEvent(
    _relayUrl: RelayUrlLike,
    _subscriptionId: string,
    event: EventLike,
  ): Promise<AdmitStatusLike> {
    return this.allowedPublicKeys.has(event.author().toHex())
      ? AdmitStatus.success()
      : AdmitStatus.rejected();
  }
}

export async function main(): Promise<void> {
  initLogger(LogLevel.Info);
  const allowed = PublicKey.parse(
    "npub1l2vyh47mk2p0qlsku7hg0vn29faehy9hy34ygaclpn66ukqp3afqutajft",
  );
  const other = PublicKey.parse(
    "npub1xtscya34g58tk0z605fvr788k263gsu6cy9x0mhnm87echrgufzsevkk5s",
  );
  const webOfTrust = new WebOfTrust();
  webOfTrust.allow(allowed);
  const client = new ClientBuilder().admitPolicy(webOfTrust).build();
  await client.addRelay(
    RelayUrl.parse("wss://relay.damus.io"), undefined, false, undefined,
  );
  await client.connect(undefined);

  const filter = new Filter().authors([allowed, other]).kind(new Kind(0));
  const events = await client.fetchEvents(
    ReqTarget.auto([filter]), 10_000, undefined, undefined,
  );
  console.log(`Received ${events.length} events`);
}
