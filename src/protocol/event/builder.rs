// Copyright (c) 2022-2023 Yuki Kishimoto
// Copyright (c) 2023-2025 Rust Nostr Developers
// Distributed under the MIT software license

use std::ops::Deref;
use std::sync::Arc;

use nostr::event::{FinalizeEvent, FinalizeEventAsync, FinalizeUnsignedEvent};
use uniffi::Object;

use super::{Event, Kind};
use crate::error::Result;
use crate::protocol::event::{PublicKey, Tag, Timestamp, UnsignedEvent};
use crate::protocol::signer::{
    AsyncNostrSigner, IntermediateAsyncNostrSigner, IntermediateNostrSigner, NostrSigner,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Object)]
#[uniffi::export(Debug, Eq, Hash)]
pub struct EventBuilder {
    inner: nostr::EventBuilder,
}

impl From<nostr::EventBuilder> for EventBuilder {
    fn from(inner: nostr::EventBuilder) -> Self {
        Self { inner }
    }
}

impl Deref for EventBuilder {
    type Target = nostr::EventBuilder;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[cfg_attr(not(target_arch = "wasm32"), uniffi::export(async_runtime = "tokio"))]
#[cfg_attr(target_arch = "wasm32", uniffi::export)]
impl EventBuilder {
    /// Construct a generic event builder.
    #[uniffi::constructor]
    pub fn new(kind: &Kind, content: &str) -> Self {
        Self {
            inner: nostr::EventBuilder::new(**kind, content),
        }
    }

    /// Add tags.
    ///
    /// This method extends the current tags, if any.
    pub fn tags(&self, tags: &[Arc<Tag>]) -> Self {
        let mut builder = self.clone();
        let tags = tags.iter().map(|tag| tag.as_ref().deref().clone());
        builder.inner = builder.inner.tags(tags);
        builder
    }

    /// Set a custom `created_at` UNIX timestamp.
    pub fn custom_created_at(&self, created_at: &Timestamp) -> Self {
        let mut builder = self.clone();
        builder.inner = builder.inner.custom_created_at(**created_at);
        builder
    }

    /// Build an unsigned event.
    pub fn finalize_unsigned(&self, public_key: &PublicKey) -> UnsignedEvent {
        self.inner.clone().finalize_unsigned(**public_key).into()
    }

    /// Build, sign and return an event.
    pub fn finalize(&self, signer: Arc<dyn NostrSigner>) -> Result<Event> {
        let signer = IntermediateNostrSigner::new(signer);
        let event = self.inner.clone().finalize(&signer)?;
        Ok(event.into())
    }

    /// Build, sign and return an event asynchronously.
    pub async fn finalize_async(&self, signer: Arc<dyn AsyncNostrSigner>) -> Result<Event> {
        let signer = IntermediateAsyncNostrSigner::new(signer);
        let event = self.inner.clone().finalize_async(&signer).await?;
        Ok(event.into())
    }
}
