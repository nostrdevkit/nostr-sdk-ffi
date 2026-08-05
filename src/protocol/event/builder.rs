// Copyright (c) 2022-2023 Yuki Kishimoto
// Copyright (c) 2023-2025 Rust Nostr Developers
// Distributed under the MIT software license

use std::ops::Deref;
use std::sync::Arc;

use nostr::event;
use uniffi::Object;

use super::Kind;
use crate::protocol::event::{Tag, Timestamp};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Object)]
#[uniffi::export(Debug, Eq, Hash)]
pub struct EventBuilder {
    inner: event::EventBuilder,
}

impl Deref for EventBuilder {
    type Target = event::EventBuilder;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl From<event::EventBuilder> for EventBuilder {
    fn from(inner: event::EventBuilder) -> Self {
        Self { inner }
    }
}

#[uniffi::export]
impl EventBuilder {
    /// Construct a generic event builder.
    #[uniffi::constructor]
    pub fn new(kind: &Kind, content: &str) -> Self {
        Self {
            inner: event::EventBuilder::new(**kind, content),
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
}

impl_finalize_unsigned!(EventBuilder, |e| e.inner.clone());
impl_finalize!(EventBuilder, |e| e.inner.clone());

macro_rules! impl_into_event_builder {
    ($type:ty, |$value:ident| $inner:expr $(,)?) => {
        #[uniffi::export]
        impl $type {
            /// Convert into a generic event builder.
            pub fn into_event_builder(&self) -> crate::protocol::event::EventBuilder {
                let $value: &$type = self;
                let inner = $inner;

                nostr::event::IntoEventBuilder::into_event_builder(inner).into()
            }
        }
    };
}

macro_rules! impl_finalize_unsigned {
    ($type:ty, |$value:ident| $inner:expr $(,)?) => {
        #[uniffi::export]
        impl $type {
            /// Build an unsigned event.
            pub fn finalize_unsigned(
                &self,
                public_key: &crate::protocol::key::PublicKey,
            ) -> crate::protocol::event::UnsignedEvent {
                let $value: &$type = self;
                let inner = $inner;

                nostr::event::FinalizeUnsignedEvent::finalize_unsigned(inner, **public_key).into()
            }
        }
    };
}

macro_rules! impl_finalize {
    ($type:ty, |$value:ident| $inner:expr $(,)?) => {
        #[cfg_attr(not(target_arch = "wasm32"), uniffi::export(async_runtime = "tokio"))]
        #[cfg_attr(target_arch = "wasm32", uniffi::export)]
        impl $type {
            /// Build, sign and return an event.
            pub fn finalize(
                &self,
                signer: std::sync::Arc<dyn crate::protocol::signer::NostrSigner>,
            ) -> crate::error::Result<crate::protocol::event::Event> {
                let signer = crate::protocol::signer::IntermediateNostrSigner::new(signer);

                let $value: &$type = self;
                let inner = $inner;

                let event = nostr::event::FinalizeEvent::finalize(inner, &signer)?;

                Ok(event.into())
            }

            /// Build, sign and return an event asynchronously.
            pub async fn finalize_async(
                &self,
                signer: std::sync::Arc<dyn crate::protocol::signer::AsyncNostrSigner>,
            ) -> crate::error::Result<crate::protocol::event::Event> {
                let signer = crate::protocol::signer::IntermediateAsyncNostrSigner::new(signer);

                let $value: &$type = self;
                let inner = $inner;

                let event =
                    nostr::event::FinalizeEventAsync::finalize_async(inner, &signer).await?;

                Ok(event.into())
            }
        }
    };
}

pub(crate) use impl_finalize;
pub(crate) use impl_finalize_unsigned;
pub(crate) use impl_into_event_builder;
