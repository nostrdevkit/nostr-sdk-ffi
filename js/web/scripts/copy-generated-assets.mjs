import { cp } from "node:fs/promises";

const root = new URL("../", import.meta.url);
const source = new URL("src/generated", root);
const destination = new URL("dist/generated", root);

await cp(source, destination, {
  recursive: true,
  force: true,
  filter(path) {
    return !path.endsWith(".ts") && !path.endsWith(".rs");
  },
});
