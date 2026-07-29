import { copyFile, mkdir, readFile, rm, writeFile } from "node:fs/promises";
import { basename, resolve } from "node:path";

const packageManifest = JSON.parse(
  await readFile(new URL("../package.json", import.meta.url), "utf8"),
);
const supportedAbis = new Set([
  "arm64-v8a",
  "armeabi-v7a",
  "x86",
  "x86_64",
]);
const [abi, libraryArgument] = process.argv.slice(2);

if (!supportedAbis.has(abi) || libraryArgument === undefined) {
  throw new Error("Usage: package-android.mjs <android-abi> <library>");
}

const source = resolve(libraryArgument);
const library = "libnostr_sdk_ffi.a";
if (basename(source) !== library) {
  throw new Error(`Expected library named ${library}`);
}

const packageName = `${packageManifest.name}-android-${abi}`;
if (packageManifest.dependencies?.[packageName] !== packageManifest.version) {
  throw new Error(`${packageName} is not declared at ${packageManifest.version}`);
}

const output = new URL(`../platforms/${abi}/`, import.meta.url);
await rm(output, { recursive: true, force: true });
await mkdir(output, { recursive: true });
await copyFile(source, new URL(library, output));

const manifest = {
  name: packageName,
  version: packageManifest.version,
  description: `Native Nostr SDK library for Android ${abi}.`,
  license: "MIT",
  repository: {
    type: "git",
    url: "git+https://github.com/nostrdevkit/nostr-sdk-ffi.git",
    directory: "js/react-native",
  },
  author: packageManifest.author,
  publishConfig: {
    access: "public",
  },
  files: [library],
};

await writeFile(
  new URL("package.json", output),
  `${JSON.stringify(manifest, null, 2)}\n`,
);
