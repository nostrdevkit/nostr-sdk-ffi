import { rm } from "node:fs/promises";

const root = new URL("../", import.meta.url);

await Promise.all([
  rm(new URL("dist", root), { recursive: true, force: true }),
  rm(new URL("src/generated", root), { recursive: true, force: true }),
]);
