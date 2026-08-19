import { gitHashVersion } from "../src/generated/nostr_sdk";

export function main(): string | undefined {
  return gitHashVersion();
}
