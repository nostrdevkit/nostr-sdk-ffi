import {
  EventBuilder,
  Keys,
  Kind,
  KindStandard,
  SingleThreadPow,
} from "../dist/index.js";
import { runIfMain } from "./_main.mjs";

export function main() {
  const keys = Keys.generate();

  const builder = new EventBuilder(
    Kind.fromStd(KindStandard.TextNote),
    "Note from rust-nostr JavaScript bindings",
  );
  console.log(builder.finalize(keys).asJson());

  const customBuilder = new EventBuilder(new Kind(1234), "My custom content");
  console.log(`Event: ${customBuilder.finalize(keys).asJson()}`);

  const powEvent = customBuilder
    .finalizeUnsigned(keys.publicKey())
    .mine(new SingleThreadPow(), 8)
    .sign(keys);
  console.log(`POW event: ${powEvent.asJson()}`);

  console.log(
    `Unsigned event: ${customBuilder.finalizeUnsigned(keys.publicKey()).asJson()}`,
  );
}

await runIfMain(import.meta.url, main);
