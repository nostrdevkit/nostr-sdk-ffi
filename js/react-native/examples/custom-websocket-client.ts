import {
  ClientBuilder,
  CustomWebSocketTransport,
  EventBuilder,
  Filter,
  Keys,
  Kind,
  KindStandard,
  LogLevel,
  RelayUrl,
  ReqTarget,
  SignerAuthenticator,
  SocketAddr,
  SocketAddrLike,
  WebSocketAdapter,
  WebSocketAdapterWrapper,
  WebSocketAdapterWrapperLike,
  WebSocketMessage,
  initLogger,
} from "../src/generated/nostr_sdk";

export interface ExampleWebSocketConnection {
  send(message: WebSocketMessage): Promise<void>;
  receive(): Promise<WebSocketMessage | undefined>;
  close(): Promise<void>;
}

class ExampleWebSocketAdapter implements WebSocketAdapter {
  constructor(private readonly connection: ExampleWebSocketConnection) {}
  send(message: WebSocketMessage): Promise<void> { return this.connection.send(message); }
  recv(): Promise<WebSocketMessage | undefined> { return this.connection.receive(); }
  closeConnection(): Promise<void> { return this.connection.close(); }
}

class ExampleWebSocketTransport implements CustomWebSocketTransport {
  constructor(
    private readonly connectHandler: (url: string) => Promise<ExampleWebSocketConnection>,
  ) {}

  supportPing(): boolean { return false; }

  async connect(
    url: string,
    proxy: SocketAddrLike | undefined,
  ): Promise<WebSocketAdapterWrapperLike> {
    if (proxy !== undefined) {
      throw new Error("This example transport does not support proxies");
    }
    return new WebSocketAdapterWrapper(
      new ExampleWebSocketAdapter(await this.connectHandler(url)),
    );
  }
}

export async function main(
  connect: (url: string) => Promise<ExampleWebSocketConnection>,
): Promise<void> {
  initLogger(LogLevel.Trace);
  const keys = Keys.parse(
    "nsec1ufnus6pju578ste3v90xd5m2decpuzpql2295m3sknqcjzyys9ls0qlc85",
  );
  const client = new ClientBuilder()
    .authenticator(new SignerAuthenticator(keys))
    .websocketTransport(new ExampleWebSocketTransport(connect))
    .build();
  await client.addRelay(
    RelayUrl.parse("ws://127.0.0.1:7777"), undefined, false, undefined,
  );
  await client.connect(undefined);

  const event = new EventBuilder(
    Kind.fromStd(KindStandard.TextNote),
    "Test from nostrdevkit React Native bindings!",
  ).finalize(keys);
  const output = await client.sendEvent(
    event, undefined, undefined, undefined, undefined,
  );
  console.log("Event sent:", output.id.toBech32());

  const filter = new Filter().author(keys.publicKey());
  const events = await client.fetchEvents(
    ReqTarget.auto([filter]), 10_000, undefined, undefined,
  );
  events.forEach((receivedEvent) => console.log(receivedEvent.asPrettyJson()));
}
