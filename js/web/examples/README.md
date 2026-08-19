# Web example coverage

The WebAssembly binding mirrors every Python example whose API is available in
the browser. `database`, `custom-websocket-client`, `relay_builder`, and `tor`
are native-only because the WebAssembly binding does not export LMDB, custom
WebSocket transports, local relays, or proxy configuration.
