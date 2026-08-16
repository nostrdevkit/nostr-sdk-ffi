# Changelog

<!-- All notable changes to this project will be documented in this file. -->

<!-- The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), -->
<!-- and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html). -->

<!-- Template

## Unreleased

### Breaking changes

### Changed

### Added

### Fixed

### Removed

### Deprecated

-->

## v0.45.0 - 2026/08/05

### Breaking changes

- Align the client and relay APIs with the upstream 0.45 release, redesigning relay management, connections, subscriptions, event streaming and fetching, synchronization, message sending, unsubscription, and notification handling.
- Remove `ClientOptions`; clients must now be configured through `ClientBuilder`.
- Replace signer-based relay authentication with the dedicated `Authenticator` API.
- Remove the obsolete client `sign_event_builder` helper methods.
- Rename `NostrDatabase::delete` to `NostrDatabase::delete_events`.
- Change the `channel_size` argument of `Monitor::new` from `u64` to `u32`.
- Replace relay service flags with `RelayCapabilities`.
- Rework tag and event-builder APIs to match the upstream 0.45 data model and builder lifecycle.
- Update event streams and notifications to retain the originating relay and expose per-relay errors.

### Added

- Expose relay monitoring through `Monitor`, `ClientBuilder::monitor`, `Client::monitor`, and `HandleMonitorNotification`.
- Add JavaScript and TypeScript bindings for browsers, Node.js, and React Native.
- Expose SQLite-backed gossip storage.
- Add `NostrGossip::process_event`.
- Add `nip17_extract_relay_list`.
- Add expiration support to NIP-17 private-message and NIP-59 gift-wrap builders.
- Expose `UnsignedEvent::ensure_id`.
- Add `NostrDatabase::features`.
- Add ordering support for `Timestamp` and `EventId`.
- Add support for the `aarch64-unknown-freebsd` target.

### Changed

- Upgrade the upstream Nostr crates to version 0.45.
- Upgrade UniFFI to version 0.31.2.
- Improve WebAssembly compatibility.

### Fixed

- Fix an FFI panic caused by lifting flat error types.
- Fix Kotlin binding type mismatches.
- Remove the obsolete iOS simulator minimum-version linker flag from the Kotlin Multiplatform build.

### Removed

- Remove FFI implementations for NIP-15, NIP-39, NIP-48, NIP-51, NIP-53, NIP-57, NIP-88, NIP-94, NIP-96, and NIP-98.
- Remove unused event-builder signing methods from the client bindings.
- Remove superseded configuration and binding-generation workarounds.

## v0.44.2 - 2026/01/29

### Changed

- Bump nostr from 0.44.0 to 0.44.2
- Bump nostr-lmdb from 0.44.0 to 0.44.1
- Bump nostr-sdk from 0.44.0 to 0.44.1
- Bump tokio from 1.48.0 to 1.49.0

## v0.44.1 - 2025/11/21

### Fixed

