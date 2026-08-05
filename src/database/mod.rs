// Copyright (c) 2022-2023 Yuki Kishimoto
// Copyright (c) 2023-2025 Rust Nostr Developers
// Distributed under the MIT software license

use std::sync::Arc;

#[cfg(feature = "lmdb")]
#[cfg(all(feature = "native", not(target_arch = "wasm32")))]
use nostr_lmdb::NostrLmdb as InnerNostrLmdb;
#[cfg(feature = "ndb")]
#[cfg(all(feature = "native", not(target_arch = "wasm32")))]
use nostr_ndb::NdbDatabase as InnerNdbDatabase;
use nostr_sdk::prelude::{self, IntoNostrDatabase};
use uniffi::{Enum, Object, Record};

mod macros;
mod traits;

#[cfg(all(
    feature = "native",
    not(target_arch = "wasm32"),
    any(feature = "lmdb", feature = "ndb")
))]
pub(crate) use self::macros::export_nostr_database;
pub(crate) use self::macros::impl_nostr_database;
use self::traits::IntermediateNostrDatabase;
pub use self::traits::NostrDatabase;
#[cfg(all(
    feature = "native",
    not(target_arch = "wasm32"),
    any(feature = "lmdb", feature = "ndb")
))]
use crate::error::Result;

#[derive(Record)]
pub struct NostrDatabaseFeatures {
    /// Whether the database supports persistent storage.
    pub persistent: bool,
    /// Whether the database supports event expiration (NIP-40)
    ///
    /// When supported, the database will automatically exclude expired events
    /// from query results and/or delete them.
    ///
    /// <https://github.com/nostr-protocol/nips/blob/master/40.md>
    pub event_expiration: bool,
    /// Whether the database supports full-text search (NIP-50)
    ///
    /// <https://github.com/nostr-protocol/nips/blob/master/50.md>
    pub full_text_search: bool,
    /// Whether the database supports the request to vanish (NIP-62)
    ///
    /// <https://github.com/nostr-protocol/nips/blob/master/62.md>
    pub request_to_vanish: bool,
}

impl From<prelude::Features> for NostrDatabaseFeatures {
    fn from(features: prelude::Features) -> Self {
        Self {
            persistent: features.persistent,
            event_expiration: features.event_expiration,
            full_text_search: features.full_text_search,
            request_to_vanish: features.request_to_vanish,
        }
    }
}

impl From<NostrDatabaseFeatures> for prelude::Features {
    fn from(features: NostrDatabaseFeatures) -> Self {
        Self {
            persistent: features.persistent,
            event_expiration: features.event_expiration,
            full_text_search: features.full_text_search,
            request_to_vanish: features.request_to_vanish,
        }
    }
}

/// Reason why event wasn't stored into the database
#[derive(Enum)]
pub enum RejectedReason {
    /// Ephemeral events aren't expected to be stored
    Ephemeral,
    /// The event already exists
    Duplicate,
    /// The event was deleted
    Deleted,
    /// The event is expired
    Expired,
    /// The event was replaced
    Replaced,
    /// Attempt to delete a non-owned event
    InvalidDelete,
    /// The event author vanished before
    Vanished,
    /// Other reason
    Other,
}

impl From<prelude::RejectedReason> for RejectedReason {
    fn from(status: prelude::RejectedReason) -> Self {
        match status {
            prelude::RejectedReason::Ephemeral => Self::Ephemeral,
            prelude::RejectedReason::Duplicate => Self::Duplicate,
            prelude::RejectedReason::Deleted => Self::Deleted,
            prelude::RejectedReason::Expired => Self::Expired,
            prelude::RejectedReason::Replaced => Self::Replaced,
            prelude::RejectedReason::InvalidDelete => Self::InvalidDelete,
            prelude::RejectedReason::Vanished => Self::Vanished,
            prelude::RejectedReason::Other => Self::Other,
        }
    }
}

