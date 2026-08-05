// Copyright (c) 2023-2025 Rust Nostr Developers
// Distributed under the MIT software license

use std::fmt;
use std::ops::Deref;
use std::str::FromStr;
use std::sync::Arc;

use nostr::nips::nip47;
use uniffi::{Enum, Object, Record};

use crate::error::Result;
use crate::protocol::key::{PublicKey, SecretKey};
use crate::protocol::types::{RelayUrl, Timestamp};
use crate::protocol::util::JsonValue;

/// Method
#[derive(Enum)]
pub enum Method {
    /// Pay Invoice
    PayInvoice,
    /// Pay Keysend
    PayKeysend,
    /// Make Invoice
    MakeInvoice,
    /// Lookup Invoice
    LookupInvoice,
    /// List transactions
    ListTransactions,
    /// Get Balance
    GetBalance,
    /// Get Info
    GetInfo,
    /// Make Hold Invoice
    MakeHoldInvoice,
    /// Cancel Hold Invoice
    CancelHoldInvoice,
    /// Settle Hold Invoice
    SettleHoldInvoice,
    /// Unknown method
    Unknown { unknown: String },
}

impl From<nip47::Method> for Method {
    fn from(value: nip47::Method) -> Self {
        match value {
            nip47::Method::PayInvoice => Self::PayInvoice,
            nip47::Method::PayKeysend => Self::PayKeysend,
            nip47::Method::MakeInvoice => Self::MakeInvoice,
            nip47::Method::LookupInvoice => Self::LookupInvoice,
            nip47::Method::ListTransactions => Self::ListTransactions,
            nip47::Method::GetBalance => Self::GetBalance,
            nip47::Method::GetInfo => Self::GetInfo,
            nip47::Method::MakeHoldInvoice => Self::MakeHoldInvoice,
            nip47::Method::CancelHoldInvoice => Self::CancelHoldInvoice,
            nip47::Method::SettleHoldInvoice => Self::SettleHoldInvoice,
            nip47::Method::Unknown(unknown) => Self::Unknown { unknown },
        }
    }
}

impl From<Method> for nip47::Method {
    fn from(value: Method) -> Self {
        match value {
            Method::PayInvoice => Self::PayInvoice,
            Method::PayKeysend => Self::PayKeysend,
            Method::MakeInvoice => Self::MakeInvoice,
            Method::LookupInvoice => Self::LookupInvoice,
            Method::ListTransactions => Self::ListTransactions,
            Method::GetBalance => Self::GetBalance,
            Method::GetInfo => Self::GetInfo,
            Method::MakeHoldInvoice => Self::MakeHoldInvoice,
            Method::CancelHoldInvoice => Self::CancelHoldInvoice,
            Method::SettleHoldInvoice => Self::SettleHoldInvoice,
            Method::Unknown { unknown } => Self::Unknown(unknown),
        }
    }
}

/// Pay Invoice Request
#[derive(Record)]
pub struct PayInvoiceRequest {
    /// Optional id
    pub id: Option<String>,
    /// Request invoice
    pub invoice: String,
    /// Optional amount in millisatoshis
    pub amount: Option<u64>,
}

impl From<nip47::PayInvoiceRequest> for PayInvoiceRequest {
    fn from(value: nip47::PayInvoiceRequest) -> Self {
        Self {
            id: value.id,
            invoice: value.invoice,
            amount: value.amount,
        }
    }
}

impl From<PayInvoiceRequest> for nip47::PayInvoiceRequest {
    fn from(value: PayInvoiceRequest) -> Self {
        Self {
            id: value.id,
            invoice: value.invoice,
            amount: value.amount,
        }
    }
}

/// TLVs to be added to the keysend payment
#[derive(Record)]
pub struct KeysendTLVRecord {
    /// TLV type
    pub tlv_type: u64,
    /// TLV value
    pub value: String,
}

impl From<nip47::KeysendTLVRecord> for KeysendTLVRecord {
    fn from(value: nip47::KeysendTLVRecord) -> Self {
        Self {
            tlv_type: value.tlv_type,
            value: value.value,
        }
    }
}

