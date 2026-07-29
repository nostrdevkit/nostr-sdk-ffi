import { copyFile, mkdir, readFile, rm, writeFile } from "node:fs/promises";
import { basename, resolve } from "node:path";

const packageManifest = JSON.parse(
  await readFile(new URL("../package.json", import.meta.url), "utf8"),
);
const targets = {
  "darwin-arm64": { os: "darwin", cpu: "arm64" },
  "darwin-x64": { os: "darwin", cpu: "x64" },
  "linux-arm64-gnu": { os: "linux", cpu: "arm64", libc: "glibc" },
  "linux-arm64-musl": { os: "linux", cpu: "arm64", libc: "musl" },
  "linux-x64-gnu": { os: "linux", cpu: "x64", libc: "glibc" },
  "linux-x64-musl": { os: "linux", cpu: "x64", libc: "musl" },
  "win32-arm64-msvc": { os: "win32", cpu: "arm64" },
  "win32-x64-msvc": { os: "win32", cpu: "x64" },
};

const [triple, libraryArgument] = process.argv.slice(2);
const target = targets[triple];

if (target === undefined || libraryArgument === undefined) {
  throw new Error("Usage: package-native.mjs <node-triple> <library>");
}

const source = resolve(libraryArgument);
const expectedLibrary =
  target.os === "win32"
    ? "nostr_sdk_ffi.dll"
    : `libnostr_sdk_ffi.${target.os === "darwin" ? "dylib" : "so"}`;

if (basename(source) !== expectedLibrary) {
  throw new Error(`Expected library named ${expectedLibrary}`);
}

const output = new URL(`../platforms/${triple}/`, import.meta.url);
await rm(output, { recursive: true, force: true });
await mkdir(output, { recursive: true });
await copyFile(source, new URL(expectedLibrary, output));

const manifest = {
  name: `@nostrdevkit/nostr-sdk-node-${triple}`,
  version: packageManifest.version,
  description: `Native Nostr SDK library for ${triple}.`,
  license: "MIT",
  repository: {
    type: "git",
    url: "git+https://github.com/nostrdevkit/nostr-sdk-ffi.git",
    directory: "js/node",
  },
  author: packageManifest.author,
  publishConfig: {
    access: "public",
  },
  os: [target.os],
  cpu: [target.cpu],
  files: [expectedLibrary],
};

if (target.libc !== undefined) {
  manifest.libc = [target.libc];
}

await writeFile(
  new URL("package.json", output),
  `${JSON.stringify(manifest, null, 2)}\n`,
);
