async function reportFailure(error) {
  const details =
    typeof error === "object" && error !== null
      ? {
          name: error.name,
          message: error.message,
          stack: error.stack,
          ...error,
        }
      : { error };
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
  const { Keys, uniffiInitAsync } = await import("../dist/index.js");
  const { customDatabaseExample } = await import("../examples/custom-database.mjs");
  const { eventBuilderExample } = await import("../examples/event_builder.mjs");
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

  const events = eventBuilderExample();
  await reportProgress("event builder example completed");

  if (events.length !== 3 || events.some((event) => !event.startsWith("{"))) {
    throw new Error("Invalid event builder example result");
  }

  const databaseCount = await customDatabaseExample(false);
  await reportProgress("custom database example completed");

  if (databaseCount !== 0n) {
    throw new Error(`Invalid custom database result: ${databaseCount}`);
  }

  await fetch("/result?status=ok");
} catch (error) {
  await reportFailure(error);
}
