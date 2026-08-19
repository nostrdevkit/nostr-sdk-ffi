using System.Net.WebSockets;
using System.Text;
using Nostr.Sdk;

NostrSdkMethods.InitLogger(LogLevel.Trace);

using var keys = Keys.Parse("nsec1ufnus6pju578ste3v90xd5m2decpuzpql2295m3sknqcjzyys9ls0qlc85");
using var authenticator = new SignerAuthenticator(keys);
using var clientBuilder = new ClientBuilder();
using var authBuilder = clientBuilder.Authenticator(authenticator);
using var transportBuilder = authBuilder.WebsocketTransport(new WebSocketTransport());
using var client = transportBuilder.Build();

using var relayUrl = RelayUrl.Parse("ws://127.0.0.1:7777");
await client.AddRelay(relayUrl);
await client.Connect();

using var kind = Kind.FromStd(KindStandard.TextNote);
using var eventBuilder = new EventBuilder(kind, "Test from nostrdevkit C# bindings!");
using var nostrEvent = eventBuilder.Finalize(keys);
var output = await client.SendEvent(nostrEvent);
Console.WriteLine("Event sent:");
Console.WriteLine($" hex:    {output.Id.ToHex()}");
Console.WriteLine($" bech32: {output.Id.ToBech32()}");
Console.WriteLine($" Successfully sent to: {string.Join(", ", output.Success.Select(url => url.ToString()))}");
Console.WriteLine($" Failed to send to: {string.Join(", ", output.Failed.Select(item => $"{item.Key}: {item.Value}"))}");

await Task.Delay(TimeSpan.FromSeconds(2));

using var publicKey = keys.PublicKey();
using var filter = new Filter().Author(publicKey);
using var target = ReqTarget.Auto(new[] { filter });
foreach (var receivedEvent in await client.FetchEvents(target, TimeSpan.FromSeconds(10)))
{
    using (receivedEvent)
    {
        Console.WriteLine(receivedEvent.AsPrettyJson());
    }
}

sealed class WebSocketTransport : CustomWebSocketTransport
{
    public bool SupportPing() => false;

    public async Task<WebSocketAdapterWrapper?> Connect(string url, SocketAddr? proxy)
    {
        if (proxy is not null)
        {
            throw new NotSupportedException("This example transport does not support proxies");
        }

        var socket = new ClientWebSocket();
        await socket.ConnectAsync(new Uri(url), CancellationToken.None);
        return new WebSocketAdapterWrapper(new WebSocketAdapter(socket));
    }
}

sealed class WebSocketAdapter(ClientWebSocket socket) : Nostr.Sdk.WebSocketAdapter
{
    public Task Send(WebSocketMessage message) => message switch
    {
        WebSocketMessage.Text text => socket.SendAsync(
            Encoding.UTF8.GetBytes(text.TextValue),
            WebSocketMessageType.Text,
            true,
            CancellationToken.None),
        WebSocketMessage.Binary binary => socket.SendAsync(
            binary.Bytes,
            WebSocketMessageType.Binary,
            true,
            CancellationToken.None),
        WebSocketMessage.Close => socket.CloseAsync(
            WebSocketCloseStatus.NormalClosure,
            null,
            CancellationToken.None),
        _ => throw new NotSupportedException("PING and PONG are handled by ClientWebSocket"),
    };

    public async Task<WebSocketMessage?> Recv()
    {
        using var payload = new MemoryStream();
        var buffer = new byte[16 * 1024];
        WebSocketReceiveResult result;
        do
        {
            result = await socket.ReceiveAsync(buffer, CancellationToken.None);
            payload.Write(buffer, 0, result.Count);
        }
        while (!result.EndOfMessage);

        return result.MessageType switch
        {
            WebSocketMessageType.Text =>
                new WebSocketMessage.Text(Encoding.UTF8.GetString(payload.ToArray())),
            WebSocketMessageType.Binary => new WebSocketMessage.Binary(payload.ToArray()),
            WebSocketMessageType.Close => new WebSocketMessage.Close(null),
            _ => null,
        };
    }

    public async Task CloseConnection()
    {
        if (socket.State is WebSocketState.Open or WebSocketState.CloseReceived)
        {
            await socket.CloseAsync(
                WebSocketCloseStatus.NormalClosure,
                null,
                CancellationToken.None);
        }
        socket.Dispose();
    }
}
