using Nostr.Sdk;

using var relayBuilder = new LocalRelayBuilder();
using var portBuilder = relayBuilder.Port(7676);
using var writeBuilder = portBuilder.WritePolicy(new RejectEmptyEvents());
using var queryBuilder = writeBuilder.QueryPolicy(new LocalQueriesOnly());
using var relay = queryBuilder.Build();

await relay.Run();
Console.WriteLine($"Relay URL: {await relay.Url()}");
await Task.Delay(Timeout.InfiniteTimeSpan);

sealed class RejectEmptyEvents : WritePolicy
{
    public Task<WritePolicyResult> AdmitEvent(Event nostrEvent, string socketAddr) =>
        Task.FromResult<WritePolicyResult>(string.IsNullOrEmpty(nostrEvent.Content())
            ? new WritePolicyResult.Reject("empty content")
            : new WritePolicyResult.Accept());
}

sealed class LocalQueriesOnly : QueryPolicy
{
    public Task<QueryPolicyResult> AdmitQuery(Filter query, string socketAddr)
    {
        var isLocal = socketAddr.StartsWith("127.0.0.1:") || socketAddr.StartsWith("[::1]:");
        return Task.FromResult<QueryPolicyResult>(isLocal
            ? new QueryPolicyResult.Accept()
            : new QueryPolicyResult.Reject("local queries only"));
    }
}