impl From<KeysendTLVRecord> for nip47::KeysendTLVRecord {
    fn from(value: KeysendTLVRecord) -> Self {
        Self {
            tlv_type: value.tlv_type,
            value: value.value,
        }
    }
}

/// Pay Invoice Request
#[derive(Record)]
pub struct PayKeysendRequest {
    /// Optional id
    pub id: Option<String>,
    /// Amount in millisatoshis
    pub amount: u64,
    /// Receiver's node id
    pub pubkey: String,
    /// Optional preimage
    pub preimage: Option<String>,
    /// Optional TLVs to be added to the keysend payment
    pub tlv_records: Vec<KeysendTLVRecord>,
}

impl From<nip47::PayKeysendRequest> for PayKeysendRequest {
    fn from(value: nip47::PayKeysendRequest) -> Self {
        Self {
            id: value.id,
            amount: value.amount,
            pubkey: value.pubkey,
            preimage: value.preimage,
            tlv_records: value.tlv_records.into_iter().map(|t| t.into()).collect(),
        }
    }
}

impl From<PayKeysendRequest> for nip47::PayKeysendRequest {
    fn from(value: PayKeysendRequest) -> Self {
        Self {
            id: value.id,
            amount: value.amount,
            pubkey: value.pubkey,
            preimage: value.preimage,
            tlv_records: value.tlv_records.into_iter().map(|t| t.into()).collect(),
        }
    }
}

/// Transaction Type
#[derive(Enum)]
pub enum TransactionType {
    /// Incoming payments
    Incoming,
    /// Outgoing payments
    Outgoing,
}

impl From<TransactionType> for nip47::TransactionType {
    fn from(value: TransactionType) -> Self {
        match value {
            TransactionType::Incoming => Self::Incoming,
            TransactionType::Outgoing => Self::Outgoing,
        }
    }
}

impl From<nip47::TransactionType> for TransactionType {
    fn from(value: nip47::TransactionType) -> Self {
        match value {
            nip47::TransactionType::Incoming => Self::Incoming,
            nip47::TransactionType::Outgoing => Self::Outgoing,
        }
    }
}

/// Transaction State
#[derive(Enum)]
pub enum TransactionState {
    /// Pending
    Pending,
    /// Settled
    Settled,
    /// Expired (for invoices)
    Expired,
    /// Failed (for payments)
    Failed,
    /// Accepted (for hold invoices)
    Accepted,
}

impl From<nip47::TransactionState> for TransactionState {
    fn from(value: nip47::TransactionState) -> Self {
        match value {
            nip47::TransactionState::Pending => Self::Pending,
            nip47::TransactionState::Settled => Self::Settled,
            nip47::TransactionState::Expired => Self::Expired,
            nip47::TransactionState::Failed => Self::Failed,
            nip47::TransactionState::Accepted => Self::Accepted,
        }
    }
}

impl From<TransactionState> for nip47::TransactionState {
    fn from(value: TransactionState) -> Self {
        match value {
            TransactionState::Pending => Self::Pending,
            TransactionState::Settled => Self::Settled,
            TransactionState::Expired => Self::Expired,
            TransactionState::Failed => Self::Failed,
            TransactionState::Accepted => Self::Accepted,
        }
    }
}

/// Make Invoice Request
#[derive(Record)]
pub struct MakeInvoiceRequest {
    /// Amount in millisatoshis
    pub amount: u64,
    /// Invoice description
    pub description: Option<String>,
    /// Invoice description hash
    pub description_hash: Option<String>,
    /// Invoice expiry in seconds
    pub expiry: Option<u64>,
}

impl From<nip47::MakeInvoiceRequest> for MakeInvoiceRequest {
    fn from(value: nip47::MakeInvoiceRequest) -> Self {
        Self {
            amount: value.amount,
            description: value.description,
            description_hash: value.description_hash,
            expiry: value.expiry,
        }
    }
}

impl From<MakeInvoiceRequest> for nip47::MakeInvoiceRequest {
    fn from(value: MakeInvoiceRequest) -> Self {
        Self {
            amount: value.amount,
            description: value.description,
            description_hash: value.description_hash,
            expiry: value.expiry,
        }
    }
}

