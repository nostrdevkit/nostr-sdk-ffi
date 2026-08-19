import assert from "node:assert/strict";
import test from "node:test";

test("package exports the generated API", async () => {
  const sdk = await import("../dist/index.js");

  assert.equal(typeof sdk.Keys, "function");
  const keys = sdk.Keys.generate();
  assert.equal(typeof keys.publicKey().toHex(), "string");

  const customDatabase = {
    backend: () => "node-test",
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
    count: async () => 7n,
    query: async () => [],
    deleteEvents: async () => {},
    wipe: async () => {},
  };
  const client = new sdk.ClientBuilder().database(customDatabase).build();

  assert.equal(await client.database().count(new sdk.Filter()), 7n);
});
