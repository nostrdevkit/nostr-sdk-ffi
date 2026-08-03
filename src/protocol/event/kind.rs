// Copyright (c) 2022-2023 Yuki Kishimoto
// Copyright (c) 2023-2025 Rust Nostr Developers
// Distributed under the MIT software license

use std::fmt;
use std::ops::Deref;

use nostr::event;
use uniffi::{Enum, Object};

/// Event Kind
#[derive(Debug, PartialEq, Eq, Hash, Object)]
#[uniffi::export(Debug, Display, Eq, Hash)]
pub struct Kind {
    inner: event::Kind,
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl Deref for Kind {
    type Target = event::Kind;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl From<event::Kind> for Kind {
    fn from(inner: event::Kind) -> Self {
        Self { inner }
    }
}

#[uniffi::export]
impl Kind {
    #[uniffi::constructor]
    pub fn new(kind: u16) -> Self {
        Self {
            inner: event::Kind::from_u16(kind),
        }
    }

    #[uniffi::constructor]
    pub fn from_std(e: KindStandard) -> Self {
        Self { inner: e.into() }
    }

    /// Get as 16-bit unsigned integer
    pub fn as_u16(&self) -> u16 {
        self.inner.as_u16()
    }

    pub fn as_std(&self) -> Option<KindStandard> {
        convert(self.inner)
    }

    /// Check if it's regular
    ///
    /// Regular means that event is expected to be stored by relays.
    ///
    /// <https://github.com/nostr-protocol/nips/blob/master/01.md>
    pub fn is_regular(&self) -> bool {
        self.inner.is_regular()
    }

    /// Check if it's replaceable
    ///
    /// Replaceable means that, for each combination of `pubkey` and `kind`,
    /// only the latest event MUST be stored by relays, older versions MAY be discarded.
    ///
    /// <https://github.com/nostr-protocol/nips/blob/master/01.md>
    pub fn is_replaceable(&self) -> bool {
        self.inner.is_replaceable()
    }

    /// Check if it's ephemeral
    ///
    /// Ephemeral means that event is not expected to be stored by relays.
    ///
    /// <https://github.com/nostr-protocol/nips/blob/master/01.md>
    pub fn is_ephemeral(&self) -> bool {
        self.inner.is_ephemeral()
    }

    /// Check if it's addressable
    ///
    /// Addressable means that, for each combination of `pubkey`, `kind` and the `d` tag's first value,
    /// only the latest event MUST be stored by relays, older versions MAY be discarded.
    ///
    /// <https://github.com/nostr-protocol/nips/blob/master/01.md>
    pub fn is_addressable(&self) -> bool {
        self.inner.is_addressable()
    }
}

/// Standardized kind
#[derive(Enum)]
pub enum KindStandard {
    /// Metadata (NIP01 and NIP05)
    Metadata,
    /// Short Text Note (NIP01)
    TextNote,
    /// Contacts (NIP02)
    ContactList,
    /// OpenTimestamps Attestations (NIP03)
    OpenTimestamps,
    /// Event Deletion (NIP09)
    EventDeletion,
    /// Repost (NIP18)
    Repost,
    /// Generic Repost (NIP18)
    GenericRepost,
    /// Comment (NIP22)
    Comment,
    /// Reaction (NIP25)
    Reaction,
    /// Badge Award (NIP58)
    BadgeAward,
    /// Blossom Authorization
    BlossomAuth,
    /// Channel Creation (NIP28)
    ChannelCreation,
    /// Channel Metadata (NIP28)
    ChannelMetadata,
    /// Channel Message (NIP28)
    ChannelMessage,
    /// Channel Hide Message (NIP28)
    ChannelHideMessage,
    /// Channel Mute User (NIP28)
    ChannelMuteUser,
    /// Git Patch
    ///
    /// <https://github.com/nostr-protocol/nips/blob/master/34.md>
    GitPatch,
    /// Git Issue
    ///
    /// <https://github.com/nostr-protocol/nips/blob/master/34.md>
    GitIssue,
    /// Git Reply
    ///
    /// <https://github.com/nostr-protocol/nips/blob/master/34.md>
    GitReply,
    /// Open Status of Git Patch or Issue
    ///
    /// <https://github.com/nostr-protocol/nips/blob/master/34.md>
    GitStatusOpen,
    /// Applied / Merged Status of Git Patch or Resolved Status of Git Issue
    ///
    /// <https://github.com/nostr-protocol/nips/blob/master/34.md>
    GitStatusApplied,
    /// Closed Status of Git Patch or Issue
    ///
    /// <https://github.com/nostr-protocol/nips/blob/master/34.md>
    GitStatusClosed,
    /// Draft Status of Git Patch or Issue
    ///
    /// <https://github.com/nostr-protocol/nips/blob/master/34.md>
    GitStatusDraft,
    /// Torrent
    ///
    /// <https://github.com/nostr-protocol/nips/blob/master/35.md>
    Torrent,
    /// Torrent comment
    ///
    /// <https://github.com/nostr-protocol/nips/blob/master/35.md>
    TorrentComment,
    /// Label
    ///
    /// <https://github.com/nostr-protocol/nips/blob/master/32.md>
    Label,
    /// Wallet Service Info (NIP47)
    WalletConnectInfo,
    /// Wallet Connect notification (NIP47)
    WalletConnectNotification,
    /// Wallet Connect notification encrypted with NIP44 v2 (NIP47)
    WalletConnectNotificationNip44V2,
    /// Reporting (NIP56)
    Reporting,
    /// Zap Private Message (NIP57)
    ZapPrivateMessage,
    /// Zap Request (NIP57)
    ZapRequest,
    /// Zap Receipt (NIP57)
    ZapReceipt,
    /// Mute List
    ///
    /// <https://github.com/nostr-protocol/nips/blob/master/51.md>
    MuteList,
    /// Pin List
    ///
    /// <https://github.com/nostr-protocol/nips/blob/master/51.md>
    PinList,
    /// Bookmarks
    ///
    /// <https://github.com/nostr-protocol/nips/blob/master/51.md>
    Bookmarks,
    /// Communities
    ///
    /// <https://github.com/nostr-protocol/nips/blob/master/51.md>
    Communities,
    /// Public Chats
    ///
    /// <https://github.com/nostr-protocol/nips/blob/master/51.md>
    PublicChats,
    /// Blocked Relays
    ///
    /// <https://github.com/nostr-protocol/nips/blob/master/51.md>
    BlockedRelays,
    /// Search Relays
    ///
    /// <https://github.com/nostr-protocol/nips/blob/master/51.md>
    SearchRelays,
    /// Simple Groups
    ///
    /// <https://github.com/nostr-protocol/nips/blob/master/51.md>
    SimpleGroups,
    /// Interests
    ///
    /// <https://github.com/nostr-protocol/nips/blob/master/51.md>
    Interests,
    /// Emojis
    ///
    /// <https://github.com/nostr-protocol/nips/blob/master/51.md>
    Emojis,
    /// Follow Set
    ///
    /// <https://github.com/nostr-protocol/nips/blob/master/51.md>
    FollowSet,
    /// Relay Set
    ///
    /// <https://github.com/nostr-protocol/nips/blob/master/51.md>
    RelaySet,
    /// Bookmark Set
    ///
    /// <https://github.com/nostr-protocol/nips/blob/master/51.md>
    BookmarkSet,
    /// Articles Curation Set
    ///
    /// <https://github.com/nostr-protocol/nips/blob/master/51.md>
    ArticlesCurationSet,
    /// Videos Curation Set
    ///
    /// <https://github.com/nostr-protocol/nips/blob/master/51.md>
    VideosCurationSet,
    /// Interest Set
    ///
    /// <https://github.com/nostr-protocol/nips/blob/master/51.md>
    InterestSet,
    /// Emoji Set
    ///
    /// <https://github.com/nostr-protocol/nips/blob/master/51.md>
    EmojiSet,
    /// Release Artifact Set
    ///
    /// <https://github.com/nostr-protocol/nips/blob/master/51.md>
    ReleaseArtifactSet,
    /// Relay List Metadata (NIP65)
    RelayList,
    /// Peer-to-peer Order events
    ///
    /// <https://github.com/nostr-protocol/nips/blob/master/69.md>
    PeerToPeerOrder,
    /// Request to Vanish (NIP62)
    ///
    /// <https://github.com/nostr-protocol/nips/blob/master/62.md>
    RequestToVanish,
    /// Client Authentication (NIP42)
    Authentication,
    /// Wallet Connect Request (NIP47)
    WalletConnectRequest,
    /// Wallet Connect Response (NIP47)
    WalletConnectResponse,
    /// Nostr Connect (NIP46)
    NostrConnect,
    /// Live Event (NIP53)
    LiveEvent,
    /// Live Event Message (NIP53)
    LiveEventMessage,
    /// Profile Badges (NIP58)
    ProfileBadges,
    /// Badge Definition (NIP58)
    BadgeDefinition,
    /// Seal (NIP59)
    Seal,
    /// Gift Wrap (NIP59)
    GiftWrap,
    /// Private Direct message
    ///
    /// <https://github.com/nostr-protocol/nips/blob/master/17.md>
    PrivateDirectMessage,
    /// Inbox Relays (NIP17)
    InboxRelays,
    /// MLS Key Package Relays (NIP104)
    MlsKeyPackageRelays,
    /// MLS Key Package (NIP104)
    MlsKeyPackage,
    /// MLS Welcome (NIP104)
    MlsWelcome,
    /// MLS Group Message (NIP104)
    MlsGroupMessage,
    /// Long-form Text Note (NIP23)
    LongFormTextNote,
    /// Git Repository Announcement
    ///
    /// <https://github.com/nostr-protocol/nips/blob/master/34.md>
    GitRepoAnnouncement,
    /// Application-specific Data (NIP78)
    ApplicationSpecificData,
    /// File Metadata (NIP94)
    FileMetadata,
    /// HTTP Auth (NIP98)
    HttpAuth,
    /// Set stall (NIP15)
    SetStall,
    /// Set product (NIP15)
    SetProduct,
    /// Job Feedback (NIP90)
    JobFeedback,
    /// User Status
    ///
    /// <https://github.com/nostr-protocol/nips/blob/master/38.md>
    UserStatus,
    /// Cashu Wallet
    ///
    /// <https://github.com/nostr-protocol/nips/blob/master/60.md>
    CashuWallet,
    /// Cashu Wallet Unspent Proof
    ///
    /// <https://github.com/nostr-protocol/nips/blob/master/60.md>
    CashuWalletUnspentProof,
    /// Cashu Wallet Spending History
    ///
    /// <https://github.com/nostr-protocol/nips/blob/master/60.md>
    CashuWalletSpendingHistory,
    /// Cashu Wallet Redeeming a quote
    CashuWalletQuote,
    /// Cashu Nut Zap informational event
    CashuNutZapInfo,
    /// Cashu Nut Zap
    CashuNutZap,
    /// Code Snippet
    ///
    /// <https://github.com/nostr-protocol/nips/blob/master/C0.md>
    CodeSnippet,
    /// Poll
    ///
    /// <https://github.com/nostr-protocol/nips/blob/master/88.md>
    Poll,
    /// Poll response
    ///
    /// <https://github.com/nostr-protocol/nips/blob/master/88.md>
    PollResponse,
    RepoState,
    VoiceMessage,
    VoiceMessageReply,
    Thread,
    WebBookmark,
    ChatMessage,
}

fn convert(k: event::Kind) -> Option<KindStandard> {
    match k {
        event::Kind::Metadata => Some(KindStandard::Metadata),
        event::Kind::TextNote => Some(KindStandard::TextNote),
        event::Kind::RecommendRelay | event::Kind::EncryptedDirectMessage => None,
        event::Kind::ContactList => Some(KindStandard::ContactList),
        event::Kind::OpenTimestamps => Some(KindStandard::OpenTimestamps),
        event::Kind::EventDeletion => Some(KindStandard::EventDeletion),
        event::Kind::Repost => Some(KindStandard::Repost),
        event::Kind::GenericRepost => Some(KindStandard::GenericRepost),
        event::Kind::Comment => Some(KindStandard::Comment),
        event::Kind::Reaction => Some(KindStandard::Reaction),
        event::Kind::BadgeAward => Some(KindStandard::BadgeAward),
        event::Kind::ChannelCreation => Some(KindStandard::ChannelCreation),
        event::Kind::ChannelMetadata => Some(KindStandard::ChannelMetadata),
        event::Kind::ChannelMessage => Some(KindStandard::ChannelMessage),
        event::Kind::ChannelHideMessage => Some(KindStandard::ChannelHideMessage),
        event::Kind::ChannelMuteUser => Some(KindStandard::ChannelMuteUser),
        event::Kind::PublicChatReserved45
        | event::Kind::PublicChatReserved46
        | event::Kind::PublicChatReserved47
        | event::Kind::PublicChatReserved48
        | event::Kind::PublicChatReserved49 => None,
        event::Kind::GitPatch => Some(KindStandard::GitPatch),
        event::Kind::GitIssue => Some(KindStandard::GitIssue),
        event::Kind::GitReply => Some(KindStandard::GitReply),
        event::Kind::GitStatusOpen => Some(KindStandard::GitStatusOpen),
        event::Kind::GitStatusApplied => Some(KindStandard::GitStatusApplied),
        event::Kind::GitStatusClosed => Some(KindStandard::GitStatusClosed),
        event::Kind::GitStatusDraft => Some(KindStandard::GitStatusDraft),
        event::Kind::Label => Some(KindStandard::Label),
        event::Kind::WalletConnectInfo => Some(KindStandard::WalletConnectInfo),
        event::Kind::WalletConnectNotification => Some(KindStandard::WalletConnectNotification),
        event::Kind::WalletConnectNotificationNip44V2 => {
            Some(KindStandard::WalletConnectNotificationNip44V2)
        }
        event::Kind::Reporting => Some(KindStandard::Reporting),
        event::Kind::ZapPrivateMessage => Some(KindStandard::ZapPrivateMessage),
        event::Kind::ZapRequest => Some(KindStandard::ZapRequest),
        event::Kind::ZapReceipt => Some(KindStandard::ZapReceipt),
        event::Kind::MuteList => Some(KindStandard::MuteList),
        event::Kind::PinList => Some(KindStandard::PinList),
        event::Kind::Bookmarks => Some(KindStandard::Bookmarks),
        event::Kind::Communities => Some(KindStandard::Communities),
        event::Kind::PublicChats => Some(KindStandard::PublicChats),
        event::Kind::BlockedRelays => Some(KindStandard::BlockedRelays),
        event::Kind::SearchRelays => Some(KindStandard::SearchRelays),
        event::Kind::SimpleGroups => Some(KindStandard::SimpleGroups),
        event::Kind::Interests => Some(KindStandard::Interests),
        event::Kind::Emojis => Some(KindStandard::Emojis),
        event::Kind::FollowSet => Some(KindStandard::FollowSet),
        event::Kind::RelaySet => Some(KindStandard::RelaySet),
        event::Kind::BookmarkSet => Some(KindStandard::BookmarkSet),
        event::Kind::ArticlesCurationSet => Some(KindStandard::ArticlesCurationSet),
        event::Kind::VideosCurationSet => Some(KindStandard::VideosCurationSet),
        event::Kind::InterestSet => Some(KindStandard::InterestSet),
        event::Kind::EmojiSet => Some(KindStandard::EmojiSet),
        event::Kind::ReleaseArtifactSet => Some(KindStandard::ReleaseArtifactSet),
        event::Kind::RelayList => Some(KindStandard::RelayList),
        event::Kind::Authentication => Some(KindStandard::Authentication),
        event::Kind::WalletConnectRequest => Some(KindStandard::WalletConnectRequest),
        event::Kind::WalletConnectResponse => Some(KindStandard::WalletConnectResponse),
        event::Kind::NostrConnect => Some(KindStandard::NostrConnect),
        event::Kind::LiveEvent => Some(KindStandard::LiveEvent),
        event::Kind::LiveEventMessage => Some(KindStandard::LiveEventMessage),
        event::Kind::ProfileBadges => Some(KindStandard::ProfileBadges),
        event::Kind::BadgeDefinition => Some(KindStandard::BadgeDefinition),
        event::Kind::Seal => Some(KindStandard::Seal),
        event::Kind::GiftWrap => Some(KindStandard::GiftWrap),
        event::Kind::PrivateDirectMessage => Some(KindStandard::PrivateDirectMessage),
        event::Kind::LongFormTextNote => Some(KindStandard::LongFormTextNote),
        event::Kind::GitRepoAnnouncement => Some(KindStandard::GitRepoAnnouncement),
        event::Kind::ApplicationSpecificData => Some(KindStandard::ApplicationSpecificData),
        event::Kind::FileMetadata => Some(KindStandard::FileMetadata),
        event::Kind::HttpAuth => Some(KindStandard::HttpAuth),
        event::Kind::SetStall => Some(KindStandard::SetStall),
        event::Kind::SetProduct => Some(KindStandard::SetProduct),
        event::Kind::JobFeedback => Some(KindStandard::JobFeedback),
        event::Kind::InboxRelays => Some(KindStandard::InboxRelays),
        event::Kind::MlsKeyPackageRelays => Some(KindStandard::MlsKeyPackageRelays),
        event::Kind::MlsKeyPackage => Some(KindStandard::MlsKeyPackage),
        event::Kind::MlsWelcome => Some(KindStandard::MlsWelcome),
        event::Kind::MlsGroupMessage => Some(KindStandard::MlsGroupMessage),
        event::Kind::Torrent => Some(KindStandard::Torrent),
        event::Kind::TorrentComment => Some(KindStandard::TorrentComment),
        event::Kind::PeerToPeerOrder => Some(KindStandard::PeerToPeerOrder),
        event::Kind::RequestToVanish => Some(KindStandard::RequestToVanish),
        event::Kind::UserStatus => Some(KindStandard::UserStatus),
        event::Kind::CashuWallet => Some(KindStandard::CashuWallet),
        event::Kind::CashuWalletUnspentProof => Some(KindStandard::CashuWalletUnspentProof),
        event::Kind::CashuWalletSpendingHistory => Some(KindStandard::CashuWalletSpendingHistory),
        event::Kind::CashuWalletQuote => Some(KindStandard::CashuWalletQuote),
        event::Kind::CashuNutZapInfo => Some(KindStandard::CashuNutZapInfo),
        event::Kind::CashuNutZap => Some(KindStandard::CashuNutZap),
        event::Kind::CodeSnippet => Some(KindStandard::CodeSnippet),
        event::Kind::BlossomAuth => Some(KindStandard::BlossomAuth),
        event::Kind::Poll => Some(KindStandard::Poll),
        event::Kind::PollResponse => Some(KindStandard::PollResponse),
        event::Kind::RepoState => Some(KindStandard::RepoState),
        event::Kind::VoiceMessage => Some(KindStandard::VoiceMessage),
        event::Kind::VoiceMessageReply => Some(KindStandard::VoiceMessageReply),
        event::Kind::Thread => Some(KindStandard::Thread),
        event::Kind::WebBookmark => Some(KindStandard::WebBookmark),
        event::Kind::ChatMessage => Some(KindStandard::ChatMessage),
        _ => None,
    }
}

impl From<KindStandard> for event::Kind {
    fn from(value: KindStandard) -> Self {
        match value {
            KindStandard::Metadata => Self::Metadata,
            KindStandard::TextNote => Self::TextNote,
            KindStandard::ContactList => Self::ContactList,
            KindStandard::OpenTimestamps => Self::OpenTimestamps,
            KindStandard::EventDeletion => Self::EventDeletion,
            KindStandard::Repost => Self::Repost,
            KindStandard::GenericRepost => Self::GenericRepost,
            KindStandard::Comment => Self::Comment,
            KindStandard::Reaction => Self::Reaction,
            KindStandard::BadgeAward => Self::BadgeAward,
            KindStandard::BlossomAuth => Self::BlossomAuth,
            KindStandard::ChannelCreation => Self::ChannelCreation,
            KindStandard::ChannelMetadata => Self::ChannelMetadata,
            KindStandard::ChannelMessage => Self::ChannelMessage,
            KindStandard::ChannelHideMessage => Self::ChannelHideMessage,
            KindStandard::ChannelMuteUser => Self::ChannelMuteUser,
            KindStandard::GitPatch => Self::GitPatch,
            KindStandard::GitIssue => Self::GitIssue,
            KindStandard::GitReply => Self::GitReply,
            KindStandard::GitStatusOpen => Self::GitStatusOpen,
            KindStandard::GitStatusApplied => Self::GitStatusApplied,
            KindStandard::GitStatusClosed => Self::GitStatusClosed,
            KindStandard::GitStatusDraft => Self::GitStatusDraft,
            KindStandard::Label => Self::Label,
            KindStandard::WalletConnectInfo => Self::WalletConnectInfo,
            KindStandard::WalletConnectNotification => Self::WalletConnectNotification,
            KindStandard::WalletConnectNotificationNip44V2 => {
                Self::WalletConnectNotificationNip44V2
            }
            KindStandard::Reporting => Self::Reporting,
            KindStandard::ZapPrivateMessage => Self::ZapPrivateMessage,
            KindStandard::ZapRequest => Self::ZapRequest,
            KindStandard::ZapReceipt => Self::ZapReceipt,
            KindStandard::MuteList => Self::MuteList,
            KindStandard::PinList => Self::PinList,
            KindStandard::Bookmarks => Self::Bookmarks,
            KindStandard::Communities => Self::Communities,
            KindStandard::PublicChats => Self::PublicChats,
            KindStandard::BlockedRelays => Self::BlockedRelays,
            KindStandard::SearchRelays => Self::SearchRelays,
            KindStandard::SimpleGroups => Self::SimpleGroups,
            KindStandard::Interests => Self::Interests,
            KindStandard::Emojis => Self::Emojis,
            KindStandard::FollowSet => Self::FollowSet,
            KindStandard::RelaySet => Self::RelaySet,
            KindStandard::BookmarkSet => Self::BookmarkSet,
            KindStandard::ArticlesCurationSet => Self::ArticlesCurationSet,
            KindStandard::VideosCurationSet => Self::VideosCurationSet,
            KindStandard::InterestSet => Self::InterestSet,
            KindStandard::EmojiSet => Self::EmojiSet,
            KindStandard::ReleaseArtifactSet => Self::ReleaseArtifactSet,
            KindStandard::RelayList => Self::RelayList,
            KindStandard::Authentication => Self::Authentication,
            KindStandard::WalletConnectRequest => Self::WalletConnectRequest,
            KindStandard::WalletConnectResponse => Self::WalletConnectResponse,
            KindStandard::NostrConnect => Self::NostrConnect,
            KindStandard::LiveEvent => Self::LiveEvent,
            KindStandard::LiveEventMessage => Self::LiveEventMessage,
            KindStandard::ProfileBadges => Self::ProfileBadges,
            KindStandard::BadgeDefinition => Self::BadgeDefinition,
            KindStandard::Seal => Self::Seal,
            KindStandard::GiftWrap => Self::GiftWrap,
            KindStandard::PrivateDirectMessage => Self::PrivateDirectMessage,
            KindStandard::LongFormTextNote => Self::LongFormTextNote,
            KindStandard::ApplicationSpecificData => Self::ApplicationSpecificData,
            KindStandard::GitRepoAnnouncement => Self::GitRepoAnnouncement,
            KindStandard::FileMetadata => Self::FileMetadata,
            KindStandard::HttpAuth => Self::HttpAuth,
            KindStandard::SetStall => Self::SetStall,
            KindStandard::SetProduct => Self::SetProduct,
            KindStandard::JobFeedback => Self::JobFeedback,
            KindStandard::InboxRelays => Self::InboxRelays,
            KindStandard::MlsKeyPackageRelays => Self::MlsKeyPackageRelays,
            KindStandard::MlsKeyPackage => Self::MlsKeyPackage,
            KindStandard::MlsWelcome => Self::MlsWelcome,
            KindStandard::MlsGroupMessage => Self::MlsGroupMessage,
            KindStandard::Torrent => Self::Torrent,
            KindStandard::TorrentComment => Self::TorrentComment,
            KindStandard::PeerToPeerOrder => Self::PeerToPeerOrder,
            KindStandard::RequestToVanish => Self::RequestToVanish,
            KindStandard::UserStatus => Self::UserStatus,
            KindStandard::CashuWallet => Self::CashuWallet,
            KindStandard::CashuWalletUnspentProof => Self::CashuWalletUnspentProof,
            KindStandard::CashuWalletSpendingHistory => Self::CashuWalletSpendingHistory,
            KindStandard::CashuWalletQuote => Self::CashuWalletQuote,
            KindStandard::CashuNutZapInfo => Self::CashuNutZapInfo,
            KindStandard::CashuNutZap => Self::CashuNutZap,
            KindStandard::CodeSnippet => Self::CodeSnippet,
            KindStandard::Poll => Self::Poll,
            KindStandard::PollResponse => Self::PollResponse,
            KindStandard::RepoState => Self::RepoState,
            KindStandard::VoiceMessage => Self::VoiceMessage,
            KindStandard::VoiceMessageReply => Self::VoiceMessageReply,
            KindStandard::Thread => Self::Thread,
            KindStandard::WebBookmark => Self::WebBookmark,
            KindStandard::ChatMessage => Self::ChatMessage,
        }
    }
}