/// Lookup Invoice Request
#[derive(Record)]
pub struct LookupInvoiceRequest {
    /// Payment hash of invoice
    pub payment_hash: Option<String>,
    /// Bolt11 invoice
    pub invoice: Option<String>,
}

impl From<nip47::LookupInvoiceRequest> for LookupInvoiceRequest {
    fn from(value: nip47::LookupInvoiceRequest) -> Self {
        Self {
            payment_hash: value.payment_hash,
            invoice: value.invoice,
        }
    }
}

impl From<LookupInvoiceRequest> for nip47::LookupInvoiceRequest {
    fn from(value: LookupInvoiceRequest) -> Self {
        Self {
            payment_hash: value.payment_hash,
            invoice: value.invoice,
        }
    }
}

/// List Invoice Request
#[derive(Record)]
pub struct ListTransactionsRequest {
    /// Starting timestamp in seconds since epoch
    pub from: Option<Arc<Timestamp>>,
    /// Ending timestamp in seconds since epoch
    pub until: Option<Arc<Timestamp>>,
    /// Number of invoices to return
    pub limit: Option<u64>,
    /// Offset of the first invoice to return
    pub offset: Option<u64>,
    /// If true, include unpaid invoices
    pub unpaid: Option<bool>,
    /// [`TransactionType::Incoming`] for invoices, [`TransactionType::Outgoing`] for payments, [`None`] for both
    pub transaction_type: Option<TransactionType>,
}

impl From<nip47::ListTransactionsRequest> for ListTransactionsRequest {
    fn from(value: nip47::ListTransactionsRequest) -> Self {
        Self {
            from: value.from.map(|t| Arc::new(t.into())),
            until: value.until.map(|t| Arc::new(t.into())),
            limit: value.limit,
            offset: value.offset,
            unpaid: value.unpaid,
            transaction_type: value.transaction_type.map(|t| t.into()),
        }
    }
}

impl From<ListTransactionsRequest> for nip47::ListTransactionsRequest {
    fn from(value: ListTransactionsRequest) -> Self {
        Self {
            from: value.from.map(|t| **t),
            until: value.until.map(|t| **t),
            limit: value.limit,
            offset: value.offset,
            unpaid: value.unpaid,
            transaction_type: value.transaction_type.map(|t| t.into()),
        }
    }
}

/// NIP47 Response Result
#[derive(Record)]
pub struct PayInvoiceResponse {
    /// Response preimage
    pub preimage: String,
    /// Fees paid
    pub fees_paid: Option<u64>,
}

impl From<nip47::PayInvoiceResponse> for PayInvoiceResponse {
    fn from(value: nip47::PayInvoiceResponse) -> Self {
        Self {
            preimage: value.preimage,
            fees_paid: value.fees_paid,
        }
    }
}

impl From<PayInvoiceResponse> for nip47::PayInvoiceResponse {
    fn from(value: PayInvoiceResponse) -> Self {
        Self {
            preimage: value.preimage,
            fees_paid: value.fees_paid,
        }
    }
}

/// NIP47 Response Result
#[derive(Record)]
pub struct PayKeysendResponse {
    /// Response preimage
    pub preimage: String,
    /// Fees paid
    pub fees_paid: Option<u64>,
}

impl From<nip47::PayKeysendResponse> for PayKeysendResponse {
    fn from(value: nip47::PayKeysendResponse) -> Self {
        Self {
            preimage: value.preimage,
            fees_paid: value.fees_paid,
        }
    }
}

impl From<PayKeysendResponse> for nip47::PayKeysendResponse {
    fn from(value: PayKeysendResponse) -> Self {
        Self {
            preimage: value.preimage,
            fees_paid: value.fees_paid,
        }
    }
}

/// NIP47 Response Result
#[derive(Record)]
pub struct MakeInvoiceResponse {
    /// Bolt 11 invoice
    pub invoice: String,
    /// Invoice's payment hash
    pub payment_hash: Option<String>,
    /// Invoice's description
    pub description: Option<String>,
    /// Invoice's description hash
    pub description_hash: Option<String>,
    /// Payment preimage
    pub preimage: Option<String>,
    /// Amount in msats.
    pub amount: Option<u64>,
    /// Creation timestamp in seconds since epoch
    pub created_at: Option<Arc<Timestamp>>,
    /// Expiration timestamp in seconds since epoch
    pub expires_at: Option<Arc<Timestamp>>,
}

