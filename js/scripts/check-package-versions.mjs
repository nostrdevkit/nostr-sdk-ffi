import { readFile } from "node:fs/promises";

const manifests = await Promise.all(
  [
    "../web/package.json",
    "../node/package.json",
    "../react-native/package.json",
  ].map(async (path) =>
    JSON.parse(await readFile(new URL(path, import.meta.url), "utf8")),
  ),
);
const [web, node, reactNative] = manifests;

if (web.version !== node.version || web.version !== reactNative.version) {
  throw new Error("JavaScript package versions do not match");
}

for (const [name, version] of Object.entries(node.optionalDependencies)) {
  if (version !== node.version) {
    throw new Error(`${name} does not match the Node.js package version`);
  }
}

for (const [name, version] of Object.entries(reactNative.dependencies)) {
  if (
    name.startsWith(`${reactNative.name}-android-`) &&
    version !== reactNative.version
  ) {
    throw new Error(`${name} does not match the React Native package version`);
  }
}

console.log(web.version);
