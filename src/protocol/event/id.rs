// Copyright (c) 2022-2023 Yuki Kishimoto
// Copyright (c) 2023-2025 Rust Nostr Developers
// Distributed under the MIT software license

use std::ops::Deref;
use std::sync::Arc;

use nostr::event;
use nostr::event::Tags;
use nostr::nips::nip19::ToBech32;
use nostr::nips::nip21::ToNostrUri;
use uniffi::Object;

use super::{Kind, Tag};
use crate::error::Result;
use crate::protocol::event::{PublicKey, Timestamp};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Object)]
#[uniffi::export(Debug, Eq, Ord, Hash)]
pub struct EventId {
    inner: event::EventId,
}

impl Deref for EventId {
    type Target = event::EventId;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl From<event::EventId> for EventId {
    fn from(inner: event::EventId) -> Self {
        Self { inner }
    }
}

#[uniffi::export]
impl EventId {
    #[uniffi::constructor]
    pub fn compute(
        public_key: &PublicKey,
        created_at: &Timestamp,
        kind: &Kind,
        tags: &[Arc<Tag>],
        content: &str,
    ) -> Self {
        let tags: Tags = tags
            .iter()
            .map(|tag| tag.as_ref().deref().clone())
            .collect();

        Self {
            inner: event::EventId::compute(
                public_key.deref(),
                created_at.deref(),
                kind.deref(),
                &tags,
                content,
            ),
        }
    }

    /// Try to parse event ID from `hex`, `bech32` or [NIP21](https://github.com/nostr-protocol/nips/blob/master/21.md) uri
    #[uniffi::constructor]
    pub fn parse(id: &str) -> Result<Self> {
        Ok(Self {
            inner: event::EventId::parse(id)?,
        })
    }

    #[uniffi::constructor]
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        Ok(Self {
            inner: event::EventId::from_slice(bytes)?,
        })
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.inner.as_bytes().to_vec()
    }

    pub fn to_hex(&self) -> String {
        self.inner.to_hex()
    }

    pub fn to_bech32(&self) -> Result<String> {
        Ok(self.inner.to_bech32()?)
    }

    pub fn to_nostr_uri(&self) -> Result<String> {
        Ok(self.inner.to_nostr_uri()?)
    }
}
