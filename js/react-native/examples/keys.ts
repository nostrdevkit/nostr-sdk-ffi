import { Keys } from "../src/generated/nostr_sdk";

export function main(): string {
  return Keys.generate().publicKey().toBech32();
}
