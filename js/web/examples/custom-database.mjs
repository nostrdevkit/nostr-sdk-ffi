import {
  ClientBuilder,
  DatabaseEventStatus,
  Filter,
  Keys,
  NostrDatabaseFeatures,
  RelayUrl,
  SaveEventStatus,
} from "../dist/index.js";

class InMemoryDatabase {
  events = new Map();

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
    this.events.set(event.id().toHex(), event);
    return SaveEventStatus.success();
  }

  async checkId(eventId) {
    return this.events.has(eventId.toHex())
      ? DatabaseEventStatus.Saved
      : DatabaseEventStatus.NotExistent;
  }

  async eventById(eventId) {
    return this.events.get(eventId.toHex());
  }

  async count() {
    return BigInt(this.events.size);
  }

  async query() {
    return [...this.events.values()].slice(0, 10);
  }

  async deleteEvents() {
    this.events.clear();
  }

  async wipe() {
    this.events.clear();
  }
}

export async function customDatabaseExample(connectToRelay = true) {
  const client = new ClientBuilder().database(new InMemoryDatabase()).build();
  if (!connectToRelay) {
    return client.database().count(new Filter());
  }

  await client.addRelay(RelayUrl.parse("wss://relay.damus.io"));
  await client.connect();

  const keys = Keys.parse(
    "nsec1ufnus6pju578ste3v90xd5m2decpuzpql2295m3sknqcjzyys9ls0qlc85",
  );
  await client.sync(new Filter().author(keys.publicKey()));

  const events = await client.database().query(
    new Filter().author(keys.publicKey()).limit(10n),
  );
  if (events.length === 0) console.log("Query did not find any event");
  events.forEach((event) => console.log(event.asJson()));
  return BigInt(events.length);
}
