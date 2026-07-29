import { execFileSync } from "node:child_process";
import { readdir } from "node:fs/promises";
import { resolve } from "node:path";

const npmTag = process.env.NPM_TAG;
if (npmTag === undefined) {
  throw new Error("NPM_TAG is not set");
}
const npmToken = process.env.NODE_AUTH_TOKEN;

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

function readManifest(tarball) {
  return JSON.parse(
    execFileSync("tar", ["-xOzf", tarball, "package/package.json"], {
      encoding: "utf8",
    }),
  );
}

const directories = process.argv.slice(2);
if (directories.length === 0) {
  throw new Error("Usage: publish-packages.mjs <directory>...");
}

const tarballs = (
  await Promise.all(directories.map((directory) => findTarballs(directory)))
)
  .flat()
  .sort();

for (const tarball of tarballs) {
  const manifest = readManifest(tarball);
  const packagePath = encodeURIComponent(manifest.name);
  const versionPath = encodeURIComponent(manifest.version);
  const response = await fetch(
    `https://registry.npmjs.org/${packagePath}/${versionPath}`,
    {
      headers:
        npmToken === undefined
          ? {}
          : {
              Authorization: `Bearer ${npmToken}`,
            },
    },
  );

  if (response.ok) {
    console.log(
      `Skipping ${manifest.name}@${manifest.version}: already published`,
    );
    continue;
  }
  if (response.status !== 404) {
    throw new Error(
      `Unable to check ${manifest.name}@${manifest.version}: npm returned ${response.status}`,
    );
  }

  execFileSync(
    "npm",
    [
      "publish",
      tarball,
      "--access",
      "public",
      "--tag",
      npmTag,
      "--provenance",
    ],
    { stdio: "inherit" },
  );
}