impl From<RejectedReason> for prelude::RejectedReason {
    fn from(status: RejectedReason) -> Self {
        match status {
            RejectedReason::Ephemeral => Self::Ephemeral,
            RejectedReason::Duplicate => Self::Duplicate,
            RejectedReason::Deleted => Self::Deleted,
            RejectedReason::Expired => Self::Expired,
            RejectedReason::Replaced => Self::Replaced,
            RejectedReason::InvalidDelete => Self::InvalidDelete,
            RejectedReason::Vanished => Self::Vanished,
            RejectedReason::Other => Self::Other,
        }
    }
}

/// Save event status
#[derive(Object)]
pub struct SaveEventStatus {
    inner: prelude::SaveEventStatus,
}

impl From<prelude::SaveEventStatus> for SaveEventStatus {
    fn from(inner: prelude::SaveEventStatus) -> Self {
        Self { inner }
    }
}

#[uniffi::export]
impl SaveEventStatus {
    #[uniffi::constructor]
    pub fn success() -> Self {
        Self {
            inner: prelude::SaveEventStatus::Success,
        }
    }

    #[uniffi::constructor]
    pub fn rejected(reason: RejectedReason) -> Self {
        Self {
            inner: prelude::SaveEventStatus::Rejected(reason.into()),
        }
    }

    /// The event has been successfully saved
    pub fn is_success(&self) -> bool {
        self.inner.is_success()
    }

    /// Get rejection reason, if the event wasn't saved successfully
    pub fn rejection_reason(&self) -> Option<RejectedReason> {
        match self.inner {
            prelude::SaveEventStatus::Success => None,
            prelude::SaveEventStatus::Rejected(reason) => Some(reason.into()),
        }
    }
}

pub(crate) fn into_nostr_database(
    database: Arc<dyn NostrDatabase>,
) -> Arc<dyn prelude::NostrDatabase> {
    IntermediateNostrDatabase { inner: database }.into_nostr_database()
}

struct RustNostrDatabase {
    inner: Arc<dyn prelude::NostrDatabase>,
}

impl_nostr_database!(RustNostrDatabase, |database| database.inner.as_ref());

pub(crate) fn from_nostr_database(
    database: Arc<dyn prelude::NostrDatabase>,
) -> Arc<dyn NostrDatabase> {
    Arc::new(RustNostrDatabase { inner: database })
}

/// LMDB database backend.
#[cfg(all(feature = "lmdb", feature = "native", not(target_arch = "wasm32")))]
#[derive(Object)]
pub struct NostrLmdb {
    inner: InnerNostrLmdb,
}

#[cfg(all(feature = "lmdb", feature = "native", not(target_arch = "wasm32")))]
#[uniffi::export(async_runtime = "tokio")]
impl NostrLmdb {
    /// Open an LMDB database.
    #[uniffi::constructor]
    pub async fn open(path: &str) -> Result<Self> {
        Ok(Self {
            inner: InnerNostrLmdb::open(path).await?,
        })
    }
}

#[cfg(all(feature = "lmdb", feature = "native", not(target_arch = "wasm32")))]
export_nostr_database!(NostrLmdb, |database| &database.inner);

/// nostrdb database backend.
#[cfg(all(feature = "ndb", feature = "native", not(target_arch = "wasm32")))]
#[derive(Object)]
pub struct NdbDatabase {
    inner: InnerNdbDatabase,
}

#[cfg(all(feature = "ndb", feature = "native", not(target_arch = "wasm32")))]
#[uniffi::export]
impl NdbDatabase {
    /// Open a nostrdb database.
    #[uniffi::constructor]
    pub fn open(path: &str) -> Result<Self> {
        Ok(Self {
            inner: InnerNdbDatabase::open(path)?,
        })
    }
}

#[cfg(all(feature = "ndb", feature = "native", not(target_arch = "wasm32")))]
export_nostr_database!(NdbDatabase, |database| &database.inner);
