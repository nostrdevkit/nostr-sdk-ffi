import { Keys } from "../dist/index.js";

export function keysExample() {
  return Keys.generate().publicKey().toBech32();
}
