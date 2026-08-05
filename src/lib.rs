// Copyright (c) 2022-2023 Yuki Kishimoto
// Copyright (c) 2023-2025 Rust Nostr Developers
// Distributed under the MIT software license

#![allow(clippy::new_without_default)]
#![allow(clippy::len_without_is_empty)]
#![allow(clippy::wrong_self_convention)]

mod authenticator;
mod client;
#[cfg(feature = "connect")]
mod connect;
mod database;
mod error;
mod future;
mod gossip;
#[cfg(all(feature = "native", not(target_arch = "wasm32")))]
mod local_relay;
#[cfg(feature = "logger")]
mod logger;
mod monitor;
mod negentropy;
#[cfg(all(feature = "native", not(target_arch = "wasm32")))]
mod net;
#[cfg(feature = "nwc")]
mod nwc;
mod parser;
mod policy;
mod protocol;
#[cfg(all(feature = "native", not(target_arch = "wasm32")))]
mod proxy;
mod relay;
#[cfg(all(feature = "native", not(target_arch = "wasm32")))]
mod transport;
#[cfg(target_arch = "wasm32")]
mod wasm32_time;

/// Get git hash version of library
#[uniffi::export]
fn git_hash_version() -> Option<String> {
    option_env!("GIT_HASH").map(|v| v.to_string())
}

// Workaround to fix UPX compression error
//
// Error: CantPackException: need DT_INIT; try "void _init(void){}"
// Workaround comes from https://github.com/upx/upx/issues/740
#[unsafe(no_mangle)]
#[cfg(target_os = "android")]
fn _init() {}

// Changes to this arg will break binding packages (in particular Swift).
// If this is removed, make sure to update `uniffi.toml`
uniffi::setup_scaffolding!("nostr_sdk");
