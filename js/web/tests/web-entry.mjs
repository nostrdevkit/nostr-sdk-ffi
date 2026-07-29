import {
  Filter,
  Keys,
  NostrDatabase,
  uniffiInitAsync,
} from "../dist/index.js";

let customDatabaseCountCalls = 0;
const customDatabase = {
  backend: () => "web-test",
  features: () => ({
    persistent: false,
    eventExpiration: false,
    fullTextSearch: false,
    requestToVanish: false,
  }),
  saveEvent: async () => undefined,
  checkId: async () => {
    throw new Error("Unexpected checkId call");
  },
  eventById: async () => undefined,
  count: async () => {
    customDatabaseCountCalls += 1;
    return 7n;
  },
  query: async () => [],
  deleteEvents: async () => {},
  wipe: async () => {},
};

try {
  await uniffiInitAsync();

  const keys = Keys.generate();
  const publicKey = keys.publicKey().toHex();

  if (typeof publicKey !== "string" || publicKey.length !== 64) {
    throw new Error("Invalid public key returned by the WASM binding");
  }

  const database = NostrDatabase.custom(customDatabase);
  const count = await database.count(new Filter());

  if (count !== 7n) {
    throw new Error(`Invalid custom database count: ${count}`);
  }

  await fetch("/result?status=ok");
} catch (error) {
  const details =
    typeof error === "object" && error !== null
      ? {
          name: error.name,
          message: error.message,
          stack: error.stack,
          customDatabaseCountCalls,
          ...error,
        }
      : { error, customDatabaseCountCalls };
  const message = JSON.stringify(details);
  await fetch(`/result?status=error&message=${encodeURIComponent(message)}`);
}
