// Copyright (c) 2022-2023 Yuki Kishimoto
// Copyright (c) 2023-2025 Rust Nostr Developers
// Distributed under the MIT software license

use std::fmt;
use std::sync::Arc;

use nostr_sdk::prelude;
use uniffi::Enum;

use super::{NostrDatabaseFeatures, SaveEventStatus};
use crate::error::Result;
use crate::protocol::event::{Event, EventId};
use crate::protocol::filter::Filter;

#[derive(Enum)]
pub enum DatabaseEventStatus {
    Saved,
    Deleted,
    NotExistent,
}

impl From<DatabaseEventStatus> for prelude::DatabaseEventStatus {
    fn from(value: DatabaseEventStatus) -> Self {
        match value {
            DatabaseEventStatus::Saved => Self::Saved,
            DatabaseEventStatus::Deleted => Self::Deleted,
            DatabaseEventStatus::NotExistent => Self::NotExistent,
        }
    }
}

impl From<prelude::DatabaseEventStatus> for DatabaseEventStatus {
    fn from(value: prelude::DatabaseEventStatus) -> Self {
        match value {
            prelude::DatabaseEventStatus::Saved => Self::Saved,
            prelude::DatabaseEventStatus::Deleted => Self::Deleted,
            prelude::DatabaseEventStatus::NotExistent => Self::NotExistent,
        }
    }
}

#[uniffi::export(with_foreign)]
#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
pub trait NostrDatabase: Send + Sync {
    /// Name of backend
    fn backend(&self) -> String;

    /// Gets the database features
    fn features(&self) -> NostrDatabaseFeatures;

    /// Save [`Event`] into store
    ///
    /// **This method assumes that [`Event`] was already verified**
    async fn save_event(&self, event: Arc<Event>) -> Result<Option<Arc<SaveEventStatus>>>;

    /// Check event status by ID
    ///
    /// Check if the event is saved, deleted or not existent.
    async fn check_id(&self, event_id: Arc<EventId>) -> Result<DatabaseEventStatus>;

    /// Get event by ID
    async fn event_by_id(&self, event_id: Arc<EventId>) -> Result<Option<Arc<Event>>>;

    /// Count the number of [`Event`] found by filter
    ///
    /// Use `Filter::new()` or `Filter::default()` to count all events.
    async fn count(&self, filter: Arc<Filter>) -> Result<u64>;

    /// Query store with filter
    async fn query(&self, filter: Arc<Filter>) -> Result<Vec<Arc<Event>>>;

    /// Delete all events that match the `Filter`
    async fn delete_events(&self, filter: Arc<Filter>) -> Result<()>;

    /// Wipe all data
    async fn wipe(&self) -> Result<()>;
}

pub(crate) struct IntermediateNostrDatabase {
    pub(crate) inner: Arc<dyn NostrDatabase>,
}

impl fmt::Debug for IntermediateNostrDatabase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("IntermediateNostrDatabase").finish()
    }
}

mod inner {
    use std::collections::BTreeSet;
    use std::future::Future;
    use std::ops::Deref;
    use std::pin::Pin;
    use std::sync::Arc;

    use nostr_database::error::Error;
    use nostr_sdk::prelude::*;

    use super::IntermediateNostrDatabase;
    use crate::error::MiddleError;
    use crate::future::assume_send;

    impl NostrDatabase for IntermediateNostrDatabase {
        fn backend(&self) -> &'static str {
            self.inner.backend().leak()
        }

        fn features(&self) -> Features {
            self.inner.features().into()
        }

        fn save_event<'a>(
            &'a self,
            event: &'a Event,
        ) -> Pin<Box<dyn Future<Output = Result<SaveEventStatus, Error>> + Send + 'a>> {
            Box::pin(async move {
                let status = assume_send(self.inner.save_event(Arc::new(event.to_owned().into())))
                    .await
                    .map_err(|e| Error::other(MiddleError::from(e)))?
                    .ok_or_else(|| {
                        Error::other(MiddleError::new("Received null instead of SaveEventStatus"))
                    })?;
                Ok(status.inner)
            })
        }

        fn check_id<'a>(
            &'a self,
            event_id: &'a EventId,
        ) -> Pin<Box<dyn Future<Output = Result<DatabaseEventStatus, Error>> + Send + 'a>> {
            Box::pin(async move {
                assume_send(self.inner.check_id(Arc::new((*event_id).into())))
                    .await
                    .map(|s| s.into())
                    .map_err(|e| Error::other(MiddleError::from(e)))
            })
        }

        fn event_by_id<'a>(
            &'a self,
            event_id: &'a EventId,
        ) -> Pin<Box<dyn Future<Output = Result<Option<Event>, Error>> + Send + 'a>> {
            Box::pin(async move {
                Ok(
                    assume_send(self.inner.event_by_id(Arc::new((*event_id).into())))
                        .await
                        .map_err(|e| Error::other(MiddleError::from(e)))?
                        .map(|e| e.as_ref().deref().clone()),
                )
            })
        }

        fn count(
            &self,
            filter: Filter,
        ) -> Pin<Box<dyn Future<Output = Result<usize, Error>> + Send + '_>> {
            Box::pin(async move {
                let res = assume_send(self.inner.count(Arc::new(filter.into())))
                    .await
                    .map_err(|e| Error::other(MiddleError::from(e)))?;
                Ok(res as usize)
            })
        }

        fn query(
            &self,
            filter: Filter,
        ) -> Pin<Box<dyn Future<Output = Result<BTreeSet<Event>, Error>> + Send + '_>> {
            Box::pin(async move {
                let mut events = BTreeSet::new();

                let output = assume_send(self.inner.query(Arc::new(filter.into())))
                    .await
                    .map_err(|e| Error::other(MiddleError::from(e)))?;

                // Extend events
                events.extend(output.into_iter().map(|e| e.as_ref().deref().clone()));

                Ok(events)
            })
        }

        fn delete(
            &self,
            filter: Filter,
        ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + '_>> {
            Box::pin(async move {
                assume_send(self.inner.delete_events(Arc::new(filter.into())))
                    .await
                    .map_err(|e| Error::other(MiddleError::from(e)))
            })
        }

        fn wipe(&self) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + '_>> {
            Box::pin(async move {
                assume_send(self.inner.wipe())
                    .await
                    .map_err(|e| Error::other(MiddleError::from(e)))
            })
        }
    }
}
