import {
  EventBuilder,
  Keys,
  Kind,
  KindStandard,
  SingleThreadPow,
} from "../src/generated/nostr_sdk";

export function main(): string[] {
  const keys = Keys.generate();
  const kind = Kind.fromStd(KindStandard.TextNote);
  const textNote = new EventBuilder(
    kind,
    "Note from rust-nostr React Native bindings",
  ).finalize(keys);

  const customBuilder = new EventBuilder(new Kind(1234), "My custom content");
  const customEvent = customBuilder.finalize(keys);
  const powEvent = customBuilder
    .finalizeUnsigned(keys.publicKey())
    .mine(new SingleThreadPow(), 8)
    .sign(keys);
  const unsignedEvent = customBuilder.finalizeUnsigned(keys.publicKey());

  return [
    textNote.asJson(),
    customEvent.asJson(),
    powEvent.asJson(),
    unsignedEvent.asJson(),
  ];
}
