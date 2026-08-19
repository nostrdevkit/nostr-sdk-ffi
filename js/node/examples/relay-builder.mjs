import {
  LocalRelayBuilder,
  QueryPolicyResult,
  WritePolicyResult,
} from "../dist/index.js";
import { runIfMain } from "./_main.mjs";

class RejectEmptyEvents {
  async admitEvent(event) {
    return event.content()
      ? new WritePolicyResult.Accept()
      : new WritePolicyResult.Reject({ message: "empty content" });
  }
}

class LocalQueriesOnly {
  async admitQuery(_query, socketAddress) {
    const isLocal =
      socketAddress.startsWith("127.0.0.1:") ||
      socketAddress.startsWith("[::1]:");
    return isLocal
      ? new QueryPolicyResult.Accept()
      : new QueryPolicyResult.Reject({ message: "local queries only" });
  }
}

export async function main() {
  const relay = new LocalRelayBuilder()
    .port(7676)
    .writePolicy(new RejectEmptyEvents())
    .queryPolicy(new LocalQueriesOnly())
    .build();

  await relay.run();
  console.log(`Relay URL: ${await relay.url()}`);
  await new Promise(() => {});
}

await runIfMain(import.meta.url, main);
