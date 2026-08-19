import {
  ClientBuilder,
  DatabaseEventStatus,
  Filter,
  Keys,
  LogLevel,
  NostrDatabaseFeatures,
  RelayUrl,
  SaveEventStatus,
  initLogger,
} from "../dist/index.js";
import { runIfMain } from "./_main.mjs";

class InMemoryDatabase {
  #events = new Map();

  backend() {
    return "my-in-memory-backend";
  }

  features() {
    return NostrDatabaseFeatures.new({
      persistent: false,
      eventExpiration: false,
      fullTextSearch: false,
      requestToVanish: false,
    });
  }

  async saveEvent(event) {
    this.#events.set(event.id().toHex(), event);
    return SaveEventStatus.success();
  }

  async checkId(eventId) {
    return this.#events.has(eventId.toHex())
      ? DatabaseEventStatus.Saved
      : DatabaseEventStatus.NotExistent;
  }

  async eventById(eventId) {
    return this.#events.get(eventId.toHex());
  }

  async count() {
    return BigInt(this.#events.size);
  }

  async query() {
    return [...this.#events.values()].slice(0, 10);
  }

  async deleteEvents() {
    this.#events.clear();
  }

  async wipe() {
    this.#events.clear();
  }
}

export async function main() {
  initLogger(LogLevel.Info);

  const client = new ClientBuilder().database(new InMemoryDatabase()).build();
  await client.addRelay(RelayUrl.parse("wss://relay.damus.io"));
  await client.connect();

  const keys = Keys.parse(
    "nsec1ufnus6pju578ste3v90xd5m2decpuzpql2295m3sknqcjzyys9ls0qlc85",
  );
  console.log(keys.publicKey().toBech32());

  await client.sync(new Filter().author(keys.publicKey()));

  const events = await client.database().query(
    new Filter().author(keys.publicKey()).limit(10n),
  );
  if (events.length === 0) {
    console.log("Query did not find any event");
  } else {
    for (const event of events) {
      console.log(event.asJson());
    }
  }
}

await runIfMain(import.meta.url, main);
