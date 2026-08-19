import {
  ClientBuilder,
  Filter,
  Keys,
  LogLevel,
  NostrLmdb,
  RelayUrl,
  initLogger,
} from "../src/generated/nostr_sdk";

export async function main(databasePath: string): Promise<void> {
  initLogger(LogLevel.Info);

  const keys = Keys.parse(
    "nsec1ufnus6pju578ste3v90xd5m2decpuzpql2295m3sknqcjzyys9ls0qlc85",
  );
  console.log(keys.publicKey().toBech32());

  const client = new ClientBuilder()
    .database(await NostrLmdb.open(databasePath))
    .build();
  console.log(`Database backend: ${client.database().backend()}`);

  await client.addRelay(
    RelayUrl.parse("wss://relay.damus.io"), undefined, false, undefined,
  );
  await client.connect(undefined);
  await client.sync(new Filter().author(keys.publicKey()), undefined, undefined);

  const events = await client.database().query(
    new Filter().author(keys.publicKey()).limit(10n),
  );
  events.forEach((event) => console.log(event.asJson()));
}