impl From<nip47::MakeInvoiceResponse> for MakeInvoiceResponse {
    fn from(value: nip47::MakeInvoiceResponse) -> Self {
        Self {
            invoice: value.invoice,
            payment_hash: value.payment_hash,
            description: value.description,
            description_hash: value.description_hash,
            preimage: value.preimage,
            amount: value.amount,
            created_at: value.created_at.map(|t| Arc::new(t.into())),
            expires_at: value.expires_at.map(|t| Arc::new(t.into())),
        }
    }
}

impl From<MakeInvoiceResponse> for nip47::MakeInvoiceResponse {
    fn from(value: MakeInvoiceResponse) -> Self {
        Self {
            invoice: value.invoice,
            payment_hash: value.payment_hash,
            description: value.description,
            description_hash: value.description_hash,
            preimage: value.preimage,
            amount: value.amount,
            created_at: value.created_at.map(|t| **t),
            expires_at: value.expires_at.map(|t| **t),
        }
    }
}

/// NIP47 Response Result
#[derive(Record)]
pub struct LookupInvoiceResponse {
    /// Transaction type
    pub transaction_type: Option<TransactionType>,
    /// Transaction state.
    pub state: Option<TransactionState>,
    /// Bolt11 invoice
    pub invoice: Option<String>,
    /// Invoice's description
    pub description: Option<String>,
    /// Invoice's description hash
    pub description_hash: Option<String>,
    /// Payment preimage
    pub preimage: Option<String>,
    /// Payment hash
    pub payment_hash: String,
    /// Amount in millisatoshis
    pub amount: u64,
    /// Fees paid in millisatoshis
    pub fees_paid: u64,
    /// Creation timestamp in seconds since epoch
    pub created_at: Arc<Timestamp>,
    /// Expiration timestamp in seconds since epoch
    pub expires_at: Option<Arc<Timestamp>>,
    /// Settled timestamp in seconds since epoch
    pub settled_at: Option<Arc<Timestamp>>,
    /// Optional metadata about the payment
    pub metadata: Option<JsonValue>,
}

impl From<nip47::LookupInvoiceResponse> for LookupInvoiceResponse {
    fn from(value: nip47::LookupInvoiceResponse) -> Self {
        Self {
            transaction_type: value.transaction_type.map(|t| t.into()),
            state: value.state.map(|t| t.into()),
            invoice: value.invoice,
            description: value.description,
            description_hash: value.description_hash,
            preimage: value.preimage,
            payment_hash: value.payment_hash,
            amount: value.amount,
            fees_paid: value.fees_paid,
            created_at: Arc::new(value.created_at.into()),
            expires_at: value.expires_at.map(|t| Arc::new(t.into())),
            settled_at: value.settled_at.map(|t| Arc::new(t.into())),
            metadata: value.metadata.and_then(|m| m.try_into().ok()),
        }
    }
}

impl From<LookupInvoiceResponse> for nip47::LookupInvoiceResponse {
    fn from(value: LookupInvoiceResponse) -> Self {
        Self {
            transaction_type: value.transaction_type.map(|t| t.into()),
            state: value.state.map(|t| t.into()),
            invoice: value.invoice,
            description: value.description,
            description_hash: value.description_hash,
            preimage: value.preimage,
            payment_hash: value.payment_hash,
            amount: value.amount,
            fees_paid: value.fees_paid,
            created_at: **value.created_at,
            expires_at: value.expires_at.map(|t| **t),
            settled_at: value.settled_at.map(|t| **t),
            metadata: value.metadata.and_then(|m| m.try_into().ok()),
        }
    }
}

/// NIP47 Response Result
#[derive(Record)]
pub struct GetBalanceResponse {
    /// Balance amount in msats
    pub balance: u64,
}

