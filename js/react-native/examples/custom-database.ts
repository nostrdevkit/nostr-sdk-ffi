import {
  ClientBuilder,
  DatabaseEventStatus,
  EventLike,
  EventIdLike,
  Filter,
  Keys,
  LogLevel,
  NostrDatabase,
  NostrDatabaseFeatures,
  RelayUrl,
  SaveEventStatus,
  SaveEventStatusLike,
  initLogger,
} from "../src/generated/nostr_sdk";

class InMemoryDatabase implements NostrDatabase {
  private readonly events = new Map<string, EventLike>();

  backend(): string {
    return "my-in-memory-backend";
  }

  features(): NostrDatabaseFeatures {
    return NostrDatabaseFeatures.new({
      persistent: false,
      eventExpiration: false,
      fullTextSearch: false,
      requestToVanish: false,
    });
  }

  async saveEvent(event: EventLike): Promise<SaveEventStatusLike | undefined> {
    this.events.set(event.id().toHex(), event);
    return SaveEventStatus.success();
  }

  async checkId(eventId: EventIdLike): Promise<DatabaseEventStatus> {
    return this.events.has(eventId.toHex())
      ? DatabaseEventStatus.Saved
      : DatabaseEventStatus.NotExistent;
  }

  async eventById(eventId: EventIdLike): Promise<EventLike | undefined> {
    return this.events.get(eventId.toHex());
  }

  async count(_filter: Filter): Promise<bigint> {
    return BigInt(this.events.size);
  }

  async query(_filter: Filter): Promise<EventLike[]> {
    return [...this.events.values()].slice(0, 10);
  }

  async deleteEvents(_filter: Filter): Promise<void> {
    this.events.clear();
  }

  async wipe(): Promise<void> {
    this.events.clear();
  }
}

export async function main(): Promise<void> {
  initLogger(LogLevel.Info);

  const client = new ClientBuilder().database(new InMemoryDatabase()).build();
  await client.addRelay(
    RelayUrl.parse("wss://relay.damus.io"), undefined, false, undefined,
  );
  await client.connect(undefined);

  const keys = Keys.parse(
    "nsec1ufnus6pju578ste3v90xd5m2decpuzpql2295m3sknqcjzyys9ls0qlc85",
  );
  await client.sync(new Filter().author(keys.publicKey()), undefined, undefined);

  const events = await client.database().query(
    new Filter().author(keys.publicKey()).limit(10n),
  );
  if (events.length === 0) console.log("Query did not find any event");
  events.forEach((event) => console.log(event.asJson()));
}
