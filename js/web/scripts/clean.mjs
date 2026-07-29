import { readdir, rm } from "node:fs/promises";
import path from "node:path";

const root = new URL("../", import.meta.url);

await Promise.all([
  rm(new URL("dist", root), { recursive: true, force: true }),
  rm(new URL("src/generated", root), { recursive: true, force: true }),
  rm(new URL("rust_modules/wasm", root), { recursive: true, force: true }),
]);

for (const entry of await readdir(root)) {
  if (entry.endsWith(".tgz")) {
    await rm(path.join(root.pathname, entry), { force: true });
  }
}
