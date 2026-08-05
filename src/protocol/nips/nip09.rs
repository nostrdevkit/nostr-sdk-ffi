// Copyright (c) 2023-2025 Rust Nostr Developers
// Distributed under the MIT software license

use std::ops::Deref;
use std::sync::Arc;

use nostr::nips::nip09;
use uniffi::Record;

use super::nip01::Coordinate;
use crate::protocol::event::EventId;
use crate::protocol::event::builder::{
    impl_finalize, impl_finalize_unsigned, impl_into_event_builder,
};

/// Event deletion request
#[derive(Record)]
pub struct EventDeletionRequest {
    /// Event IDs
    pub ids: Vec<Arc<EventId>>,
    /// Event coordinates
    pub coordinates: Vec<Arc<Coordinate>>,
    /// Optional reason
    #[uniffi(default = None)]
    pub reason: Option<String>,
}

impl From<EventDeletionRequest> for nip09::EventDeletionRequest {
    fn from(request: EventDeletionRequest) -> Self {
        Self {
            ids: request.ids.into_iter().map(|id| **id).collect(),
            coordinates: request
                .coordinates
                .into_iter()
                .map(|c| c.as_ref().deref().clone())
                .collect(),
            reason: request.reason,
        }
    }
}

impl_into_event_builder!(EventDeletionRequest, nip09::EventDeletionRequest);
impl_finalize_unsigned!(EventDeletionRequest, nip09::EventDeletionRequest);
impl_finalize!(EventDeletionRequest, nip09::EventDeletionRequest);
