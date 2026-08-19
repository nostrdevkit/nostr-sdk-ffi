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

async function reportFailure(error) {
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

async function reportProgress(stage) {
  await fetch(`/progress?stage=${encodeURIComponent(stage)}`);
}

globalThis.addEventListener("error", (event) => {
  void reportFailure(event.error ?? event.message);
});
globalThis.addEventListener("unhandledrejection", (event) => {
  void reportFailure(event.reason);
});

try {
  await reportProgress("entrypoint loaded");
  const { ClientBuilder, Filter, Keys, uniffiInitAsync } = await import(
    "../dist/index.js"
  );
  const { keysExample } = await import("../examples/keys.mjs");
  await reportProgress("bindings imported");

  await uniffiInitAsync();
  await reportProgress("WebAssembly initialized");

  if (!keysExample().startsWith("npub1")) {
    throw new Error("Invalid bech32 public key returned by the example");
  }

  const keys = Keys.generate();
  const publicKey = keys.publicKey().toHex();

  if (typeof publicKey !== "string" || publicKey.length !== 64) {
    throw new Error("Invalid public key returned by the WASM binding");
  }

  const client = new ClientBuilder().database(customDatabase).build();
  const count = await client.database().count(new Filter());
  await reportProgress("database callback completed");

  if (count !== 7n) {
    throw new Error(`Invalid custom database count: ${count}`);
  }

  await fetch("/result?status=ok");
} catch (error) {
  await reportFailure(error);
}
