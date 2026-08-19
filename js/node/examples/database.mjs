import {
  ClientBuilder,
  Filter,
  Keys,
  LogLevel,
  NostrLmdb,
  RelayUrl,
  initLogger,
} from "../dist/index.js";
import { runIfMain } from "./_main.mjs";

export async function main() {
  initLogger(LogLevel.Info);

  const keys = Keys.parse(
    "nsec1ufnus6pju578ste3v90xd5m2decpuzpql2295m3sknqcjzyys9ls0qlc85",
  );
  console.log(keys.publicKey().toBech32());

  const lmdb = await NostrLmdb.open("nostr-lmdb");
  const client = new ClientBuilder().database(lmdb).build();
  console.log(`Database backend: ${client.database().backend()}`);

  await client.addRelay(RelayUrl.parse("wss://relay.damus.io"));
  await client.connect();

  await client.sync(new Filter().author(keys.publicKey()));

  const events = await client.database().query(
    new Filter().author(keys.publicKey()).limit(10n),
  );
  for (const event of events) {
    console.log(event.asJson());
  }
}

await runIfMain(import.meta.url, main);
