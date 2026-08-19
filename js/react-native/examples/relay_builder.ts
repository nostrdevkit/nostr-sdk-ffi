import {
  Event,
  Filter,
  LocalRelayBuilder,
  QueryPolicy,
  QueryPolicyResult,
  WritePolicy,
  WritePolicyResult,
} from "../src/generated/nostr_sdk";

class RejectEmptyEvents implements WritePolicy {
  async admitEvent(event: Event, _socketAddress: string): Promise<WritePolicyResult> {
    return event.content()
      ? new WritePolicyResult.Accept()
      : new WritePolicyResult.Reject({ message: "empty content" });
  }
}

class LocalQueriesOnly implements QueryPolicy {
  async admitQuery(_query: Filter, socketAddress: string): Promise<QueryPolicyResult> {
    const isLocal =
      socketAddress.startsWith("127.0.0.1:") ||
      socketAddress.startsWith("[::1]:");
    return isLocal
      ? new QueryPolicyResult.Accept()
      : new QueryPolicyResult.Reject({ message: "local queries only" });
  }
}

export async function main(): Promise<void> {
  const relay = new LocalRelayBuilder()
    .port(7676)
    .writePolicy(new RejectEmptyEvents())
    .queryPolicy(new LocalQueriesOnly())
    .build();
  await relay.run();
  console.log(`Relay URL: ${await relay.url()}`);
  await new Promise<void>(() => {});
}