- Fix crash on old Android APIs (https://github.com/nostrdevkit/nostr-sdk-ffi/pull/62)

## v0.44.0 - 2025/11/06

### Breaking changes

- Change `ClientOptions::gossip` behavior and arguments
- Remove `EventBuilder::reaction_extended` constructor

### Added

- Kotlin Multiplatform support (https://github.com/nostrdevkit/nostr-sdk-ffi/pull/39)
- Add support for Android and iOS runtimes in C# (https://github.com/nostrdevkit/nostr-sdk-ffi/pull/53)
- Add relay builder and local relay (https://github.com/nostrdevkit/nostr-sdk-ffi/pull/61)

### Changed

- Bump uniffi to 0.29.4 (https://github.com/nostrdevkit/nostr-sdk-ffi/pull/49)
- Bump nostr from 0.43.0 to 0.44.0 (see the Upstream CHANGELOG for more details)

## v0.43.0 - 2025/07/28

### Breaking changes

- Convert `AdmitStatus` from enum to object (https://github.com/nostrdevkit/nostr-sdk-ffi/pull/29)
- Remove `nip21_extract_from_text` and `Tags::from_text` in favor of `NostrParser`
- Remove getter and setters from `Metadata` object, in favor of `MetadataRecord
- Remove NIP-26 support (as per https://github.com/nostr-protocol/nips/pull/1051/commits/1733dd78b77bb95cde9b18db2671f33870bfcd98)
- Change the relay url arg type around the code from `String` to `RelayUrl` (https://github.com/nostrdevkit/nostr-sdk-ffi/pull/28)
- Update `Client::subscriptions` and `Client::subscription` output
- Rename `Options` to `ClientOptions`
- Convert NIP-05, NIP-11 and NIP-96 modules to be I/O-free

### Changed

- Set default params for `EventDeletionRequest` and `Contact`
- Bump nostr from 0.42.0 to 0.43.0 (see the Upstream CHANGELOG for more details)

### Added

- Expose `NostrParser` (https://github.com/nostrdevkit/nostr-sdk-ffi/pull/13)
- Expose arithmetic operations on `Timestamp` with `Duration` (https://github.com/nostrdevkit/nostr-sdk-ffi/pull/25)
- Expose `Timestamp::min` and `Timestamp::max`
- Re-expose `CustomNostrDatabase` (https://github.com/nostrdevkit/nostr-sdk-ffi/pull/33)
- Add `custom` field to `MetadataRecord`
- Expose `RelayUrl` (https://github.com/nostrdevkit/nostr-sdk-ffi/pull/28)
- Add support for `x86_64-unknown-freebsd` (https://github.com/nostrdevkit/nostr-sdk-ffi/pull/42)

### Fixed

- Fix NIP22 functions are not exposed

## v0.42.2 - 2025/06/09

### Fixed

- Update the android libraries to use 16KB page alignment (https://github.com/nostrdevkit/nostr-sdk-ffi/pull/18)

## v0.42.1 - 2025/05/26

### Changed

- Bump nostr from 0.42.0 to 0.42.1 (see the Upstream CHANGELOG for more details)

## v0.42.0 - 2025/05/20

### Breaking changes

- Rename `Nip46Request` to `NostrConnectRequest` (https://github.com/nostrdevkit/nostr-sdk-ffi/pull/11)
- Rename `ExtractedComment` to `CommentTarget` (https://github.com/nostrdevkit/nostr-sdk-ffi/pull/11)

### Changed

- Publish python wheels with cp39-abi3 (https://github.com/nostrdevkit/nostr-sdk-ffi/pull/7)
- Bump nostr upstream deps to 0.42.0 (see the Upstream CHANGELOG for more details, https://github.com/nostrdevkit/nostr-sdk-ffi/pull/11)

### Added

- Add support for event streaming (https://github.com/nostrdevkit/nostr-sdk-ffi/pull/6)
- Add `i686-unknown-linux-gnu`, `i686-pc-windows-msvc` and `aarch64-pc-windows-msvc` support for Python Wheels (https://github.com/nostrdevkit/nostr-sdk-ffi/pull/7)
- Add support for `i686-unknown-linux-musl`, `x86_64-unknown-linux-musl` and `aarch64-unknown-linux-musl` (https://github.com/nostrdevkit/nostr-sdk-ffi/pull/8)
- Add support for `armv7-unknown-linux-gnueabihf` and `armv7-unknown-linux-musleabihf` (https://github.com/nostrdevkit/nostr-sdk-ffi/pull/9)
- Add support for `riscv64gc-unknown-linux-gnu` and `riscv64gc-unknown-linux-musl` (https://github.com/nostrdevkit/nostr-sdk-ffi/pull/10)

## v0.41.0 - 2025/04/15

### Breaking changes

- Remove `TagKind::Clone` and handle as `Unknown` to fix issues with C# bindings

### Changed

- Bump upstream deps to 0.41.0 (see the upstream CHANGELOG for more details)

### Added

- Add support for `i686-pc-windows-msvc` and `aarch64-pc-windows-msvc`
- Add support to `i686-unknown-linux-gnu`
- Expose `Relay::ban`
- Derive `Hash` and `Display` traits where possible
