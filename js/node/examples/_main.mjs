import { pathToFileURL } from "node:url";

export async function runIfMain(moduleUrl, main) {
  if (process.argv[1] && moduleUrl === pathToFileURL(process.argv[1]).href) {
    await main();
  }
}
