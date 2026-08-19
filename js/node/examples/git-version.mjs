import { gitHashVersion } from "../dist/index.js";
import { runIfMain } from "./_main.mjs";

export function main() {
  console.log(gitHashVersion());
}

await runIfMain(import.meta.url, main);