impl From<nip47::GetBalanceResponse> for GetBalanceResponse {
    fn from(value: nip47::GetBalanceResponse) -> Self {
        Self {
            balance: value.balance,
        }
    }
}

impl From<GetBalanceResponse> for nip47::GetBalanceResponse {
    fn from(value: GetBalanceResponse) -> Self {
        Self {
            balance: value.balance,
        }
    }
}

/// NIP47 Response Result
#[derive(Record)]
pub struct GetInfoResponse {
    /// The alias of the lightning node
    pub alias: Option<String>,
    /// The color of the current node in hex code format
    pub color: Option<String>,
    /// Lightning Node's public key
    pub pubkey: Option<String>,
    /// Active network
    pub network: Option<String>,
    /// Current block height
    pub block_height: Option<u32>,
    /// Most Recent Block Hash
    pub block_hash: Option<String>,
    /// Available methods for this connection
    pub methods: Vec<Method>,
    /// List of supported notifications for this connection (optional)
    pub notifications: Vec<String>,
}

impl From<nip47::GetInfoResponse> for GetInfoResponse {
    fn from(value: nip47::GetInfoResponse) -> Self {
        Self {
            alias: value.alias,
            color: value.color,
            pubkey: value.pubkey.map(|p| p.to_string()),
            network: value.network,
            block_height: value.block_height,
            block_hash: value.block_hash,
            methods: value.methods.into_iter().map(|m| m.into()).collect(),
            notifications: value.notifications,
        }
    }
}

impl From<GetInfoResponse> for nip47::GetInfoResponse {
    fn from(value: GetInfoResponse) -> Self {
        Self {
            alias: value.alias,
            color: value.color,
            pubkey: value.pubkey.and_then(|p| p.parse().ok()),
            network: value.network,
            block_height: value.block_height,
            block_hash: value.block_hash,
            methods: value.methods.into_iter().map(|m| m.into()).collect(),
            notifications: value.notifications,
        }
    }
}

/// Nostr Connect URI
#[derive(Debug, PartialEq, Eq, Object)]
#[uniffi::export(Debug, Display, Eq)]
pub struct NostrWalletConnectUri {
    inner: nip47::NostrWalletConnectUri,
}

impl fmt::Display for NostrWalletConnectUri {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl Deref for NostrWalletConnectUri {
    type Target = nip47::NostrWalletConnectUri;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl From<nip47::NostrWalletConnectUri> for NostrWalletConnectUri {
    fn from(inner: nip47::NostrWalletConnectUri) -> Self {
        Self { inner }
    }
}

#[uniffi::export]
impl NostrWalletConnectUri {
    /// Create new Nostr Wallet Connect URI
    #[uniffi::constructor]
    pub fn new(
        public_key: &PublicKey,
        relays: Vec<Arc<RelayUrl>>,
        random_secret_key: &SecretKey,
        lud16: Option<String>,
    ) -> Result<Self> {
        Ok(nip47::NostrWalletConnectUri::new(
            **public_key,
            relays
                .into_iter()
                .map(|u| u.as_ref().deref().clone())
                .collect(),
            random_secret_key.deref().clone(),
            lud16,
        )
        .into())
    }

    #[uniffi::constructor]
    pub fn parse(uri: String) -> Result<Self> {
        Ok(nip47::NostrWalletConnectUri::from_str(&uri)?.into())
    }

    /// App Pubkey
    pub fn public_key(&self) -> Arc<PublicKey> {
        Arc::new(self.inner.public_key.into())
    }

    /// URLs of the relays of choice where the `App` is connected and the `Signer` must send and listen for messages.
    pub fn relays(&self) -> Vec<Arc<RelayUrl>> {
        self.inner
            .relays
            .iter()
            .cloned()
            .map(|u| Arc::new(u.into()))
            .collect()
    }

    /// 32-byte randomly generated hex encoded string
    pub fn secret(&self) -> Arc<SecretKey> {
        Arc::new(self.inner.secret.clone().into())
    }

    /// A lightning address that clients can use to automatically setup the lud16 field on the user's profile if they have none configured.
    pub fn lud16(&self) -> Option<String> {
        self.inner.lud16.clone()
    }
}
