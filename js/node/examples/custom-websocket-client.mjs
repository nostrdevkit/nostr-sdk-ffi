import {
  ClientBuilder,
  EventBuilder,
  Filter,
  Keys,
  Kind,
  KindStandard,
  LogLevel,
  RelayUrl,
  ReqTarget,
  SignerAuthenticator,
  WebSocketAdapterWrapper,
  WebSocketMessage,
  WebSocketMessage_Tags,
  initLogger,
} from "../dist/index.js";
import { runIfMain } from "./_main.mjs";

class WebSocketAdapter {
  #socket;
  #messages = [];
  #receivers = [];

  constructor(socket) {
    this.#socket = socket;
    socket.binaryType = "arraybuffer";
    socket.addEventListener("message", ({ data }) => {
      const message = typeof data === "string"
        ? new WebSocketMessage.Text({ text: data })
        : new WebSocketMessage.Binary({ bytes: data });
      this.#push(message);
    });
    socket.addEventListener("close", () => this.#push(undefined));
  }

  #push(message) {
    const receiver = this.#receivers.shift();
    if (receiver) {
      receiver(message);
    } else {
      this.#messages.push(message);
    }
  }

  async send(message) {
    switch (message.tag) {
      case WebSocketMessage_Tags.Text:
        this.#socket.send(message.inner.text);
        break;
      case WebSocketMessage_Tags.Binary:
        this.#socket.send(message.inner.bytes);
        break;
      case WebSocketMessage_Tags.Close:
        this.#socket.close();
        break;
      default:
        throw new Error(`Unsupported WebSocket message: ${message.tag}`);
    }
  }

  async recv() {
    if (this.#messages.length > 0) {
      return this.#messages.shift();
    }
    return new Promise((resolve) => this.#receivers.push(resolve));
  }

  async closeConnection() {
    this.#socket.close();
  }
}

class WebSocketTransport {
  supportPing() {
    return false;
  }

  async connect(url, proxy) {
    if (proxy !== undefined) {
      throw new Error("This example transport does not support proxies");
    }
    if (globalThis.WebSocket === undefined) {
      throw new Error("This example requires a runtime with the WebSocket API");
    }

    const socket = new WebSocket(url);
    await new Promise((resolve, reject) => {
      socket.addEventListener("open", resolve, { once: true });
      socket.addEventListener("error", reject, { once: true });
    });
    return new WebSocketAdapterWrapper(new WebSocketAdapter(socket));
  }
}

export async function main() {
  initLogger(LogLevel.Trace);

  const keys = Keys.parse(
    "nsec1ufnus6pju578ste3v90xd5m2decpuzpql2295m3sknqcjzyys9ls0qlc85",
  );
  const client = new ClientBuilder()
    .authenticator(new SignerAuthenticator(keys))
    .websocketTransport(new WebSocketTransport())
    .build();

  await client.addRelay(RelayUrl.parse("ws://127.0.0.1:7777"));
  await client.connect();

  const event = new EventBuilder(
    Kind.fromStd(KindStandard.TextNote),
    "Test from nostrdevkit JavaScript bindings!",
  ).finalize(keys);
  const output = await client.sendEvent(event);
  console.log("Event sent:");
  console.log(` hex:    ${output.id.toHex()}`);
  console.log(` bech32: ${output.id.toBech32()}`);
  console.log(" Successfully sent to:", output.success);
  console.log(" Failed to send to:", output.failed);

  await new Promise((resolve) => setTimeout(resolve, 2_000));

  const filter = new Filter().author(keys.publicKey());
  const events = await client.fetchEvents(ReqTarget.auto([filter]), 10_000);
  for (const receivedEvent of events) {
    console.log(receivedEvent.asPrettyJson());
  }
}

await runIfMain(import.meta.url, main);
