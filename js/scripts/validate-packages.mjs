import { execFileSync } from "node:child_process";
import { readdir, readFile, stat } from "node:fs/promises";
import { resolve } from "node:path";

const packagesRoot = resolve(process.argv[2] ?? "packages");
const maxTarballSize = 128 * 1024 * 1024;

async function findTarballs(directory) {
  const tarballs = [];

  for (const entry of await readdir(directory, { withFileTypes: true })) {
    const path = resolve(directory, entry.name);
    if (entry.isDirectory()) {
      tarballs.push(...(await findTarballs(path)));
    } else if (entry.isFile() && entry.name.endsWith(".tgz")) {
      tarballs.push(path);
    }
  }

  return tarballs;
}

async function readManifest(path) {
  return JSON.parse(await readFile(new URL(path, import.meta.url), "utf8"));
}

function tarOutput(...args) {
  return execFileSync("tar", args, {
    encoding: "utf8",
    maxBuffer: 16 * 1024 * 1024,
  });
}

const [web, node, reactNative] = await Promise.all([
  readManifest("../web/package.json"),
  readManifest("../node/package.json"),
  readManifest("../react-native/package.json"),
]);

if (web.version !== node.version || web.version !== reactNative.version) {
  throw new Error("JavaScript package versions do not match");
}

const requiredFiles = new Map([
  [
    web.name,
    [
      "package/dist/index.js",
      "package/dist/index.d.ts",
      "package/dist/generated/web/nostr_sdk.js",
      "package/dist/generated/web/nostr_sdk.d.ts",
      "package/dist/generated/web/wasm-bindgen/index.js",
      "package/dist/generated/web/wasm-bindgen/index.d.ts",
      "package/dist/generated/web/wasm-bindgen/index_bg.wasm",
    ],
  ],
  [
    node.name,
    [
      "package/dist/index.js",
      "package/dist/index.d.ts",
      "package/dist/generated/nostr_sdk.d.ts",
    ],
  ],
  [
    reactNative.name,
    [
      "package/src/index.tsx",
      "package/src/NativeNostrSdkReactNative.ts",
      "package/src/generated/nostr_sdk.ts",
      "package/cpp/generated/nostr_sdk.cpp",
      "package/cpp/generated/nostr_sdk.hpp",
      "package/NostrSdkReactNative.xcframework/Info.plist",
      "package/NostrSdkReactNative.xcframework/ios-arm64/libnostr_sdk_ffi.a",
      "package/NostrSdkReactNative.xcframework/ios-arm64_x86_64-simulator/libnostr_sdk_ffi.a",
    ],
  ],
]);

for (const name of Object.keys(node.optionalDependencies)) {
  let library;
  if (name.includes("-darwin-")) {
    library = "libnostr_sdk_ffi.dylib";
  } else if (name.includes("-linux-")) {
    library = "libnostr_sdk_ffi.so";
  } else if (name.includes("-win32-")) {
    library = "nostr_sdk_ffi.dll";
  } else {
    throw new Error(`Unsupported Node.js native package: ${name}`);
  }

  requiredFiles.set(name, [`package/${library}`]);
}

for (const name of Object.keys(reactNative.dependencies)) {
  if (name.startsWith(`${reactNative.name}-android-`)) {
    requiredFiles.set(name, ["package/libnostr_sdk_ffi.a"]);
  }
}

const tarballs = (await findTarballs(packagesRoot)).sort();
if (tarballs.length !== requiredFiles.size) {
  throw new Error(
    `Expected ${requiredFiles.size} JavaScript packages, found ${tarballs.length}`,
  );
}

const found = new Set();
for (const tarball of tarballs) {
  const tarballSize = (await stat(tarball)).size;
  if (tarballSize > maxTarballSize) {
    throw new Error(
      `${tarball} is ${tarballSize} bytes; the limit is ${maxTarballSize}`,
    );
  }

  const files = new Map(
    tarOutput("-tvzf", tarball)
      .trim()
      .split("\n")
      .map((line) => {
        const columns = line.trim().split(/\s+/);
        return [columns[5], Number(columns[2])];
      }),
  );
  const manifest = JSON.parse(
    tarOutput("-xOzf", tarball, "package/package.json"),
  );
  const expectedFiles = requiredFiles.get(manifest.name);

  if (expectedFiles === undefined) {
    throw new Error(`Unexpected package: ${manifest.name}`);
  }
  if (found.has(manifest.name)) {
    throw new Error(`Duplicate package: ${manifest.name}`);
  }
  if (manifest.version !== web.version) {
    throw new Error(
      `${manifest.name} has version ${manifest.version}, expected ${web.version}`,
    );
  }

  for (const script of ["preinstall", "install", "postinstall", "prepare"]) {
    if (manifest.scripts?.[script] !== undefined) {
      throw new Error(`${manifest.name} must not define a ${script} script`);
    }
  }

  for (const file of expectedFiles) {
    const size = files.get(file);
    if (size === undefined) {
      throw new Error(`${manifest.name} is missing ${file}`);
    }
    if (size === 0) {
      throw new Error(`${manifest.name} contains an empty ${file}`);
    }
  }

  if (
    manifest.name === reactNative.name &&
    [...files.keys()].some((file) =>
      file.startsWith("package/android/src/main/jniLibs/"),
    )
  ) {
    throw new Error(
      `${manifest.name} must not contain the Android native libraries`,
    );
  }

  found.add(manifest.name);
  console.log(`${manifest.name}@${manifest.version}`);
}

for (const name of requiredFiles.keys()) {
  if (!found.has(name)) {
    throw new Error(`Missing package: ${name}`);
  }
}
