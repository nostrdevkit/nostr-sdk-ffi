import { rm } from "node:fs/promises";

const root = new URL("../", import.meta.url);

await Promise.all([
  rm(new URL("android/generated", root), { recursive: true, force: true }),
  rm(new URL("android/src/main/jniLibs", root), {
    recursive: true,
    force: true,
  }),
  rm(new URL("cpp", root), { recursive: true, force: true }),
  rm(new URL("ios/generated", root), { recursive: true, force: true }),
  rm(new URL("src", root), { recursive: true, force: true }),
]);
