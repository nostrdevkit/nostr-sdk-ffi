// Copyright (c) 2022-2023 Yuki Kishimoto
// Copyright (c) 2023-2025 Rust Nostr Developers
// Distributed under the MIT software license

use nostr::types;
use uniffi::Record;

#[derive(Record)]
pub struct ImageDimensions {
    /// Width
    pub width: u64,
    /// Height
    pub height: u64,
}

impl From<types::ImageDimensions> for ImageDimensions {
    fn from(inner: types::ImageDimensions) -> Self {
        Self {
            width: inner.width,
            height: inner.height,
        }
    }
}

impl From<ImageDimensions> for types::ImageDimensions {
    fn from(inner: ImageDimensions) -> Self {
        Self {
            width: inner.width,
            height: inner.height,
        }
    }
}
