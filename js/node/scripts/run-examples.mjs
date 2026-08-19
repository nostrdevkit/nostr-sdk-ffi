import { readdir } from "node:fs/promises";

const examplesDirectory = new URL("../examples/", import.meta.url);
const examples = (await readdir(examplesDirectory))
  .filter((entry) => entry.endsWith(".mjs"))
  .sort();

for (const example of examples) {
  console.log(`Running ${example}`);
  await import(new URL(example, examplesDirectory));
}
