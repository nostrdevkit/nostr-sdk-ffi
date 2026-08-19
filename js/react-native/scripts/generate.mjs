import { spawn } from "node:child_process";
import { fileURLToPath } from "node:url";

const packageRoot = new URL("../", import.meta.url);
const repositoryRoot = new URL("../../", packageRoot);
const libraryNames = {
  darwin: "libnostr_sdk_ffi.dylib",
  linux: "libnostr_sdk_ffi.so",
  win32: "nostr_sdk_ffi.dll",
};
const libraryName = libraryNames[process.platform];

if (libraryName === undefined) {
  throw new Error(`Unsupported build platform: ${process.platform}`);
}

function run(command, args, cwd) {
  return new Promise((resolve, reject) => {
    const child = spawn(command, args, {
      cwd: fileURLToPath(cwd),
      stdio: "inherit",
    });

    child.once("error", reject);
    child.once("exit", (code) => {
      if (code === 0) {
        resolve();
      } else {
        reject(new Error(`${command} exited with status ${code}`));
      }
    });
  });
}

const configuredLibrary = process.env.NOSTR_SDK_FFI_LIBRARY;

if (configuredLibrary === undefined) {
  await run("cargo", ["build", "--lib"], repositoryRoot);
}

const library =
  configuredLibrary ??
  fileURLToPath(new URL(`target/debug/${libraryName}`, repositoryRoot));
const typescript = fileURLToPath(new URL("src/generated", packageRoot));
const cpp = fileURLToPath(new URL("cpp/generated", packageRoot));
const config = fileURLToPath(new URL("ubrn.config.yaml", packageRoot));
const ubrn = fileURLToPath(new URL("../node_modules/.bin/ubrn", packageRoot));

await run(
  ubrn,
  [
    "generate",
    "jsi",
    "bindings",
    library,
    "--library",
    "--ts-dir",
    typescript,
    "--cpp-dir",
    cpp,
    "--no-format",
  ],
  repositoryRoot,
);

await run(
  ubrn,
  [
    "generate",
    "jsi",
    "turbo-module",
    "nostr_sdk",
    "--native-bindings",
    "--config",
    config,
  ],
  packageRoot,
);
