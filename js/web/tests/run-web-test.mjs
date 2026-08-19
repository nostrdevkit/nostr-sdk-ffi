import { spawn } from "node:child_process";
import { mkdtemp, readFile, rm, writeFile } from "node:fs/promises";
import { createServer } from "node:http";
import { tmpdir } from "node:os";
import { extname, join } from "node:path";
import { build } from "esbuild";

const output = await mkdtemp(join(tmpdir(), "nostr-sdk-web-test-"));
const entrypoint = new URL("web-entry.mjs", import.meta.url);

await build({
  entryPoints: [entrypoint.pathname],
  bundle: true,
  format: "esm",
  loader: { ".wasm": "file" },
  outdir: output,
  platform: "browser",
});

await writeFile(
  join(output, "index.html"),
  '<!doctype html><meta charset="utf-8"><script type="module" src="/web-entry.js"></script>',
);

let settle;
let lastStage = "browser not started";
const result = new Promise((resolve, reject) => {
  settle = { resolve, reject };
});
const contentTypes = {
  ".html": "text/html; charset=utf-8",
  ".js": "text/javascript; charset=utf-8",
  ".wasm": "application/wasm",
};

const server = createServer(async (request, response) => {
  const url = new URL(request.url ?? "/", "http://127.0.0.1");

  if (url.pathname === "/result") {
    response.writeHead(204).end();
    if (url.searchParams.get("status") === "ok") {
      settle.resolve();
    } else {
      const message = url.searchParams.get("message") ?? "No error details";
      settle.reject(new Error(`Web test failed after ${lastStage}: ${message}`));
    }
    return;
  }

  if (url.pathname === "/progress") {
    lastStage = url.searchParams.get("stage") ?? "unknown";
    response.writeHead(204).end();
    return;
  }

  const relativePath = url.pathname === "/" ? "index.html" : url.pathname.slice(1);

  try {
    const body = await readFile(join(output, relativePath));
    response
      .writeHead(200, {
        "Content-Type": contentTypes[extname(relativePath)] ?? "application/octet-stream",
      })
      .end(body);
  } catch {
    response.writeHead(404).end();
  }
});

await new Promise((resolve) => server.listen(0, "127.0.0.1", resolve));
const address = server.address();

if (address === null || typeof address === "string") {
  throw new Error("Failed to start the web test server");
}

const profile = await mkdtemp(join(tmpdir(), "nostr-sdk-firefox-"));
const firefox = spawn(
  process.env.FIREFOX_BIN ?? "firefox",
  [
    "--headless",
    "--no-remote",
    "--profile",
    profile,
    `http://127.0.0.1:${address.port}/`,
  ],
  { stdio: "inherit" },
);
firefox.once("error", (error) => {
  settle.reject(new Error(`Failed to start Firefox: ${error.message}`));
});
firefox.once("exit", (code, signal) => {
  settle.reject(
    new Error(
      `Firefox exited before completing the WebAssembly test: code=${code}, signal=${signal}`,
    ),
  );
});
const timeout = setTimeout(
  () =>
    settle.reject(
      new Error(
        `Timed out waiting for the WebAssembly test; last stage: ${lastStage}`,
      ),
    ),
  120_000,
);

try {
  await result;
} finally {
  clearTimeout(timeout);
  firefox.kill("SIGTERM");
  await new Promise((resolve) => server.close(resolve));
  await Promise.all([
    rm(output, { recursive: true, force: true }),
    rm(profile, { recursive: true, force: true }),
  ]);
}
