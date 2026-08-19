import { Keys } from "../src/generated/nostr_sdk";

export function keysExample(): string {
  return Keys.generate().publicKey().toBech32();
}
