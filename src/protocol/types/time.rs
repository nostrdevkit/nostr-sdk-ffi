// Copyright (c) 2022-2023 Yuki Kishimoto
// Copyright (c) 2023-2025 Rust Nostr Developers
// Distributed under the MIT software license

use std::ops::Deref;
use std::time::Duration;

use nostr::types;
use uniffi::Object;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Object)]
#[uniffi::export(Debug, Eq, Ord, Hash)]
pub struct Timestamp {
    inner: types::Timestamp,
}

impl From<types::Timestamp> for Timestamp {
    fn from(inner: types::Timestamp) -> Self {
        Self { inner }
    }
}

impl Deref for Timestamp {
    type Target = types::Timestamp;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[uniffi::export]
impl Timestamp {
    /// Get UNIX timestamp
    #[uniffi::constructor]
    pub fn now() -> Self {
        Self {
            inner: types::Timestamp::now(),
        }
    }

    #[uniffi::constructor]
    pub fn from_secs(secs: u64) -> Self {
        Self {
            inner: types::Timestamp::from_secs(secs),
        }
    }

    /// The minimum representable timestamp
    #[uniffi::constructor]
    pub fn min() -> Self {
        Self {
            inner: types::Timestamp::min(),
        }
    }

    /// The maximum representable timestamp
    #[uniffi::constructor]
    pub fn max() -> Self {
        Self {
            inner: types::Timestamp::max(),
        }
    }

    /// Add duration to timestamp
    ///
    /// This sums the duration to the current timestamp and returns a new timestamp.
    #[inline]
    pub fn add_duration(&self, duration: Duration) -> Self {
        Self {
            inner: self.inner + duration,
        }
    }

    /// Subtract duration from timestamp
    ///
    /// This subtracts the duration from the current timestamp and returns a new timestamp.
    #[inline]
    pub fn sub_duration(&self, duration: Duration) -> Self {
        Self {
            inner: self.inner - duration,
        }
    }

    /// Get timestamp as [`u64`]
    pub fn as_secs(&self) -> u64 {
        self.inner.as_secs()
    }

    /// Convert [`Timestamp`] to human datetime
    pub fn to_human_datetime(&self) -> String {
        self.inner.to_human_datetime()
    }
}
