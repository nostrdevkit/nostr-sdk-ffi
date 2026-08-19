import { Keys } from "../dist/index.js";
import { runIfMain } from "./_main.mjs";

export function main() {
  const keys = Keys.generate();
  console.log(keys.publicKey().toBech32());
}

await runIfMain(import.meta.url, main);
