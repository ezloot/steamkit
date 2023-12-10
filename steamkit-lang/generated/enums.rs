#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ELauncherType(pub i32);

impl ELauncherType {
    pub const SINGLE_APP: Self = Self(8);
    pub const STEAM_CHINA: Self = Self(7);
    pub const HEADLESS: Self = Self(6);
    pub const CLIENT_UI: Self = Self(5);
    pub const CSGO: Self = Self(4);
    pub const CMD_LINE: Self = Self(3);
    pub const NEXON: Self = Self(2);
    pub const PERFECT_WORLD: Self = Self(1);
    pub const DEFAULT: Self = Self(0);
}

impl PartialEq<i32> for ELauncherType {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<ELauncherType> for i32 {
    fn eq(&self, other: &ELauncherType) -> bool {
        *self == other.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ESteamRealm(pub i32);

impl ESteamRealm {
    pub const STEAM_CHINA: Self = Self(2);
    pub const STEAM_GLOBAL: Self = Self(1);
    pub const UNKNOWN: Self = Self(0);
}

impl PartialEq<i32> for ESteamRealm {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<ESteamRealm> for i32 {
    fn eq(&self, other: &ESteamRealm) -> bool {
        *self == other.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ESteamIPv6ConnectivityState(pub i32);

impl ESteamIPv6ConnectivityState {
    pub const BAD: Self = Self(2);
    pub const GOOD: Self = Self(1);
    pub const UNKNOWN: Self = Self(0);
}

impl PartialEq<i32> for ESteamIPv6ConnectivityState {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<ESteamIPv6ConnectivityState> for i32 {
    fn eq(&self, other: &ESteamIPv6ConnectivityState) -> bool {
        *self == other.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ESteamIPv6ConnectivityProtocol(pub i32);

impl ESteamIPv6ConnectivityProtocol {
    pub const UDP: Self = Self(2);
    pub const HTTP: Self = Self(1);
    pub const INVALID: Self = Self(0);
}

impl PartialEq<i32> for ESteamIPv6ConnectivityProtocol {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<ESteamIPv6ConnectivityProtocol> for i32 {
    fn eq(&self, other: &ESteamIPv6ConnectivityProtocol) -> bool {
        *self == other.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ELobbyDistanceFilter(pub i32);

impl ELobbyDistanceFilter {
    pub const WORLDWIDE: Self = Self(3);
    pub const FAR: Self = Self(2);
    pub const DEFAULT: Self = Self(1);
    pub const CLOSE: Self = Self(0);
}

impl PartialEq<i32> for ELobbyDistanceFilter {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<ELobbyDistanceFilter> for i32 {
    fn eq(&self, other: &ELobbyDistanceFilter) -> bool {
        *self == other.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ELobbyComparison(pub i32);

impl ELobbyComparison {
    pub const NOT_EQUAL: Self = Self(3);
    pub const EQUAL_TO_OR_GREATER_THAN: Self = Self(2);
    pub const GREATER_THAN: Self = Self(1);
    pub const EQUAL: Self = Self(0);
    pub const LESS_THAN: Self = Self(-1);
    pub const EQUAL_TO_OR_LESS_THAN: Self = Self(-2);
}

impl PartialEq<i32> for ELobbyComparison {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<ELobbyComparison> for i32 {
    fn eq(&self, other: &ELobbyComparison) -> bool {
        *self == other.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ELobbyFilterType(pub i32);

impl ELobbyFilterType {
    pub const DISTANCE: Self = Self(4);
    pub const NEAR_VALUE: Self = Self(3);
    pub const SLOTS_AVAILABLE: Self = Self(2);
    pub const NUMERICAL: Self = Self(1);
    pub const STRING: Self = Self(0);
}

impl PartialEq<i32> for ELobbyFilterType {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<ELobbyFilterType> for i32 {
    fn eq(&self, other: &ELobbyFilterType) -> bool {
        *self == other.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ELobbyType(pub i32);

impl ELobbyType {
    pub const PRIVATE_UNIQUE: Self = Self(4);
    pub const INVISIBLE: Self = Self(3);
    pub const PUBLIC: Self = Self(2);
    pub const FRIENDS_ONLY: Self = Self(1);
    pub const PRIVATE: Self = Self(0);
}

impl PartialEq<i32> for ELobbyType {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<ELobbyType> for i32 {
    fn eq(&self, other: &ELobbyType) -> bool {
        *self == other.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ETradeOfferConfirmationMethod(pub i32);

impl ETradeOfferConfirmationMethod {
    pub const MOBILE_APP: Self = Self(2);
    pub const EMAIL: Self = Self(1);
    pub const INVALID: Self = Self(0);
}

impl PartialEq<i32> for ETradeOfferConfirmationMethod {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<ETradeOfferConfirmationMethod> for i32 {
    fn eq(&self, other: &ETradeOfferConfirmationMethod) -> bool {
        *self == other.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ETradeOfferState(pub i32);

impl ETradeOfferState {
    pub const IN_ESCROW: Self = Self(11);
    pub const CANCELED_BY_SECOND_FACTOR: Self = Self(10);
    pub const CREATED_NEEDS_CONFIRMATION: Self = Self(9);
    pub const INVALID_ITEMS: Self = Self(8);
    pub const DECLINED: Self = Self(7);
    pub const CANCELED: Self = Self(6);
    pub const EXPIRED: Self = Self(5);
    pub const COUNTERED: Self = Self(4);
    pub const ACCEPTED: Self = Self(3);
    pub const ACTIVE: Self = Self(2);
    pub const INVALID: Self = Self(1);
}

impl PartialEq<i32> for ETradeOfferState {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<ETradeOfferState> for i32 {
    fn eq(&self, other: &ETradeOfferState) -> bool {
        *self == other.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EVoiceCallState(pub i32);

impl EVoiceCallState {
    pub const CONNECTED: Self = Self(9);
    pub const NOTIFYING_VOICE_CHAT_OF_WEB_RTC_SESSION: Self = Self(8);
    pub const REQUESTED_PERMISSION: Self = Self(7);
    pub const WEB_RTC_CONNECTED_WAITING_ON_ICE_CONNECTED: Self = Self(6);
    pub const INITATED_WEB_RTC_SESSION: Self = Self(5);
    pub const CREATE_PEER_CONNECTION: Self = Self(4);
    pub const LOCAL_MIC_ONLY: Self = Self(3);
    pub const REQUESTED_MIC_ACCESS: Self = Self(2);
    pub const SCHEDULED_INITIATE: Self = Self(1);
    pub const NONE: Self = Self(0);
}

impl PartialEq<i32> for EVoiceCallState {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<EVoiceCallState> for i32 {
    fn eq(&self, other: &EVoiceCallState) -> bool {
        *self == other.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EChatRoomJoinState(pub i32);

impl EChatRoomJoinState {
    pub const JOINED: Self = Self(2);
    pub const NONE: Self = Self(1);
    pub const DEFAULT: Self = Self(0);
}

impl PartialEq<i32> for EChatRoomJoinState {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<EChatRoomJoinState> for i32 {
    fn eq(&self, other: &EChatRoomJoinState) -> bool {
        *self == other.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EChatRoomGroupAction(pub i32);

impl EChatRoomGroupAction {
    pub const SET_WATCHING_BROADCAST: Self = Self(11);
    pub const MENTION_ALL: Self = Self(10);
    pub const CHANGE_USER_ROLES: Self = Self(9);
    pub const CHANGE_GROUP_ROLES: Self = Self(8);
    pub const VIEW_HISTORY: Self = Self(7);
    pub const CHAT: Self = Self(6);
    pub const CHANGE_TAGLINE_AVATAR_NAME: Self = Self(5);
    pub const INVITE: Self = Self(4);
    pub const BAN: Self = Self(3);
    pub const KICK: Self = Self(2);
    pub const CREATE_RENAME_DELETE_CHANNEL: Self = Self(1);
    pub const DEFAULT: Self = Self(0);
}

impl PartialEq<i32> for EChatRoomGroupAction {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<EChatRoomGroupAction> for i32 {
    fn eq(&self, other: &EChatRoomGroupAction) -> bool {
        *self == other.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EChatRoomGroupPermissions(pub i32);

impl EChatRoomGroupPermissions {
    pub const CAN_ADMIN_CHANNEL: Self = Self(16);
    pub const CAN_BAN: Self = Self(8);
    pub const CAN_KICK: Self = Self(4);
    pub const CAN_INVITE: Self = Self(2);
    pub const VALID: Self = Self(1);
    pub const DEFAULT: Self = Self(0);
}

impl PartialEq<i32> for EChatRoomGroupPermissions {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<EChatRoomGroupPermissions> for i32 {
    fn eq(&self, other: &EChatRoomGroupPermissions) -> bool {
        *self == other.0
    }
}

impl ops::BitOr for EChatRoomGroupPermissions {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EChatRoomGroupRank(pub i32);

impl EChatRoomGroupRank {
    pub const OWNER: Self = Self(50);
    pub const OFFICER: Self = Self(40);
    pub const MODERATOR: Self = Self(30);
    pub const MEMBER: Self = Self(20);
    pub const GUEST: Self = Self(15);
    pub const VIEWER: Self = Self(10);
    pub const DEFAULT: Self = Self(0);
}

impl PartialEq<i32> for EChatRoomGroupRank {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<EChatRoomGroupRank> for i32 {
    fn eq(&self, other: &EChatRoomGroupRank) -> bool {
        *self == other.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EChatRoomServerMsg(pub i32);

impl EChatRoomServerMsg {
    pub const APP_CUSTOM: Self = Self(11);
    pub const CHAT_ROOM_AVATAR_CHANGED: Self = Self(10);
    pub const CHAT_ROOM_TAGLINE_CHANGED: Self = Self(9);
    pub const INVITE_DISMISSED: Self = Self(8);
    pub const INVITED: Self = Self(5);
    pub const KICKED: Self = Self(4);
    pub const PARTED: Self = Self(3);
    pub const JOINED: Self = Self(2);
    pub const RENAME_CHAT_ROOM: Self = Self(1);
    pub const INVALID: Self = Self(0);
}

impl PartialEq<i32> for EChatRoomServerMsg {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<EChatRoomServerMsg> for i32 {
    fn eq(&self, other: &EChatRoomServerMsg) -> bool {
        *self == other.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EChatRoomMemberStateChange(pub i32);

impl EChatRoomMemberStateChange {
    pub const ROLES_CHANGED: Self = Self(12);
    pub const BANNED: Self = Self(10);
    pub const MUTED: Self = Self(9);
    pub const INVITE_DISMISSED: Self = Self(8);
    pub const RANK_CHANGED: Self = Self(7);
    pub const INVITED: Self = Self(4);
    pub const KICKED: Self = Self(3);
    pub const PARTED: Self = Self(2);
    pub const JOINED: Self = Self(1);
    pub const INVALID: Self = Self(0);
}

impl PartialEq<i32> for EChatRoomMemberStateChange {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<EChatRoomMemberStateChange> for i32 {
    fn eq(&self, other: &EChatRoomMemberStateChange) -> bool {
        *self == other.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EChatroomNotificationLevel(pub i32);

impl EChatroomNotificationLevel {
    pub const ALL_MESSAGES: Self = Self(4);
    pub const MENTION_ALL: Self = Self(3);
    pub const MENTION_ME: Self = Self(2);
    pub const NONE: Self = Self(1);
    pub const INVALID: Self = Self(0);
}

impl PartialEq<i32> for EChatroomNotificationLevel {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<EChatroomNotificationLevel> for i32 {
    fn eq(&self, other: &EChatroomNotificationLevel) -> bool {
        *self == other.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EChatRoomGroupType(pub i32);

impl EChatRoomGroupType {
    pub const UNMODERATED: Self = Self(1);
    pub const DEFAULT: Self = Self(0);
}

impl PartialEq<i32> for EChatRoomGroupType {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<EChatRoomGroupType> for i32 {
    fn eq(&self, other: &EChatRoomGroupType) -> bool {
        *self == other.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EAppType(pub i32);

impl EAppType {
    #[deprecated]
    pub const DEPOT_ONLY: Self = Self(-2147483648);
    pub const SHORTCUT: Self = Self(1073741824);
    pub const BETA: Self = Self(65536);
    pub const COMIC: Self = Self(32768);
    pub const SERIES: Self = Self(16384);
    pub const MUSIC: Self = Self(8192);
    pub const PLUGIN: Self = Self(4096);
    pub const VIDEO: Self = Self(2048);
    pub const FRANCHISE: Self = Self(1024);
    pub const HARDWARE: Self = Self(512);
    pub const CONFIG: Self = Self(256);
    pub const DRIVER: Self = Self(128);
    pub const GUIDE: Self = Self(64);
    pub const DLC: Self = Self(32);
    pub const DEPRECTED: Self = Self(16);
    pub const DEMO: Self = Self(8);
    pub const TOOL: Self = Self(4);
    pub const APPLICATION: Self = Self(2);
    pub const GAME: Self = Self(1);
    pub const INVALID: Self = Self(0);
}

impl PartialEq<i32> for EAppType {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<EAppType> for i32 {
    fn eq(&self, other: &EAppType) -> bool {
        *self == other.0
    }
}

impl ops::BitOr for EAppType {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EDisplayStatus(pub i32);

impl EDisplayStatus {
    pub const PURCHASE: Self = Self(31);
    pub const AVAIL_GUEST_PASS: Self = Self(30);
    pub const AVAIL_TO_BORROW: Self = Self(29);
    pub const AVAIL_FOR_FREE: Self = Self(28);
    pub const LICENSE_EXPIRED: Self = Self(27);
    pub const LICENSE_PENDING: Self = Self(26);
    pub const DOWNLOAD_DISABLED: Self = Self(25);
    pub const DOWNLOAD_REQUIRED: Self = Self(24);
    pub const DOWNLOAD_QUEUED: Self = Self(23);
    pub const DOWNLOAD_PAUSED: Self = Self(22);
    pub const UPDATE_DISABLED: Self = Self(21);
    pub const UPDATE_REQUIRED: Self = Self(20);
    pub const UPDATE_QUEUED: Self = Self(19);
    pub const UPDATE_PAUSED: Self = Self(18);
    pub const BORROWER_LOCKED: Self = Self(17);
    pub const PRELOAD_ONLY: Self = Self(16);
    pub const PARENTAL_BLOCKED: Self = Self(15);
    pub const INVALID_PLATFORM: Self = Self(14);
    pub const PRESALE_ONLY: Self = Self(13);
    pub const REGION_RESTRICTED: Self = Self(12);
    pub const READY_TO_LAUNCH: Self = Self(11);
    pub const READY_TO_PRELOAD: Self = Self(10);
    pub const READY_TO_INSTALL: Self = Self(9);
    pub const SYNCHRONIZING: Self = Self(8);
    pub const DOWNLOADING: Self = Self(7);
    pub const UPDATING: Self = Self(6);
    pub const VALIDATING: Self = Self(5);
    pub const RUNNING: Self = Self(4);
    pub const INSTALLING: Self = Self(3);
    pub const UNINSTALLING: Self = Self(2);
    pub const LAUNCHING: Self = Self(1);
    pub const INVALID: Self = Self(0);
}

impl PartialEq<i32> for EDisplayStatus {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<EDisplayStatus> for i32 {
    fn eq(&self, other: &EDisplayStatus) -> bool {
        *self == other.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EPublishedFileInappropriateResult(pub i32);

impl EPublishedFileInappropriateResult {
    pub const VERY_LIKELY: Self = Self(100);
    pub const LIKELY: Self = Self(75);
    pub const POSSIBLE: Self = Self(50);
    pub const UNLIKELY: Self = Self(30);
    pub const VERY_UNLIKELY: Self = Self(1);
    pub const NOT_SCANNED: Self = Self(0);
}

impl PartialEq<i32> for EPublishedFileInappropriateResult {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<EPublishedFileInappropriateResult> for i32 {
    fn eq(&self, other: &EPublishedFileInappropriateResult) -> bool {
        *self == other.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EPublishedFileInappropriateProvider(pub i32);

impl EPublishedFileInappropriateProvider {
    pub const AMAZON: Self = Self(2);
    pub const GOOGLE: Self = Self(1);
    pub const INVALID: Self = Self(0);
}

impl PartialEq<i32> for EPublishedFileInappropriateProvider {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<EPublishedFileInappropriateProvider> for i32 {
    fn eq(&self, other: &EPublishedFileInappropriateProvider) -> bool {
        *self == other.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EPublishedFileQueryType(pub i32);

impl EPublishedFileQueryType {
    pub const RANKED_BY_INAPPROPRIATE_CONTENT_RATING: Self = Self(19);
    pub const RANKED_BY_LIFETIME_PLAYTIME_SESSIONS: Self = Self(18);
    pub const RANKED_BY_PLAYTIME_SESSIONS_TREND: Self = Self(17);
    pub const RANKED_BY_LIFETIME_AVERAGE_PLAYTIME: Self = Self(16);
    pub const RANKED_BY_AVERAGE_PLAYTIME_TREND: Self = Self(15);
    pub const RANKED_BY_TOTAL_PLAYTIME: Self = Self(14);
    pub const RANKED_BY_PLAYTIME_TREND: Self = Self(13);
    pub const RANKED_BY_TEXT_SEARCH: Self = Self(12);
    pub const RANKED_BY_VOTES_UP: Self = Self(11);
    pub const RANKED_BY_TOTAL_VOTES_ASC: Self = Self(10);
    pub const RANKED_BY_TOTAL_UNIQUE_SUBSCRIPTIONS: Self = Self(9);
    pub const NOT_YET_RATED: Self = Self(8);
    pub const CREATED_BY_FOLLOWED_USERS_RANKED_BY_PUBLICATION_DATE: Self = Self(7);
    pub const RANKED_BY_NUM_TIMES_REPORTED: Self = Self(6);
    pub const CREATED_BY_FRIENDS_RANKED_BY_PUBLICATION_DATE: Self = Self(5);
    pub const FAVORITED_BY_FRIENDS_RANKED_BY_PUBLICATION_DATE: Self = Self(4);
    pub const RANKED_BY_TREND: Self = Self(3);
    pub const ACCEPTED_FOR_GAME_RANKED_BY_ACCEPTANCE_DATE: Self = Self(2);
    pub const RANKED_BY_PUBLICATION_DATE: Self = Self(1);
    pub const RANKED_BY_VOTE: Self = Self(0);
}

impl PartialEq<i32> for EPublishedFileQueryType {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<EPublishedFileQueryType> for i32 {
    fn eq(&self, other: &EPublishedFileQueryType) -> bool {
        *self == other.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EUCMFilePrivacyState(pub i32);

impl EUCMFilePrivacyState {
    pub const ALL: Self = Self::PUBLIC | Self::FRIENDS_ONLY | Self::PRIVATE;
    pub const PUBLIC: Self = Self(8);
    pub const FRIENDS_ONLY: Self = Self(4);
    pub const PRIVATE: Self = Self(2);
    pub const INVALID: Self = Self(-1);
}

impl PartialEq<i32> for EUCMFilePrivacyState {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<EUCMFilePrivacyState> for i32 {
    fn eq(&self, other: &EUCMFilePrivacyState) -> bool {
        *self == other.0
    }
}

impl ops::BitOr for EUCMFilePrivacyState {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ELeaderboardUploadScoreMethod(pub i32);

impl ELeaderboardUploadScoreMethod {
    pub const FORCE_UPDATE: Self = Self(2);
    pub const KEEP_BEST: Self = Self(1);
    pub const NONE: Self = Self(0);
}

impl PartialEq<i32> for ELeaderboardUploadScoreMethod {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<ELeaderboardUploadScoreMethod> for i32 {
    fn eq(&self, other: &ELeaderboardUploadScoreMethod) -> bool {
        *self == other.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ELeaderboardDisplayType(pub i32);

impl ELeaderboardDisplayType {
    pub const TIME_MILLI_SECONDS: Self = Self(3);
    pub const TIME_SECONDS: Self = Self(2);
    pub const NUMERIC: Self = Self(1);
    pub const NONE: Self = Self(0);
}

impl PartialEq<i32> for ELeaderboardDisplayType {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<ELeaderboardDisplayType> for i32 {
    fn eq(&self, other: &ELeaderboardDisplayType) -> bool {
        *self == other.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ELeaderboardSortMethod(pub i32);

impl ELeaderboardSortMethod {
    pub const DESCENDING: Self = Self(2);
    pub const ASCENDING: Self = Self(1);
    pub const NONE: Self = Self(0);
}

impl PartialEq<i32> for ELeaderboardSortMethod {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<ELeaderboardSortMethod> for i32 {
    fn eq(&self, other: &ELeaderboardSortMethod) -> bool {
        *self == other.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ELeaderboardDataRequest(pub i32);

impl ELeaderboardDataRequest {
    pub const USERS: Self = Self(3);
    pub const FRIENDS: Self = Self(2);
    pub const GLOBAL_AROUND_USER: Self = Self(1);
    pub const GLOBAL: Self = Self(0);
}

impl PartialEq<i32> for ELeaderboardDataRequest {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<ELeaderboardDataRequest> for i32 {
    fn eq(&self, other: &ELeaderboardDataRequest) -> bool {
        *self == other.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EClientStatAggregateMethod(pub i32);

impl EClientStatAggregateMethod {
    pub const SCALAR: Self = Self(3);
    pub const EVENT: Self = Self(2);
    pub const SUM: Self = Self(1);
    pub const LATEST_ONLY: Self = Self(0);
}

impl PartialEq<i32> for EClientStatAggregateMethod {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<EClientStatAggregateMethod> for i32 {
    fn eq(&self, other: &EClientStatAggregateMethod) -> bool {
        *self == other.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EClientStat(pub i32);

impl EClientStat {
    pub const BYTES_DOWNLOADED: Self = Self(4);
    pub const P2P_VOICE_CONNECTIONS: Self = Self(3);
    pub const P2P_GAME_CONNECTIONS: Self = Self(2);
    pub const P2P_CONNECTIONS_RELAY: Self = Self(1);
    pub const P2P_CONNECTIONS_UDP: Self = Self(0);
}

impl PartialEq<i32> for EClientStat {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<EClientStat> for i32 {
    fn eq(&self, other: &EClientStat) -> bool {
        *self == other.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EDRMBlobDownloadErrorDetail(pub i32);

impl EDRMBlobDownloadErrorDetail {
    pub const NEXT_BASE: Self = Self(131072);
    pub const TARGET_LOCKED_MAX: Self = Self(131071);
    pub const TARGET_LOCKED_BASE: Self = Self(65536);
    pub const PATH_MANIPULATION_ERROR: Self = Self(19);
    pub const UNZIP_VALVE_SIGNATURE_HEADER: Self = Self(18);
    pub const APPLY_VALVE_SIGNATURE_HEADER: Self = Self(17);
    pub const APPLIED_SIGNATURE_CORRUPT: Self = Self(16);
    pub const APP_ID_UNEXPECTED: Self = Self(15);
    pub const APP_ID_MISMATCH: Self = Self(14);
    pub const APPLY_SIGNATURE: Self = Self(13);
    pub const APPLY_MERGE_GUID: Self = Self(12);
    pub const APPLY_STRIPS: Self = Self(11);
    pub const UNZIP_SIGNATURE: Self = Self(10);
    pub const UNZIP_MERGE_GUID: Self = Self(9);
    pub const UNZIP_STRIPS: Self = Self(8);
    pub const UNKNOWN_BLOB_TYPE: Self = Self(7);
    pub const UNZIP_FULL_FILE: Self = Self(6);
    pub const UNEXPECTED_ZIP_ENTRY: Self = Self(5);
    pub const READ_ZIP_DIRECTORY: Self = Self(4);
    pub const OPEN_ZIP: Self = Self(3);
    pub const TARGET_LOCKED: Self = Self(2);
    pub const DOWNLOAD_FAILED: Self = Self(1);
    pub const NONE: Self = Self(0);
}

impl PartialEq<i32> for EDRMBlobDownloadErrorDetail {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<EDRMBlobDownloadErrorDetail> for i32 {
    fn eq(&self, other: &EDRMBlobDownloadErrorDetail) -> bool {
        *self == other.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EDRMBlobDownloadType(pub i32);

impl EDRMBlobDownloadType {
    pub const LOW_PRIORITY: Self = Self(64);
    pub const ADD_TIMESTAMP: Self = Self(32);
    pub const HIGH_PRIORITY: Self = Self(16);
    pub const IS_JOB: Self = Self(8);
    pub const ALL_MASK: Self = Self(7);
    pub const COMPRESSED: Self = Self(4);
    pub const PARTS: Self = Self(2);
    pub const FILE: Self = Self(1);
    pub const ERROR: Self = Self(0);
}

impl PartialEq<i32> for EDRMBlobDownloadType {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<EDRMBlobDownloadType> for i32 {
    fn eq(&self, other: &EDRMBlobDownloadType) -> bool {
        *self == other.0
    }
}

impl ops::BitOr for EDRMBlobDownloadType {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ERemoteStoragePlatform(pub i32);

impl ERemoteStoragePlatform {
    pub const ALL: Self = Self(-1);
    pub const I_PHONE_OS: Self = Self(64);
    pub const ANDROID: Self = Self(32);
    pub const SWITCH: Self = Self(16);
    pub const LINUX: Self = Self(8);
    pub const PS3: Self = Self(4);
    pub const OSX: Self = Self(2);
    pub const WINDOWS: Self = Self(1);
    pub const NONE: Self = Self(0);
}

impl PartialEq<i32> for ERemoteStoragePlatform {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<ERemoteStoragePlatform> for i32 {
    fn eq(&self, other: &ERemoteStoragePlatform) -> bool {
        *self == other.0
    }
}

impl ops::BitOr for ERemoteStoragePlatform {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EChatFlags(pub i32);

impl EChatFlags {
    pub const UNJOINABLE: Self = Self(8);
    pub const MODERATED: Self = Self(4);
    pub const INVISIBLE_TO_FRIENDS: Self = Self(2);
    pub const LOCKED: Self = Self(1);
}

impl PartialEq<i32> for EChatFlags {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<EChatFlags> for i32 {
    fn eq(&self, other: &EChatFlags) -> bool {
        *self == other.0
    }
}

impl ops::BitOr for EChatFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ESystemIMType(pub i32);

impl ESystemIMType {
    pub const SUPPORT_MESSAGE_CLEAR_ALERT: Self = Self(9);
    pub const SUPPORT_MESSAGE: Self = Self(8);
    pub const GIFT_REVOKED: Self = Self(7);
    pub const GUEST_PASS_GRANTED: Self = Self(6);
    pub const GUEST_PASS_RECEIVED: Self = Self(5);
    pub const SUBSCRIPTION_EXPIRED: Self = Self(4);
    pub const CARD_WILL_EXPIRE: Self = Self(3);
    pub const RECURRING_PURCHASE_FAILED: Self = Self(2);
    pub const INVALID_CARD: Self = Self(1);
    pub const RAW_TEXT: Self = Self(0);
}

impl PartialEq<i32> for ESystemIMType {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<ESystemIMType> for i32 {
    fn eq(&self, other: &ESystemIMType) -> bool {
        *self == other.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ENewsUpdateType(pub i32);

impl ENewsUpdateType {
    pub const CLIENT_UPDATE: Self = Self(4);
    pub const CDDB_UPDATE: Self = Self(3);
    pub const STEAM_NEWS: Self = Self(2);
    pub const STEAM_ADS: Self = Self(1);
    pub const APP_NEWS: Self = Self(0);
}

impl PartialEq<i32> for ENewsUpdateType {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<ENewsUpdateType> for i32 {
    fn eq(&self, other: &ENewsUpdateType) -> bool {
        *self == other.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EMarketingMessageFlags(pub i32);

impl EMarketingMessageFlags {
    pub const PLATFORM_RESTRICTIONS: Self = Self::PLATFORM_WINDOWS | Self::PLATFORM_MAC | Self::PLATFORM_LINUX;
    pub const PLATFORM_LINUX: Self = Self(8);
    pub const PLATFORM_MAC: Self = Self(4);
    pub const PLATFORM_WINDOWS: Self = Self(2);
    pub const HIGH_PRIORITY: Self = Self(1);
    pub const NONE: Self = Self(0);
}

impl PartialEq<i32> for EMarketingMessageFlags {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<EMarketingMessageFlags> for i32 {
    fn eq(&self, other: &EMarketingMessageFlags) -> bool {
        *self == other.0
    }
}

impl ops::BitOr for EMarketingMessageFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EEconTradeResponse(pub i32);

impl EEconTradeResponse {
    pub const OK_TO_DELIVER: Self = Self(50);
    pub const DISABLED_IN_PARTNER_REGION: Self = Self(33);
    pub const DISABLED_IN_REGION: Self = Self(32);
    pub const WOULD_EXCEED_MAX_ASSET_COUNT: Self = Self(31);
    pub const TRADING_HOLD_FOR_CLEARED_TRADE_OFFERS_INITIATOR: Self = Self(30);
    pub const NEEDS_MOBILE_CONFIRMATION: Self = Self(29);
    pub const INITIATOR_RECENT_EMAIL_CHANGE: Self = Self(28);
    pub const NEEDS_EMAIL_CONFIRMATION: Self = Self(27);
    pub const INITIATOR_SENT_INVALID_COOKIE: Self = Self(26);
    pub const INITIATOR_NEW_DEVICE_COOLDOWN: Self = Self(25);
    pub const INITIATOR_PASSWORD_RESET_PROBATION: Self = Self(24);
    pub const INITIATOR_STEAM_GUARD_DURATION: Self = Self(23);
    pub const TARGET_ACCOUNT_CANNOT_TRADE: Self = Self(22);
    pub const INITIATOR_NEEDS_STEAM_GUARD: Self = Self(21);
    pub const INITIATOR_NEEDS_VERIFIED_EMAIL: Self = Self(20);
    pub const INITIATOR_BLOCKED_TARGET: Self = Self(18);
    pub const SCHOOL_LAB_TARGET: Self = Self(16);
    pub const SCHOOL_LAB_INITIATOR: Self = Self(16);
    pub const CYBER_CAFE_TARGET: Self = Self(15);
    pub const CYBER_CAFE_INITIATOR: Self = Self(14);
    pub const NO_RESPONSE: Self = Self(13);
    pub const ALREADY_HAS_TRADE_REQUEST: Self = Self(12);
    pub const ALREADY_TRADING: Self = Self(11);
    pub const CONNECTION_FAILED: Self = Self(10);
    pub const TOO_SOON_PENALTY: Self = Self(9);
    pub const TOO_SOON: Self = Self(8);
    pub const CANCEL: Self = Self(7);
    pub const NOT_LOGGED_IN: Self = Self(6);
    pub const DISABLED: Self = Self(5);
    pub const TARGET_ALREADY_TRADING: Self = Self(4);
    pub const TRADE_BANNED_TARGET: Self = Self(3);
    pub const TRADE_BANNED_INITIATOR: Self = Self(2);
    pub const DECLINED: Self = Self(1);
    pub const ACCEPTED: Self = Self(0);
}

impl PartialEq<i32> for EEconTradeResponse {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<EEconTradeResponse> for i32 {
    fn eq(&self, other: &EEconTradeResponse) -> bool {
        *self == other.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EWorkshopFileAction(pub i32);

impl EWorkshopFileAction {
    pub const COMPLETED: Self = Self(1);
    pub const PLAYED: Self = Self(0);
}

impl PartialEq<i32> for EWorkshopFileAction {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<EWorkshopFileAction> for i32 {
    fn eq(&self, other: &EWorkshopFileAction) -> bool {
        *self == other.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EWorkshopFileType(pub i32);

impl EWorkshopFileType {
    pub const GAME_MANAGED_ITEM: Self = Self(15);
    pub const STEAM_VIDEO: Self = Self(14);
    pub const STEAMWORKS_ACCESS_INVITE: Self = Self(13);
    pub const CONTROLLER_BINDING: Self = Self(12);
    pub const MERCH: Self = Self(11);
    pub const INTEGRATED_GUIDE: Self = Self(10);
    pub const WEB_GUIDE: Self = Self(9);
    pub const CONCEPT: Self = Self(8);
    pub const SOFTWARE: Self = Self(7);
    pub const GAME: Self = Self(6);
    pub const SCREENSHOT: Self = Self(5);
    pub const VIDEO: Self = Self(4);
    pub const ART: Self = Self(3);
    pub const COLLECTION: Self = Self(2);
    pub const MICROTRANSACTION: Self = Self(1);
    pub const COMMUNITY: Self = Self(0);
    pub const FIRST: Self = Self(0);
}

impl PartialEq<i32> for EWorkshopFileType {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<EWorkshopFileType> for i32 {
    fn eq(&self, other: &EWorkshopFileType) -> bool {
        *self == other.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EPublishedFileVisibility(pub i32);

impl EPublishedFileVisibility {
    pub const PRIVATE: Self = Self(2);
    pub const FRIENDS_ONLY: Self = Self(1);
    pub const PUBLIC: Self = Self(0);
}

impl PartialEq<i32> for EPublishedFileVisibility {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<EPublishedFileVisibility> for i32 {
    fn eq(&self, other: &EPublishedFileVisibility) -> bool {
        *self == other.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EWorkshopEnumerationType(pub i32);

impl EWorkshopEnumerationType {
    pub const RECENT_FROM_FOLLOWED_USERS: Self = Self(6);
    pub const CONTENT_BY_FRIENDS: Self = Self(5);
    pub const VOTED_BY_FRIENDS: Self = Self(4);
    pub const FAVORITE_OF_FRIENDS: Self = Self(3);
    pub const TRENDING: Self = Self(2);
    pub const RECENT: Self = Self(1);
    pub const RANKED_BY_VOTE: Self = Self(0);
}

impl PartialEq<i32> for EWorkshopEnumerationType {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<EWorkshopEnumerationType> for i32 {
    fn eq(&self, other: &EWorkshopEnumerationType) -> bool {
        *self == other.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EDepotFileFlag(pub i32);

impl EDepotFileFlag {
    pub const SYMLINK: Self = Self(512);
    pub const INSTALL_SCRIPT: Self = Self(256);
    pub const CUSTOM_EXECUTABLE: Self = Self(128);
    pub const DIRECTORY: Self = Self(64);
    pub const EXECUTABLE: Self = Self(32);
    pub const HIDDEN: Self = Self(16);
    pub const READ_ONLY: Self = Self(8);
    pub const ENCRYPTED: Self = Self(4);
    pub const VERSIONED_USER_CONFIG: Self = Self(2);
    pub const USER_CONFIG: Self = Self(1);
}

impl PartialEq<i32> for EDepotFileFlag {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<EDepotFileFlag> for i32 {
    fn eq(&self, other: &EDepotFileFlag) -> bool {
        *self == other.0
    }
}

impl ops::BitOr for EDepotFileFlag {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ECurrencyCode(pub i32);

impl ECurrencyCode {
    pub const UYU: Self = Self(41);
    pub const CRC: Self = Self(40);
    pub const QAR: Self = Self(39);
    pub const KWD: Self = Self(38);
    pub const KZT: Self = Self(37);
    pub const BYN: Self = Self(36);
    pub const ILS: Self = Self(35);
    pub const ARS: Self = Self(34);
    pub const AED: Self = Self(32);
    pub const SAR: Self = Self(31);
    pub const TWD: Self = Self(30);
    pub const HKD: Self = Self(29);
    pub const ZAR: Self = Self(28);
    pub const COP: Self = Self(27);
    pub const PEN: Self = Self(26);
    pub const CLP: Self = Self(25);
    pub const INR: Self = Self(24);
    pub const CNY: Self = Self(23);
    pub const NZD: Self = Self(22);
    pub const AUD: Self = Self(21);
    pub const CAD: Self = Self(20);
    pub const MXN: Self = Self(19);
    pub const UAH: Self = Self(18);
    pub const TRY: Self = Self(17);
    pub const KRW: Self = Self(16);
    pub const VND: Self = Self(15);
    pub const THB: Self = Self(14);
    pub const SGD: Self = Self(13);
    pub const PHP: Self = Self(12);
    pub const MYR: Self = Self(11);
    pub const IDR: Self = Self(10);
    pub const NOK: Self = Self(9);
    pub const JPY: Self = Self(8);
    pub const BRL: Self = Self(7);
    pub const PLN: Self = Self(6);
    pub const RUB: Self = Self(5);
    pub const CHF: Self = Self(4);
    pub const EUR: Self = Self(3);
    pub const GBP: Self = Self(2);
    pub const USD: Self = Self(1);
    pub const INVALID: Self = Self(0);
}

impl PartialEq<i32> for ECurrencyCode {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<ECurrencyCode> for i32 {
    fn eq(&self, other: &ECurrencyCode) -> bool {
        *self == other.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ERegionCode(pub i32);

impl ERegionCode {
    pub const WORLD: Self = Self(0xFF);
    pub const AFRICA: Self = Self(0x07);
    pub const MIDDLE_EAST: Self = Self(0x06);
    pub const AUSTRALIA: Self = Self(0x05);
    pub const ASIA: Self = Self(0x04);
    pub const EUROPE: Self = Self(0x03);
    pub const SOUTH_AMERICA: Self = Self(0x02);
    pub const US_WEST: Self = Self(0x01);
    pub const US_EAST: Self = Self(0x00);
}

impl PartialEq<i32> for ERegionCode {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<ERegionCode> for i32 {
    fn eq(&self, other: &ERegionCode) -> bool {
        *self == other.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EChatMemberStateChange(pub i32);

impl EChatMemberStateChange {
    pub const VOICE_DONE_SPEAKING: Self = Self(0x2000);
    pub const VOICE_SPEAKING: Self = Self(0x1000);
    pub const BANNED: Self = Self(0x10);
    pub const KICKED: Self = Self(0x08);
    pub const DISCONNECTED: Self = Self(0x04);
    pub const LEFT: Self = Self(0x02);
    pub const ENTERED: Self = Self(0x01);
}

impl PartialEq<i32> for EChatMemberStateChange {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<EChatMemberStateChange> for i32 {
    fn eq(&self, other: &EChatMemberStateChange) -> bool {
        *self == other.0
    }
}

impl ops::BitOr for EChatMemberStateChange {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EActivationCodeClass(pub i32);

impl EActivationCodeClass {
    pub const INVALID: Self = Self(4294967295);
    pub const TEST: Self = Self(2147483647);
    pub const MAX: Self = Self(5);
    pub const STEAM2010_KEY: Self = Self(4);
    pub const DB_LOOKUP: Self = Self(3);
    pub const DOOM3_CD_KEY: Self = Self(2);
    pub const VALVE_CD_KEY: Self = Self(1);
    pub const WON_CD_KEY: Self = Self(0);
}

impl PartialEq<i32> for EActivationCodeClass {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<EActivationCodeClass> for i32 {
    fn eq(&self, other: &EActivationCodeClass) -> bool {
        *self == other.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EPackageStatus(pub i32);

impl EPackageStatus {
    pub const INVALID: Self = Self(3);
    pub const UNAVAILABLE: Self = Self(2);
    pub const PREORDER: Self = Self(1);
    pub const AVAILABLE: Self = Self(0);
}

impl PartialEq<i32> for EPackageStatus {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<EPackageStatus> for i32 {
    fn eq(&self, other: &EPackageStatus) -> bool {
        *self == other.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EBillingType(pub i32);

impl EBillingType {
    pub const NUM_BILLING_TYPES: Self = Self(16);
    pub const FREE_COMMERCIAL_LICENSE: Self = Self(15);
    pub const COMMERCIAL_LICENSE: Self = Self(14);
    pub const RENTAL: Self = Self(13);
    pub const FREE_ON_DEMAND: Self = Self(12);
    pub const REPURCHASEABLE: Self = Self(11);
    pub const BILL_ONCE_OR_CD_KEY: Self = Self(10);
    pub const RECURRING_OPTION: Self = Self(9);
    pub const OEM_TICKET: Self = Self(8);
    pub const AUTO_GRANT: Self = Self(7);
    pub const GIFT: Self = Self(6);
    pub const HARDWARE_PROMO: Self = Self(5);
    pub const GUEST_PASS: Self = Self(4);
    pub const PROOF_OF_PREPURCHASE_ONLY: Self = Self(3);
    pub const BILL_MONTHLY: Self = Self(2);
    pub const BILL_ONCE_ONLY: Self = Self(1);
    pub const NO_COST: Self = Self(0);
}

impl PartialEq<i32> for EBillingType {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<EBillingType> for i32 {
    fn eq(&self, other: &EBillingType) -> bool {
        *self == other.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EServerType(pub i32);

impl EServerType {
    pub const CRASH_DUMP: Self = Self(126);
    pub const CHINA: Self = Self(125);
    pub const STEAM_AR: Self = Self(124);
    pub const BROADCAST_CHANNEL: Self = Self(123);
    pub const BROADCAST_ORIGIN: Self = Self(122);
    pub const REMOTE_CLIENT: Self = Self(121);
    pub const TURN: Self = Self(120);
    pub const UGS_AGGREGATE: Self = Self(119);
    pub const ML_INFERENCE: Self = Self(118);
    pub const TAX_SERVICE: Self = Self(117);
    pub const VAC_TEST: Self = Self(116);
    pub const ML_TRAIN: Self = Self(115);
    pub const MINIGAME: Self = Self(114);
    pub const CONTENT_SERVER_CONFIG: Self = Self(113);
    pub const VACDB_MASTER: Self = Self(112);
    pub const TIME_MACHINE: Self = Self(111);
    pub const TRUST: Self = Self(110);
    pub const QMS: Self = Self(109);
    pub const VOICE_CHAT: Self = Self(108);
    pub const CHAT_ROOM: Self = Self(107);
    pub const GIVEAWAY: Self = Self(106);
    pub const WEB_RTC: Self = Self(105);
    pub const ACCOUNT_HARDWARE: Self = Self(104);
    pub const SHADER: Self = Self(103);
    pub const CHAT: Self = Self(102);
    pub const ACCOUNT_HISTORY: Self = Self(101);
    pub const WATCHDOG: Self = Self(100);
    pub const ITEM_INVENTORY: Self = Self(99);
    pub const PERF: Self = Self(98);
    pub const IDLS: Self = Self(97);
    pub const HLTV_RELAY: Self = Self(96);
    pub const STORE_CATALOG: Self = Self(95);
    pub const AUTH: Self = Self(94);
    pub const INVENTORY_MANAGEMENT: Self = Self(93);
    pub const EMAIL_DELIVERY: Self = Self(92);
    pub const LOG_WORKER: Self = Self(91);
    pub const LOG_REQUEST: Self = Self(90);
    pub const SUPPORT: Self = Self(89);
    pub const ACCOUNT_SCORE: Self = Self(88);
    pub const PHONE: Self = Self(87);
    pub const BROADCAST_CHAT: Self = Self(86);
    pub const TRADE_OFFER: Self = Self(85);
    pub const VIDEO_MANAGER: Self = Self(84);
    pub const BROADCAST_DIRECTORY: Self = Self(83);
    pub const BROADCAST_RELAY: Self = Self(82);
    pub const SOLR_MGR: Self = Self(81);
    pub const PUBLIC_TEST: Self = Self(80);
    pub const STEAM2_EMULATOR: Self = Self(79);
    pub const LOCALIZATION: Self = Self(78);
    pub const MARKET_SEARCH: Self = Self(77);
    pub const MARKET_REPL: Self = Self(76);
    pub const GAME_NOTIFICATIONS: Self = Self(75);
    pub const EXTERNAL_CONFIG: Self = Self(74);
    pub const DEPOT_WEB_CONTENT: Self = Self(73);
    pub const ES: Self = Self(72);
    pub const PARTNER: Self = Self(71);
    pub const PARTNER_UPLOAD: Self = Self(70);
    pub const PARENTAL: Self = Self(69);
    pub const EXTERNAL_MONITOR: Self = Self(68);
    pub const TAX_FORM: Self = Self(67);
    pub const PNP: Self = Self(66);
    pub const ACS: Self = Self(65);
    pub const WDS: Self = Self(64);
    pub const QUEST: Self = Self(63);
    pub const MARKET: Self = Self(62);
    pub const LOGSINK: Self = Self(61);
    pub const SECRETS: Self = Self(60);
    pub const TRADE: Self = Self(59);
    pub const MPAS: Self = Self(58);
    pub const GCH: Self = Self(57);
    pub const BRP: Self = Self(56);
    pub const WORKSHOP: Self = Self(55);
    pub const UMQ: Self = Self(54);
    pub const CRE: Self = Self(53);
    pub const MONEY_STATS: Self = Self(52);
    pub const STORE_FEATURE: Self = Self(51);
    // renamed to StoreFeature
    // pub const STORE: Self = Self(51);
    pub const UGS: Self = Self(50);
    pub const BACKPACK: Self = Self(49);
    pub const ECON: Self = Self(48);
    pub const FS: Self = Self(47);
    pub const RM: Self = Self(46);
    pub const UCM: Self = Self(45);
    pub const KGS: Self = Self(44);
    pub const GMS: Self = Self(43);
    pub const MMS: Self = Self(42);
    pub const UDS: Self = Self(41);
    pub const WEB_API: Self = Self(40);
    pub const OGS: Self = Self(39);
    pub const NS: Self = Self(38);
    pub const GC: Self = Self(37);
    pub const CS: Self = Self(36);
    pub const MDS: Self = Self(35);
    pub const LBS: Self = Self(34);
    pub const DFS: Self = Self(33);
    pub const CCS: Self = Self(32);
    pub const IS: Self = Self(31);
    pub const PS: Self = Self(30);
    pub const SITE_LICENSE: Self = Self(29);
    // pub const EPMOBSOLETE: Self = Self(29);
    // pub const EPM: Self = Self(29);
    pub const FTS: Self = Self(28);
    pub const SPARE: Self = Self(27);
    pub const APP_INFORMATION: Self = Self(26);
    // pub const P2P_RELAY_OBSOLETE: Self = Self(25);
    pub const COMMUNITY: Self = Self(24);
    // renamed to Community
    // pub const DSS: Self = Self(24);
    // pub const UTIL: Self = Self(23);
    pub const UFS: Self = Self(21);
    pub const SLC: Self = Self(20);
    pub const SM: Self = Self(19);
    pub const WG: Self = Self(18);
    pub const DP: Self = Self(17);
    pub const CONTENT_STATS: Self = Self(16);
    // pub const BOOTSTRAP_OBSOLETE: Self = Self(16);
    // pub const CLIENT: Self = Self(15);
    pub const PICS: Self = Self(14);
    // pub const ASBOBSOLETE: Self = Self(14);
    pub const CONSOLE: Self = Self(13);
    // pub const HUB_OBSOLETE: Self = Self(12);
    pub const DRMS: Self = Self(11);
    pub const SS: Self = Self(10);
    pub const BOX_MONITOR: Self = Self(9);
    // renamed to BoxMonitor
    // pub const FG: Self = Self(9);
    pub const FBS: Self = Self(8);
    pub const CM: Self = Self(7);
    pub const ATS: Self = Self(6);
    pub const VS: Self = Self(5);
    pub const BS: Self = Self(4);
    pub const AM: Self = Self(3);
    // pub const BUMOBOSOLETE: Self = Self(2);
    // pub const BUM: Self = Self(2);
    pub const GM: Self = Self(1);
    pub const SHELL: Self = Self(0);
    pub const FIRST: Self = Self(0);
    pub const INVALID: Self = Self(-1);
    pub const C_ECON_BASE: Self = Self(-5);
    pub const C_SERVER: Self = Self(-4);
    pub const CLIENT: Self = Self(-3);
    pub const UTIL: Self = Self(-2);
}

impl PartialEq<i32> for EServerType {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<EServerType> for i32 {
    fn eq(&self, other: &EServerType) -> bool {
        *self == other.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EOSType(pub i32);

impl EOSType {
    pub const WIN_MAX: Self = Self(21);
    pub const WIN11: Self = Self(20);
    pub const WIN2022: Self = Self(19);
    pub const WIN2019: Self = Self(18);
    pub const WIN2016: Self = Self(17);
    pub const WINDOWS10: Self = Self(16);
    // renamed to Windows10
    // pub const WIN10: Self = Self(16);
    pub const WIN2012_R2: Self = Self(15);
    pub const WINDOWS81: Self = Self(14);
    // renamed to Windows81
    // pub const WIN81: Self = Self(14);
    pub const WINDOWS8: Self = Self(13);
    // renamed to Windows8
    // pub const WIN8: Self = Self(13);
    pub const WIN2012: Self = Self(12);
    pub const WIN2008: Self = Self(11);
    pub const WINDOWS7: Self = Self(10);
    // renamed to Windows7
    // pub const WIN7: Self = Self(10);
    pub const WIN_VISTA: Self = Self(9);
    pub const WIN2003: Self = Self(8);
    pub const WIN_XP: Self = Self(7);
    pub const WIN2000: Self = Self(6);
    // renamed to Win2000
    // pub const WIN200: Self = Self(6);
    pub const WIN_NT: Self = Self(5);
    pub const WIN_ME: Self = Self(4);
    pub const WIN98: Self = Self(3);
    pub const WIN95: Self = Self(2);
    pub const WIN311: Self = Self(1);
    pub const WIN_UNKNOWN: Self = Self(0);
    pub const LINUX_MAX: Self = Self(-101);
    pub const LINUX510: Self = Self(-182);
    pub const LINUX7X: Self = Self(-183);
    pub const LINUX6X: Self = Self(-184);
    pub const LINUX54: Self = Self(-185);
    pub const LINUX5X: Self = Self(-186);
    pub const LINUX419: Self = Self(-187);
    pub const LINUX414: Self = Self(-188);
    pub const LINUX49: Self = Self(-189);
    pub const LINUX44: Self = Self(-190);
    pub const LINUX41: Self = Self(-191);
    pub const LINUX4X: Self = Self(-192);
    pub const LINUX3X: Self = Self(-193);
    pub const LINUX318: Self = Self(-194);
    pub const LINUX316: Self = Self(-195);
    pub const LINUX310: Self = Self(-196);
    pub const LINUX36: Self = Self(-197);
    pub const LINUX35: Self = Self(-198);
    pub const LINUX32: Self = Self(-199);
    pub const LINUX26: Self = Self(-200);
    pub const LINUX24: Self = Self(-201);
    pub const LINUX22: Self = Self(-202);
    pub const LINUX_UNKNOWN: Self = Self(-203);
    pub const MAC_OS_MAX: Self = Self(-1);
    pub const MAC_OS13: Self = Self(-76);
    pub const MAC_OS12: Self = Self(-77);
    pub const MAC_OS1017: Self = Self(-78);
    pub const MAC_OS111: Self = Self(-79);
    pub const MAC_OS11: Self = Self(-80);
    pub const MAC_OS1016: Self = Self(-81);
    pub const MACOS1015: Self = Self(-82);
    pub const MACOS1014: Self = Self(-83);
    pub const MACOS1013: Self = Self(-84);
    pub const MAC_OS1012: Self = Self(-85);
    pub const MAC_OS1011: Self = Self(-86);
    pub const MAC_OS1010: Self = Self(-87);
    pub const MAC_OS109: Self = Self(-88);
    pub const MAC_OS108: Self = Self(-89);
    pub const MAC_OS107: Self = Self(-90);
    pub const MAC_OS1067: Self = Self(-92);
    pub const MAC_OS1064_SLGU: Self = Self(-93);
    pub const MAC_OS1063: Self = Self(-94);
    pub const MAC_OS106: Self = Self(-95);
    pub const MAC_OS1058: Self = Self(-99);
    pub const MAC_OS105: Self = Self(-100);
    pub const MAC_OS104: Self = Self(-101);
    pub const MAC_OS_UNKNOWN: Self = Self(-102);
    pub const PS3: Self = Self(-300);
    pub const UMQ: Self = Self(-400);
    pub const ANDROID9: Self = Self(-496);
    pub const ANDROID8: Self = Self(-497);
    pub const ANDROID7: Self = Self(-498);
    pub const ANDROID6: Self = Self(-499);
    pub const ANDROID_UNKNOWN: Self = Self(-500);
    pub const IOS12_1: Self = Self(-571);
    pub const IOS12: Self = Self(-572);
    pub const IOS11_4: Self = Self(-573);
    pub const IOS11_3: Self = Self(-574);
    pub const IOS11_2: Self = Self(-575);
    pub const IOS11_1: Self = Self(-576);
    pub const IOS11: Self = Self(-577);
    pub const IOS10_3: Self = Self(-578);
    pub const IOS10_2: Self = Self(-579);
    pub const IOS10_1: Self = Self(-580);
    pub const IOS10: Self = Self(-581);
    pub const IOS9_3: Self = Self(-582);
    pub const IOS9_2: Self = Self(-583);
    pub const IOS9_1: Self = Self(-584);
    pub const IOS9: Self = Self(-585);
    pub const IOS8_4: Self = Self(-586);
    pub const IOS8_3: Self = Self(-587);
    pub const IOS8_2: Self = Self(-588);
    pub const IOS8_1: Self = Self(-589);
    pub const IOS8: Self = Self(-590);
    pub const IOS7_1: Self = Self(-591);
    pub const IOS7: Self = Self(-592);
    pub const IOS6_1: Self = Self(-593);
    pub const IOS6: Self = Self(-594);
    pub const IOS5: Self = Self(-595);
    pub const IOS4: Self = Self(-596);
    pub const IOS3: Self = Self(-597);
    pub const IOS2: Self = Self(-598);
    pub const IOS1: Self = Self(-599);
    pub const IOS_UNKNOWN: Self = Self(-600);
    pub const WEB: Self = Self(-700);
    pub const UNKNOWN: Self = Self(-1);
}

impl PartialEq<i32> for EOSType {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<EOSType> for i32 {
    fn eq(&self, other: &EOSType) -> bool {
        *self == other.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EPlatformType(pub i32);

impl EPlatformType {
    pub const LINUX32: Self = Self(6);
    pub const PS3: Self = Self(5);
    pub const OSX: Self = Self(4);
    pub const LINUX64: Self = Self(3);
    // split to Linux64 and Linux32
    // pub const LINUX: Self = Self(3);
    pub const WIN64: Self = Self(2);
    pub const WIN32: Self = Self(1);
    pub const UNKNOWN: Self = Self(0);
}

impl PartialEq<i32> for EPlatformType {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<EPlatformType> for i32 {
    fn eq(&self, other: &EPlatformType) -> bool {
        *self == other.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EContentDownloadSourceType(pub i32);

impl EContentDownloadSourceType {
    pub const LAN_CACHE: Self = Self(9);
    pub const OPEN_CACHE: Self = Self(8);
    pub const STEAM_CACHE: Self = Self(7);
    pub const SLS: Self = Self(6);
    pub const LAN_PEER: Self = Self(5);
    pub const PROXY_CACHE: Self = Self(4);
    pub const LCS: Self = Self(3);
    pub const CDN: Self = Self(2);
    pub const CS: Self = Self(1);
    pub const INVALID: Self = Self(0);
}

impl PartialEq<i32> for EContentDownloadSourceType {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<EContentDownloadSourceType> for i32 {
    fn eq(&self, other: &EContentDownloadSourceType) -> bool {
        *self == other.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EAppInfoSection(pub i32);

impl EAppInfoSection {
    pub const ALBUMMETADATA: Self = Self(20);
    pub const COMPUTED: Self = Self(19);
    pub const BROADCASTGAMEDATA: Self = Self(18);
    pub const LOCALIZATION: Self = Self(17);
    pub const STORE: Self = Self(16);
    pub const COMMUNITY: Self = Self(15);
    pub const SYS_REQS: Self = Self(14);
    pub const POLICIES: Self = Self(13);
    // pub const ITEMS_UNUSED: Self = Self(12);
    // pub const ITEMS: Self = Self(12);
    pub const OGG: Self = Self(11);
    pub const UFS: Self = Self(10);
    // pub const DRM_UNUSED: Self = Self(9);
    // pub const DRM: Self = Self(9);
    // pub const VAC_UNUSED: Self = Self(8);
    // pub const VAC: Self = Self(8);
    pub const DEPOTS: Self = Self(7);
    pub const INSTALL: Self = Self(6);
    pub const STATS: Self = Self(5);
    pub const CONFIG: Self = Self(4);
    pub const EXTENDED: Self = Self(3);
    pub const COMMON: Self = Self(2);
    pub const FIRST: Self = Self(2);
    pub const ALL: Self = Self(1);
    pub const UNKNOWN: Self = Self(0);
}

impl PartialEq<i32> for EAppInfoSection {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<EAppInfoSection> for i32 {
    fn eq(&self, other: &EAppInfoSection) -> bool {
        *self == other.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EChatActionResult(pub i32);

impl EChatActionResult {
    pub const VOICE_SLOTS_FULL: Self = Self(10);
    pub const CHAT_FULL: Self = Self(9);
    pub const CHAT_DOESNT_EXIST: Self = Self(8);
    pub const NOT_ALLOWED_ON_SELF: Self = Self(7);
    pub const NOT_ALLOWED_ON_CHAT_OWNER: Self = Self(6);
    pub const NOT_ALLOWED_ON_BANNED_USER: Self = Self(5);
    pub const NOT_ALLOWED_ON_CLAN_MEMBER: Self = Self(4);
    pub const NOT_PERMITTED: Self = Self(3);
    pub const ERROR: Self = Self(2);
    pub const SUCCESS: Self = Self(1);
}

impl PartialEq<i32> for EChatActionResult {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<EChatActionResult> for i32 {
    fn eq(&self, other: &EChatActionResult) -> bool {
        *self == other.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EChatAction(pub i32);

impl EChatAction {
    pub const SET_UNMODERATED: Self = Self(16);
    pub const SET_MODERATED: Self = Self(15);
    pub const SET_VISIBLE_TO_FRIENDS: Self = Self(14);
    pub const SET_INVISIBLE_TO_FRIENDS: Self = Self(13);
    pub const SET_OWNER: Self = Self(12);
    pub const SET_UNJOINABLE: Self = Self(11);
    pub const SET_JOINABLE: Self = Self(10);
    pub const CLOSE_CHAT: Self = Self(9);
    pub const UNLOCK_CHAT: Self = Self(8);
    pub const LOCK_CHAT: Self = Self(7);
    pub const END_VOICE_SPEAK: Self = Self(6);
    pub const START_VOICE_SPEAK: Self = Self(5);
    pub const UN_BAN: Self = Self(4);
    pub const BAN: Self = Self(3);
    pub const KICK: Self = Self(2);
    pub const INVITE_CHAT: Self = Self(1);
}

impl PartialEq<i32> for EChatAction {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<EChatAction> for i32 {
    fn eq(&self, other: &EChatAction) -> bool {
        *self == other.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EChatInfoType(pub i32);

impl EChatInfoType {
    pub const MEMBER_LIMIT_CHANGE: Self = Self(3);
    pub const INFO_UPDATE: Self = Self(2);
    pub const STATE_CHANGE: Self = Self(1);
}

impl PartialEq<i32> for EChatInfoType {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<EChatInfoType> for i32 {
    fn eq(&self, other: &EChatInfoType) -> bool {
        *self == other.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EChatRoomType(pub i32);

impl EChatRoomType {
    pub const LOBBY: Self = Self(3);
    pub const MUC: Self = Self(2);
    pub const FRIEND: Self = Self(1);
}

impl PartialEq<i32> for EChatRoomType {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<EChatRoomType> for i32 {
    fn eq(&self, other: &EChatRoomType) -> bool {
        *self == other.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EChatRoomEnterResponse(pub i32);

impl EChatRoomEnterResponse {
    // pub const RANK_OUT_OF_RANGE: Self = Self(14);
    // pub const NO_RANKING_DATA_USER: Self = Self(13);
    // pub const NO_RANKING_DATA_LOBBY: Self = Self(12);
    pub const YOU_BLOCKED_MEMBER: Self = Self(11);
    pub const MEMBER_BLOCKED_YOU: Self = Self(10);
    pub const COMMUNITY_BAN: Self = Self(9);
    pub const CLAN_DISABLED: Self = Self(8);
    pub const LIMITED: Self = Self(7);
    pub const BANNED: Self = Self(6);
    pub const ERROR: Self = Self(5);
    pub const FULL: Self = Self(4);
    pub const NOT_ALLOWED: Self = Self(3);
    pub const DOESNT_EXIST: Self = Self(2);
    pub const SUCCESS: Self = Self(1);
}

impl PartialEq<i32> for EChatRoomEnterResponse {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<EChatRoomEnterResponse> for i32 {
    fn eq(&self, other: &EChatRoomEnterResponse) -> bool {
        *self == other.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EAuthSessionResponse(pub i32);

impl EAuthSessionResponse {
    pub const PUBLISHER_ISSUED_BAN: Self = Self(9);
    pub const AUTH_TICKET_INVALID: Self = Self(8);
    pub const AUTH_TICKET_INVALID_ALREADY_USED: Self = Self(7);
    pub const AUTH_TICKET_CANCELED: Self = Self(6);
    pub const VAC_CHECK_TIMED_OUT: Self = Self(5);
    pub const LOGGED_IN_ELSE_WHERE: Self = Self(4);
    pub const VAC_BANNED: Self = Self(3);
    pub const NO_LICENSE_OR_EXPIRED: Self = Self(2);
    pub const USER_NOT_CONNECTED_TO_STEAM: Self = Self(1);
    pub const OK: Self = Self(0);
}

impl PartialEq<i32> for EAuthSessionResponse {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<EAuthSessionResponse> for i32 {
    fn eq(&self, other: &EAuthSessionResponse) -> bool {
        *self == other.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EClanRelationship(pub i32);

impl EClanRelationship {
    pub const REQUEST_DENIED: Self = Self(7);
    pub const PENDING_APPROVAL: Self = Self(6);
    pub const KICK_ACKNOWLEDGED: Self = Self(5);
    pub const KICKED: Self = Self(4);
    pub const MEMBER: Self = Self(3);
    pub const INVITED: Self = Self(2);
    pub const BLOCKED: Self = Self(1);
    pub const NONE: Self = Self(0);
}

impl PartialEq<i32> for EClanRelationship {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<EClanRelationship> for i32 {
    fn eq(&self, other: &EClanRelationship) -> bool {
        *self == other.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EClanRank(pub i32);

impl EClanRank {
    pub const MODERATOR: Self = Self(4);
    pub const MEMBER: Self = Self(3);
    pub const OFFICER: Self = Self(2);
    pub const OWNER: Self = Self(1);
    pub const NONE: Self = Self(0);
}

impl PartialEq<i32> for EClanRank {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<EClanRank> for i32 {
    fn eq(&self, other: &EClanRank) -> bool {
        *self == other.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EDenyReason(pub i32);

impl EDenyReason {
    pub const STEAM_OWNER_LEFT_GUEST_USER: Self = Self(15);
    pub const STEAM_VALIDATION_STALLED: Self = Self(14);
    pub const STEAM_RESPONSE_TIMED_OUT: Self = Self(13);
    pub const STEAM_CONNECTION_ERROR: Self = Self(12);
    pub const STEAM_CONNECTION_LOST: Self = Self(11);
    pub const INCOMPATIBLE_SOFTWARE: Self = Self(10);
    pub const MEMORY_CORRUPTION: Self = Self(9);
    pub const INCOMPATIBLE_ANTICHEAT: Self = Self(8);
    pub const UNKNOWN_TEXT: Self = Self(7);
    pub const LOGGED_IN_ELSE_WHERE: Self = Self(6);
    pub const CHEATER: Self = Self(5);
    pub const NO_LICENSE: Self = Self(4);
    pub const NOT_LOGGED_ON: Self = Self(3);
    pub const GENERIC: Self = Self(2);
    pub const INVALID_VERSION: Self = Self(1);
}

impl PartialEq<i32> for EDenyReason {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<EDenyReason> for i32 {
    fn eq(&self, other: &EDenyReason) -> bool {
        *self == other.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EServerFlags(pub i32);

impl EServerFlags {
    pub const PRIVATE: Self = Self(32);
    pub const PASSWORDED: Self = Self(16);
    pub const LINUX: Self = Self(8);
    pub const DEDICATED: Self = Self(4);
    pub const SECURE: Self = Self(2);
    pub const ACTIVE: Self = Self(1);
    pub const NONE: Self = Self(0);
}

impl PartialEq<i32> for EServerFlags {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<EServerFlags> for i32 {
    fn eq(&self, other: &EServerFlags) -> bool {
        *self == other.0
    }
}

impl ops::BitOr for EServerFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EIntroducerRouting(pub i32);

impl EIntroducerRouting {
    pub const P2P_NETWORKING: Self = Self(2);
    pub const P2P_VOICE_CHAT: Self = Self(1);
    // pub const FILE_SHARE: Self = Self(0);
}

impl PartialEq<i32> for EIntroducerRouting {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<EIntroducerRouting> for i32 {
    fn eq(&self, other: &EIntroducerRouting) -> bool {
        *self == other.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EPurchaseResultDetail(pub i32);

impl EPurchaseResultDetail {
    pub const PAYMENT_METHOD_NOT_SUPPORTED_FOR_PRODUCT: Self = Self(83);
    pub const PAYMENT_METHOD_TEMPORARILY_UNAVAILABLE: Self = Self(82);
    pub const BILLING_NAME_INVALID_RESEMBLES_CREDIT_CARD: Self = Self(81);
    pub const CANNOT_SHIP_TO_MILITARY_POST_OFFICE: Self = Self(80);
    pub const CREDIT_CARD_NUMBER_INVALID: Self = Self(79);
    pub const ADDRESS_INVALID: Self = Self(78);
    pub const USER_NOT_ASSOCIATED_WITH_CAFE: Self = Self(77);
    pub const USER_ASSOCIATED_WITH_MANY_CAFES: Self = Self(76);
    pub const BUSINESS_STORE_COUNTRY_CODE_MISMATCH: Self = Self(75);
    pub const ITEMS_NOT_ALLOWED_FOR_COMMERCIAL_USE: Self = Self(74);
    pub const GIFT_RECIPIENT_NOT_SPECIFIED: Self = Self(73);
    pub const GIFT_PRICING_IMBALANCE: Self = Self(72);
    pub const GIFT_INVALID_FOR_RECIPIENT_REGION: Self = Self(71);
    pub const GIFT_ALREADY_OWNED: Self = Self(70);
    pub const ITEMS_RESERVED_FOR_COMMERCIAL_USE: Self = Self(69);
    pub const BLOCKED_BY_US_GOV: Self = Self(68);
    pub const BUNDLE_TYPE_CANNOT_BE_GIFTED: Self = Self(67);
    pub const DELAYED_COMPLETION: Self = Self(66);
    pub const PURCHASE_CANNOT_BE_REPLAYED: Self = Self(65);
    pub const PHYSICAL_PRODUCT_LIMIT_EXCEEDED: Self = Self(64);
    pub const INVALID_TAX_ADDRESS: Self = Self(63);
    pub const UNKNOWN_GLOBAL_COLLECT_ERROR: Self = Self(62);
    pub const PAYPAL_INTERNAL_ERROR: Self = Self(61);
    pub const HUNG_TRANSACTION_CANCELLED: Self = Self(60);
    pub const CANNOT_SHIP_TO_COUNTRY: Self = Self(59);
    pub const POSA_CODE_NOT_ACTIVATED: Self = Self(58);
    pub const BILLING_AGREEMENT_ALREADY_EXISTS: Self = Self(57);
    pub const CART_VALUE_TOO_HIGH: Self = Self(56);
    pub const CREDIT_CARD_BIN_MISMATCHES_TYPE: Self = Self(55);
    pub const OWNS_EXCLUDED_APP: Self = Self(54);
    pub const RATE_LIMITED: Self = Self(53);
    pub const OVERLAPPING_PACKAGES_IN_PENDING_TRANSACTION: Self = Self(52);
    pub const PURCHASE_AMOUNT_NO_SUPPORTED_BY_PROVIDER: Self = Self(51);
    pub const CANNOT_REDEEM_CODE_FROM_CLIENT: Self = Self(50);
    pub const NO_CACHED_PAYMENT_METHOD: Self = Self(49);
    pub const NO_WALLET: Self = Self(48);
    pub const OVERLAPPING_PACKAGES_IN_CART: Self = Self(47);
    pub const EXCEEDED_STEAM_LIMIT: Self = Self(46);
    pub const OTHER_ABORTABLE_IN_PROGRESS: Self = Self(45);
    pub const ACCOUNT_LOCKED: Self = Self(44);
    pub const EXPIRED_COUPON: Self = Self(43);
    pub const INVALID_COUPON: Self = Self(42);
    pub const BILLING_AGREEMENT_CANCELLED: Self = Self(41);
    pub const CANNOT_SHIP_INTERNATIONALLY: Self = Self(40);
    pub const CANNOT_GIFT_SHIPPED_GOODS: Self = Self(39);
    pub const INSUFFICIENT_INVENTORY: Self = Self(38);
    pub const CANNOT_SHIP_TO_PO_BOX: Self = Self(37);
    pub const MUST_LOGIN_PS3_APP_FOR_PURCHASE: Self = Self(36);
    pub const WOULD_EXCEED_MAX_WALLET: Self = Self(35);
    pub const TRANSACTION_EXPIRED: Self = Self(34);
    pub const EXPIRED_CARD: Self = Self(33);
    pub const EMAIL_NOT_VALIDATED: Self = Self(32);
    pub const WALLET_CURRENCY_MISMATCH: Self = Self(31);
    pub const PRE_APPROVAL_DENIED: Self = Self(30);
    pub const NEEDS_PRE_APPROVAL: Self = Self(29);
    pub const FAILED_CYBER_CAFE: Self = Self(28);
    pub const FAIL_CURRENCY_TRANS_PROVIDER: Self = Self(27);
    pub const FORCE_CANCELED_PENDING: Self = Self(26);
    pub const CANCELED_BY_NEW_TRANSACTION: Self = Self(25);
    pub const DOES_NOT_OWN_REQUIRED_APP: Self = Self(24);
    pub const STORE_BILLING_COUNTRY_MISMATCH: Self = Self(23);
    pub const INVALID_ACCOUNT: Self = Self(22);
    pub const ACCT_NOT_VERIFIED: Self = Self(21);
    pub const ACCT_IS_BLOCKED: Self = Self(20);
    pub const REGION_NOT_SUPPORTED: Self = Self(19);
    pub const INVALID_SHIPPING_ADDRESS: Self = Self(18);
    pub const USE_OTHER_FUNCTION_SOURCE: Self = Self(17);
    pub const USE_OTHER_PAYMENT_METHOD: Self = Self(16);
    pub const DUPLICATE_ACTIVATION_CODE: Self = Self(15);
    pub const BAD_ACTIVATION_CODE: Self = Self(14);
    pub const RESTRICTED_COUNTRY: Self = Self(13);
    pub const CANCELLED_BY_USER: Self = Self(12);
    pub const FRAUD_CHECK_FAILED: Self = Self(11);
    pub const WRONG_PRICE: Self = Self(10);
    pub const ALREADY_PURCHASED: Self = Self(9);
    pub const OTHERS_IN_PROGRESS: Self = Self(8);
    pub const INVALID_DATA: Self = Self(7);
    pub const INVALID_PAYMENT_METHOD: Self = Self(6);
    pub const INVALID_PACKAGE: Self = Self(5);
    pub const TIMEOUT: Self = Self(4);
    pub const CONTACT_SUPPORT: Self = Self(3);
    pub const INSUFFICIENT_FUNDS: Self = Self(2);
    pub const AVS_FAILURE: Self = Self(1);
    pub const NO_DETAIL: Self = Self(0);
}

impl PartialEq<i32> for EPurchaseResultDetail {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<EPurchaseResultDetail> for i32 {
    fn eq(&self, other: &EPurchaseResultDetail) -> bool {
        *self == other.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EPaymentMethod(pub i32);

impl EPaymentMethod {
    pub const COMPLIMENTARY: Self = Self(1024);
    pub const SPLIT: Self = Self(512);
    pub const OEM_TICKET: Self = Self(256);
    pub const MOBILE_WALLET_JAPAN: Self = Self(136);
    pub const PAYCO: Self = Self(135);
    pub const MASTER_SUBSCRIPTION: Self = Self(134);
    pub const PROMOTIONAL: Self = Self(131);
    // renamed to Promotional
    // pub const STORE_PROMOTION: Self = Self(131);
    pub const MASTER_COMP: Self = Self(130);
    // renamed to MasterComp
    // pub const STEAM_PRESS_MASTER: Self = Self(130);
    pub const VALVE: Self = Self(129);
    pub const WALLET: Self = Self(128);
    pub const BIT_COIN: Self = Self(79);
    pub const UNION_PAY: Self = Self(78);
    pub const TRUSTLY: Self = Self(77);
    pub const PAGO_EFECTIVO: Self = Self(76);
    pub const SAFETY_PAY: Self = Self(75);
    pub const BBVA_CONTINENTAL: Self = Self(74);
    pub const BANCO_CREDITO_DE_PERU: Self = Self(73);
    pub const MANGIR_KART: Self = Self(72);
    pub const PIN_VALIDDA: Self = Self(71);
    pub const PALOTO: Self = Self(70);
    pub const EFECTY: Self = Self(69);
    pub const EXITO: Self = Self(68);
    pub const PSE: Self = Self(67);
    pub const ONE_CARD: Self = Self(66);
    pub const WEB_MONEY_JAPAN: Self = Self(65);
    pub const AUTO_GRANT: Self = Self(64);
    pub const CASH_U: Self = Self(62);
    pub const PTT: Self = Self(61);
    pub const DENIZ_BANK: Self = Self(60);
    pub const FINANSBANK: Self = Self(59);
    pub const BANK_ASYA: Self = Self(58);
    pub const HALKBANK: Self = Self(57);
    pub const YAPI_KREDI: Self = Self(56);
    pub const AKBANK: Self = Self(55);
    pub const GARANTI: Self = Self(54);
    pub const IS_BANK: Self = Self(53);
    pub const THREE_PAY: Self = Self(52);
    pub const SPEI: Self = Self(51);
    pub const CARNET: Self = Self(50);
    pub const TODITO_CASH: Self = Self(49);
    pub const OXXO: Self = Self(48);
    pub const MAESTRO_BOA_COMPRA: Self = Self(47);
    // renamed to MaestroBoaCompra
    // pub const MAESTRO: Self = Self(47);
    pub const PAYSHOP: Self = Self(46);
    pub const MULTIBANCO: Self = Self(45);
    pub const GAME_VOUCHER: Self = Self(44);
    pub const CONVENIENT_STORE_VOUCHER: Self = Self(43);
    pub const HAPPYMONEY_VOUCHER: Self = Self(42);
    pub const BOOK_VOUCHER: Self = Self(41);
    pub const CULTURE_VOUCHER: Self = Self(40);
    pub const ZONG: Self = Self(39);
    pub const PAY_EASY: Self = Self(38);
    // renamed to PayEasy
    // pub const PAY_EASY_JAPAN: Self = Self(38);
    pub const BANK_TRANSFER_JAPAN: Self = Self(37);
    pub const CREDIT_CARD_JAPAN: Self = Self(36);
    pub const E_CLUB_POINTS: Self = Self(35);
    pub const KONBINI: Self = Self(34);
    pub const BEELINE: Self = Self(33);
    pub const CLICK_AND_BUY: Self = Self(32);
    pub const MOL_POINTS: Self = Self(31);
    pub const AUTHORIZED_DEVICE: Self = Self(30);
    pub const DINERS_CARD_BRAZIL: Self = Self(29);
    pub const MASTERCARD_BRAZIL: Self = Self(28);
    pub const HIPERCARD: Self = Self(27);
    pub const AURA: Self = Self(26);
    pub const AMEX_BRAZIL: Self = Self(25);
    pub const VISA_BRAZIL: Self = Self(24);
    pub const PAGSEGURO: Self = Self(23);
    pub const BRADESCO_ONLINE: Self = Self(22);
    pub const ITAU_ONLINE: Self = Self(21);
    pub const BANCO_DO_BRASIL_ONLINE: Self = Self(20);
    pub const BOA_COMPRA_GOLD: Self = Self(19);
    pub const BOLETO_BANCARIO: Self = Self(18);
    pub const MO_PAY: Self = Self(17);
    pub const HARDWARE_PROMO: Self = Self(16);
    pub const GAME_STOP: Self = Self(15);
    pub const QIWI: Self = Self(14);
    pub const KIOSK: Self = Self(13);
    pub const YANDEX: Self = Self(12);
    pub const ALI_PAY: Self = Self(11);
    pub const MONEY_BOOKERS: Self = Self(10);
    pub const WEB_MONEY: Self = Self(9);
    pub const GUEST_PASS: Self = Self(8);
    pub const SOFORT: Self = Self(7);
    pub const PAY_SAFE_CARD: Self = Self(6);
    pub const IDEAL: Self = Self(5);
    pub const PAY_PAL: Self = Self(4);
    pub const GIROPAY: Self = Self(3);
    pub const CREDIT_CARD: Self = Self(2);
    pub const ACTIVATION_CODE: Self = Self(1);
    pub const NONE: Self = Self(0);
}

impl PartialEq<i32> for EPaymentMethod {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<EPaymentMethod> for i32 {
    fn eq(&self, other: &EPaymentMethod) -> bool {
        *self == other.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ELicenseType(pub i32);

impl ELicenseType {
    pub const LIMITED_USE_DELAYED_ACTIVATION: Self = Self(7);
    pub const RECURRING_OPTION: Self = Self(6);
    pub const RECURRING_CHARGE_LIMITED_USE_WITH_OVERAGES: Self = Self(5);
    pub const RECURRING_CHARGE_LIMITED_USE: Self = Self(4);
    pub const RECURRING_CHARGE: Self = Self(3);
    pub const SINGLE_PURCHASE_LIMITED_USE: Self = Self(2);
    pub const SINGLE_PURCHASE: Self = Self(1);
    pub const NO_LICENSE: Self = Self(0);
}

impl PartialEq<i32> for ELicenseType {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<ELicenseType> for i32 {
    fn eq(&self, other: &ELicenseType) -> bool {
        *self == other.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ELicenseFlags(pub i32);

impl ELicenseFlags {
    pub const NOT_ACTIVATED: Self = Self(0x800);
    pub const CANCELLED_BY_FRIENDLY_FRAUD_LOCK: Self = Self(0x400);
    pub const REGION_RESTRICTION_EXPIRED: Self = Self(0x200);
    pub const FORCE_RUN_RESTRICTION: Self = Self(0x100);
    pub const IMPORTED_FROM_STEAM2: Self = Self(0x80);
    pub const LOW_VIOLENCE_CONTENT: Self = Self(0x40);
    pub const CANCELLED_BY_ADMIN: Self = Self(0x20);
    pub const CANCELLED_BY_USER: Self = Self(0x10);
    pub const EXPIRED: Self = Self(0x08);
    pub const PENDING: Self = Self(0x04);
    pub const RENEWAL_FAILED: Self = Self(0x02);
    pub const RENEW: Self = Self(0x01);
    pub const NONE: Self = Self(0);
}

impl PartialEq<i32> for ELicenseFlags {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<ELicenseFlags> for i32 {
    fn eq(&self, other: &ELicenseFlags) -> bool {
        *self == other.0
    }
}

impl ops::BitOr for ELicenseFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EAppUsageEvent(pub i32);

impl EAppUsageEvent {
    pub const GAME_LAUNCH_FREE_WEEKEND: Self = Self(8);
    pub const IN_GAME_AD_VIEWED: Self = Self(7);
    pub const MARKETING_MESSAGE_VIEW: Self = Self(6);
    pub const PRELOAD_FINISH: Self = Self(5);
    pub const PRELOAD_START: Self = Self(4);
    pub const MEDIA: Self = Self(3);
    pub const GAME_LAUNCH_TRIAL: Self = Self(2);
    pub const GAME_LAUNCH: Self = Self(1);
}

impl PartialEq<i32> for EAppUsageEvent {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<EAppUsageEvent> for i32 {
    fn eq(&self, other: &EAppUsageEvent) -> bool {
        *self == other.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EClientPersonaStateFlag(pub i32);

impl EClientPersonaStateFlag {
    pub const WATCHING: Self = Self(16384);
    pub const BROADCAST: Self = Self(8192);
    pub const RICH_PRESENCE: Self = Self(4096);
    pub const FACEBOOK: Self = Self(2048);
    pub const CLAN_DATA: Self = Self(1024);
    #[deprecated = "renamed to ClanData"]
    pub const CLAN_TAG: Self = Self(1024);
    pub const GAME_DATA_BLOB: Self = Self(512);
    pub const GAME_EXTRA_INFO: Self = Self(256);
    pub const USER_CLAN_RANK: Self = Self(128);
    #[deprecated = "renamed to UserClanRank"]
    pub const CLAN_INFO: Self = Self(128);
    pub const LAST_SEEN: Self = Self(64);
    // pub const METADATA: Self = Self(32);
    pub const PRESENCE: Self = Self(16);
    pub const SOURCE_ID: Self = Self(8);
    pub const QUERY_PORT: Self = Self(4);
    pub const PLAYER_NAME: Self = Self(2);
    pub const STATUS: Self = Self(1);
}

impl PartialEq<i32> for EClientPersonaStateFlag {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<EClientPersonaStateFlag> for i32 {
    fn eq(&self, other: &EClientPersonaStateFlag) -> bool {
        *self == other.0
    }
}

impl ops::BitOr for EClientPersonaStateFlag {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EPersonaStateFlag(pub i32);

impl EPersonaStateFlag {
    pub const LAUNCH_TYPE_COMPAT_TOOL: Self = Self(8192);
    pub const LAUNCH_TYPE_GAMEPAD: Self = Self(4096);
    pub const CLIENT_TYPE_VR: Self = Self(2048);
    // renamed to ClientTypeVR
    // pub const ONLINE_USING_VR: Self = Self(2048);
    pub const CLIENT_TYPE_TENFOOT: Self = Self(1024);
    // renamed to ClientTypeTenfoot
    // pub const ONLINE_USING_BIG_PICTURE: Self = Self(1024);
    pub const CLIENT_TYPE_MOBILE: Self = Self(512);
    // renamed to ClientTypeMobile
    // pub const ONLINE_USING_MOBILE: Self = Self(512);
    pub const CLIENT_TYPE_WEB: Self = Self(256);
    // renamed to ClientTypeWeb
    // pub const ONLINE_USING_WEB: Self = Self(256);
    pub const REMOTE_PLAY_TOGETHER: Self = Self(8);
    pub const GOLDEN: Self = Self(4);
    pub const IN_JOINABLE_GAME: Self = Self(2);
    pub const HAS_RICH_PRESENCE: Self = Self(1);
}

impl PartialEq<i32> for EPersonaStateFlag {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<EPersonaStateFlag> for i32 {
    fn eq(&self, other: &EPersonaStateFlag) -> bool {
        *self == other.0
    }
}

impl ops::BitOr for EPersonaStateFlag {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EFriendFlags(pub i32);

impl EFriendFlags {
    pub const FLAG_ALL: Self = Self(65535);
    pub const CHAT_MEMBER: Self = Self(4096);
    pub const SUGGESTED: Self = Self(2048);
    pub const IGNORED_FRIEND: Self = Self(1024);
    pub const IGNORED: Self = Self(512);
    pub const REQUESTING_INFO: Self = Self(256);
    pub const REQUESTING_FRIENDSHIP: Self = Self(128);
    pub const ON_GAME_SERVER: Self = Self(16);
    pub const CLAN_MEMBER: Self = Self(8);
    pub const IMMEDIATE: Self = Self(4);
    pub const FRIENDSHIP_REQUESTED: Self = Self(2);
    pub const BLOCKED: Self = Self(1);
    pub const NONE: Self = Self(0);
}

impl PartialEq<i32> for EFriendFlags {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<EFriendFlags> for i32 {
    fn eq(&self, other: &EFriendFlags) -> bool {
        *self == other.0
    }
}

impl ops::BitOr for EFriendFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EChatPermission(pub i32);

impl EChatPermission {
    pub const MASK: Self = Self(1019);
    pub const OWNER_DEFAULT: Self = Self::CHANGE_ACCESS | Self::BAN | Self::SET_METADATA | Self::MUTE | Self::KICK | Self::TALK | Self::INVITE | Self::CLOSE;
    pub const OFFICER_DEFAULT: Self = Self::BAN | Self::KICK | Self::TALK | Self::INVITE;
    pub const MEMBER_DEFAULT: Self = Self::BAN | Self::KICK | Self::TALK | Self::INVITE;
    pub const EVERYONE_DEFAULT: Self = Self::TALK | Self::INVITE;
    pub const EVERYONE_NOT_IN_CLAN_DEFAULT: Self = Self::TALK;
    pub const CHANGE_ACCESS: Self = Self(512);
    pub const BAN: Self = Self(256);
    pub const CHANGE_PERMISSIONS: Self = Self(128);
    pub const SET_METADATA: Self = Self(64);
    pub const MUTE: Self = Self(32);
    pub const KICK: Self = Self(16);
    pub const TALK: Self = Self(8);
    pub const INVITE: Self = Self(2);
    pub const CLOSE: Self = Self(1);
}

impl PartialEq<i32> for EChatPermission {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<EChatPermission> for i32 {
    fn eq(&self, other: &EChatPermission) -> bool {
        *self == other.0
    }
}

impl ops::BitOr for EChatPermission {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EClanPermission(pub i32);

impl EClanPermission {
    pub const ANYBODY: Self = Self::NON_MEMBER | Self::MEMBER | Self::MODERATOR | Self::OFFICER | Self::OWNER;
    pub const OWNER_ALLOWED: Self = Self::NON_MEMBER | Self::MEMBER | Self::MODERATOR | Self::OFFICER | Self::OWNER;
    pub const OFFICER_ALLOWED: Self = Self::NON_MEMBER | Self::MEMBER | Self::MODERATOR | Self::OFFICER;
    pub const MODERATOR_ALLOWED: Self = Self::NON_MEMBER | Self::MEMBER | Self::MODERATOR;
    pub const MEMBER_ALLOWED: Self = Self::NON_MEMBER | Self::MEMBER;
    pub const NON_MEMBER: Self = Self(128);
    pub const OGG_GAME_OWNER: Self = Self(16);
    pub const ALL_MEMBERS: Self = Self::OWNER | Self::OFFICER | Self::MODERATOR | Self::MEMBER;
    pub const OWNER_OFFICER_MODERATOR: Self = Self::OWNER | Self::OFFICER | Self::MODERATOR;
    pub const MODERATOR: Self = Self(8);
    pub const MEMBER: Self = Self(4);
    pub const OWNER_AND_OFFICER: Self = Self(3);
    pub const OFFICER: Self = Self(2);
    pub const OWNER: Self = Self(1);
    pub const NOBODY: Self = Self(0);
}

impl PartialEq<i32> for EClanPermission {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<EClanPermission> for i32 {
    fn eq(&self, other: &EClanPermission) -> bool {
        *self == other.0
    }
}

impl ops::BitOr for EClanPermission {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EAccountFlags(pub i32);

impl EAccountFlags {
    pub const NEEDS_SSA_NEXT_STEAM_LOGON: Self = Self(1073741824);
    pub const THIRD_PARTY_SUPPORT: Self = Self(536870912);
    pub const PARENTAL_SETTINGS: Self = Self(268435456);
    pub const GLOBAL_MODERATOR: Self = Self(134217728);
    pub const CLANS_ONLY_FROM_FRIENDS: Self = Self(67108864);
    pub const BANNED_FROM_WEB_API: Self = Self(33554432);
    pub const MASTER_APP_EDITOR: Self = Self(16777216);
    pub const LOCKDOWN: Self = Self(8388608);
    pub const NEED_LOGS: Self = Self(4194304);
    pub const STEAM2_MIGRATION_COMPLETE: Self = Self(2097152);
    pub const LOGON_EXTRA_SECURITY_DISABLED: Self = Self(1048576);
    pub const LOGON_EXTRA_SECURITY: Self = Self(524288);
    pub const FORCE_EMAIL_VERIFICATION: Self = Self(262144);
    pub const FORCE_PASSWORD_CHANGE: Self = Self(131072);
    pub const OGG_INVITE_OPT_OUT: Self = Self(65536);
    pub const MARKETING_TREATMENT: Self = Self(32768);
    pub const EMAIL_VALIDATED: Self = Self(16384);
    pub const LIMITED_USER_FORCE: Self = Self(8192);
    pub const LIMITED_USER: Self = Self(4096);
    pub const DISABLED: Self = Self(2048);
    pub const DEBUG: Self = Self(1024);
    pub const VAC_BETA: Self = Self(512);
    pub const PERSONAL_QA_SET: Self = Self(256);
    pub const HWID_SET: Self = Self(128);
    pub const APP_EDITOR: Self = Self(64);
    pub const SUPERVISOR: Self = Self(32);
    pub const ADMIN: Self = Self(16);
    pub const SUPPORT: Self = Self(8);
    pub const PASSWORD_SET: Self = Self(4);
    pub const UNBANNABLE: Self = Self(2);
    pub const PERSONA_NAME_SET: Self = Self(1);
    pub const NORMAL_USER: Self = Self(0);
}

impl PartialEq<i32> for EAccountFlags {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<EAccountFlags> for i32 {
    fn eq(&self, other: &EAccountFlags) -> bool {
        *self == other.0
    }
}

impl ops::BitOr for EAccountFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EFriendRelationship(pub i32);

impl EFriendRelationship {
    // was used by the original implementation of the facebook linking feature; but now unused.
    // pub const SUGGESTED_FRIEND: Self = Self(7);
    pub const IGNORED_FRIEND: Self = Self(6);
    pub const IGNORED: Self = Self(5);
    pub const REQUEST_INITIATOR: Self = Self(4);
    pub const FRIEND: Self = Self(3);
    pub const REQUEST_RECIPIENT: Self = Self(2);
    pub const BLOCKED: Self = Self(1);
    pub const NONE: Self = Self(0);
}

impl PartialEq<i32> for EFriendRelationship {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<EFriendRelationship> for i32 {
    fn eq(&self, other: &EFriendRelationship) -> bool {
        *self == other.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EAccountType(pub i32);

impl EAccountType {
    pub const ANON_USER: Self = Self(10);
    pub const CONSOLE_USER: Self = Self(9);
    pub const CHAT: Self = Self(8);
    pub const CLAN: Self = Self(7);
    pub const CONTENT_SERVER: Self = Self(6);
    pub const PENDING: Self = Self(5);
    pub const ANON_GAME_SERVER: Self = Self(4);
    pub const GAME_SERVER: Self = Self(3);
    pub const MULTISEAT: Self = Self(2);
    pub const INDIVIDUAL: Self = Self(1);
    pub const INVALID: Self = Self(0);
}

impl PartialEq<i32> for EAccountType {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<EAccountType> for i32 {
    fn eq(&self, other: &EAccountType) -> bool {
        *self == other.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EPersonaState(pub i32);

impl EPersonaState {
    pub const INVISIBLE: Self = Self(7);
    pub const LOOKING_TO_PLAY: Self = Self(6);
    pub const LOOKING_TO_TRADE: Self = Self(5);
    pub const SNOOZE: Self = Self(4);
    pub const AWAY: Self = Self(3);
    pub const BUSY: Self = Self(2);
    pub const ONLINE: Self = Self(1);
    pub const OFFLINE: Self = Self(0);
}

impl PartialEq<i32> for EPersonaState {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<EPersonaState> for i32 {
    fn eq(&self, other: &EPersonaState) -> bool {
        *self == other.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EChatEntryType(pub i32);

impl EChatEntryType {
    pub const LINK_BLOCKED: Self = Self(14);
    pub const RESERVED2: Self = Self(13);
    pub const RESERVED1: Self = Self(12);
    pub const HISTORICAL_CHAT: Self = Self(11);
    pub const DISCONNECTED: Self = Self(10);
    pub const WAS_BANNED: Self = Self(9);
    pub const WAS_KICKED: Self = Self(8);
    pub const ENTERED: Self = Self(7);
    pub const LEFT_CONVERSATION: Self = Self(6);
    // Listen for LobbyGameCreated_t callback instead
    // pub const LOBBY_GAME_START: Self = Self(5);
    // No longer supported by clients
    // pub const EMOTE: Self = Self(4);
    pub const INVITE_GAME: Self = Self(3);
    pub const TYPING: Self = Self(2);
    pub const CHAT_MSG: Self = Self(1);
    pub const INVALID: Self = Self(0);
}

impl PartialEq<i32> for EChatEntryType {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<EChatEntryType> for i32 {
    fn eq(&self, other: &EChatEntryType) -> bool {
        *self == other.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EUniverse(pub i32);

impl EUniverse {
    pub const DEV: Self = Self(4);
    pub const INTERNAL: Self = Self(3);
    pub const BETA: Self = Self(2);
    pub const PUBLIC: Self = Self(1);
    pub const INVALID: Self = Self(0);
}

impl PartialEq<i32> for EUniverse {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<EUniverse> for i32 {
    fn eq(&self, other: &EUniverse) -> bool {
        *self == other.0
    }
}
