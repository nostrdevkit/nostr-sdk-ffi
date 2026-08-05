// Copyright (c) 2022-2023 Yuki Kishimoto
// Copyright (c) 2023-2025 Rust Nostr Developers
// Distributed under the MIT software license

use std::fmt;
use std::ops::Deref;

use nostr::nips::nip46;
use uniffi::Object;

use crate::error::Result;

#[derive(Debug, PartialEq, Eq, Hash, Object)]
#[uniffi::export(Debug, Display, Eq, Hash)]
pub struct NostrConnectUri {
    inner: nip46::NostrConnectUri,
}

impl fmt::Display for NostrConnectUri {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl From<nip46::NostrConnectUri> for NostrConnectUri {
    fn from(inner: nip46::NostrConnectUri) -> Self {
        Self { inner }
    }
}

impl Deref for NostrConnectUri {
    type Target = nip46::NostrConnectUri;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[uniffi::export]
impl NostrConnectUri {
    #[uniffi::constructor]
    pub fn parse(uri: &str) -> Result<Self> {
        Ok(Self {
            inner: nip46::NostrConnectUri::parse(uri)?,
        })
    }
}
