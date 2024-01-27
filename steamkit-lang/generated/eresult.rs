#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EResult(pub i32);

impl EResult {
    pub const INVALID: Self = Self(0);
    pub const OK: Self = Self(1);
    pub const FAIL: Self = Self(2);
    pub const NO_CONNECTION: Self = Self(3);
    pub const INVALID_PASSWORD: Self = Self(5);
    pub const LOGGED_IN_ELSEWHERE: Self = Self(6);
    pub const INVALID_PROTOCOL_VER: Self = Self(7);
    pub const INVALID_PARAM: Self = Self(8);
    pub const FILE_NOT_FOUND: Self = Self(9);
    pub const BUSY: Self = Self(10);
    pub const INVALID_STATE: Self = Self(11);
    pub const INVALID_NAME: Self = Self(12);
    pub const INVALID_EMAIL: Self = Self(13);
    pub const DUPLICATE_NAME: Self = Self(14);
    pub const ACCESS_DENIED: Self = Self(15);
    pub const TIMEOUT: Self = Self(16);
    pub const BANNED: Self = Self(17);
    pub const ACCOUNT_NOT_FOUND: Self = Self(18);
    pub const INVALID_STEAM_ID: Self = Self(19);
    pub const SERVICE_UNAVAILABLE: Self = Self(20);
    pub const NOT_LOGGED_ON: Self = Self(21);
    pub const PENDING: Self = Self(22);
    pub const ENCRYPTION_FAILURE: Self = Self(23);
    pub const INSUFFICIENT_PRIVILEGE: Self = Self(24);
    pub const LIMIT_EXCEEDED: Self = Self(25);
    pub const REVOKED: Self = Self(26);
    pub const EXPIRED: Self = Self(27);
    pub const ALREADY_REDEEMED: Self = Self(28);
    pub const DUPLICATE_REQUEST: Self = Self(29);
    pub const ALREADY_OWNED: Self = Self(30);
    pub const IP_NOT_FOUND: Self = Self(31);
    pub const PERSIST_FAILED: Self = Self(32);
    pub const LOCKING_FAILED: Self = Self(33);
    pub const LOGON_SESSION_REPLACED: Self = Self(34);
    pub const CONNECT_FAILED: Self = Self(35);
    pub const HANDSHAKE_FAILED: Self = Self(36);
    pub const IO_FAILURE: Self = Self(37);
    pub const REMOTE_DISCONNECT: Self = Self(38);
    pub const SHOPPING_CART_NOT_FOUND: Self = Self(39);
    pub const BLOCKED: Self = Self(40);
    pub const IGNORED: Self = Self(41);
    pub const NO_MATCH: Self = Self(42);
    pub const ACCOUNT_DISABLED: Self = Self(43);
    pub const SERVICE_READ_ONLY: Self = Self(44);
    pub const ACCOUNT_NOT_FEATURED: Self = Self(45);
    pub const ADMINISTRATOR_OK: Self = Self(46);
    pub const CONTENT_VERSION: Self = Self(47);
    pub const TRY_ANOTHER_CM: Self = Self(48);
    pub const PASSWORD_REQUIRED_TO_KICK_SESSION: Self = Self(49);
    pub const ALREADY_LOGGED_IN_ELSEWHERE: Self = Self(50);
    pub const SUSPENDED: Self = Self(51);
    pub const CANCELLED: Self = Self(52);
    pub const DATA_CORRUPTION: Self = Self(53);
    pub const DISK_FULL: Self = Self(54);
    pub const REMOTE_CALL_FAILED: Self = Self(55);
    // renamed to PasswordUnset
    // pub const PASSWORD_NOT_SET: Self = Self(56);
    pub const PASSWORD_UNSET: Self = Self(56);
    pub const EXTERNAL_ACCOUNT_UNLINKED: Self = Self(57);
    pub const PSN_TICKET_INVALID: Self = Self(58);
    pub const EXTERNAL_ACCOUNT_ALREADY_LINKED: Self = Self(59);
    pub const REMOTE_FILE_CONFLICT: Self = Self(60);
    pub const ILLEGAL_PASSWORD: Self = Self(61);
    pub const SAME_AS_PREVIOUS_VALUE: Self = Self(62);
    pub const ACCOUNT_LOGON_DENIED: Self = Self(63);
    pub const CANNOT_USE_OLD_PASSWORD: Self = Self(64);
    pub const INVALID_LOGIN_AUTH_CODE: Self = Self(65);
    // renamed to AccountLogonDeniedNoMail
    // pub const ACCOUNT_LOGON_DENIED_NO_MAIL_SENT: Self = Self(66);
    pub const ACCOUNT_LOGON_DENIED_NO_MAIL: Self = Self(66);
    pub const HARDWARE_NOT_CAPABLE_OF_IPT: Self = Self(67);
    pub const IPT_INIT_ERROR: Self = Self(68);
    pub const PARENTAL_CONTROL_RESTRICTED: Self = Self(69);
    pub const FACEBOOK_QUERY_ERROR: Self = Self(70);
    pub const EXPIRED_LOGIN_AUTH_CODE: Self = Self(71);
    pub const IP_LOGIN_RESTRICTION_FAILED: Self = Self(72);
    // renamed to AccountLockedDown
    // pub const ACCOUNT_LOCKED: Self = Self(73);
    pub const ACCOUNT_LOCKED_DOWN: Self = Self(73);
    pub const ACCOUNT_LOGON_DENIED_VERIFIED_EMAIL_REQUIRED: Self = Self(74);
    pub const NO_MATCHING_URL: Self = Self(75);
    pub const BAD_RESPONSE: Self = Self(76);
    pub const REQUIRE_PASSWORD_RE_ENTRY: Self = Self(77);
    pub const VALUE_OUT_OF_RANGE: Self = Self(78);
    pub const UNEXPECTED_ERROR: Self = Self(79);
    pub const DISABLED: Self = Self(80);
    pub const INVALID_CEG_SUBMISSION: Self = Self(81);
    pub const RESTRICTED_DEVICE: Self = Self(82);
    pub const REGION_LOCKED: Self = Self(83);
    pub const RATE_LIMIT_EXCEEDED: Self = Self(84);
    // renamed to AccountLoginDeniedNeedTwoFactor
    // pub const ACCOUNT_LOGON_DENIED_NEED_TWO_FACTOR_CODE: Self = Self(85);
    pub const ACCOUNT_LOGIN_DENIED_NEED_TWO_FACTOR: Self = Self(85);
    // renamed to ItemDeleted
    // pub const ITEM_OR_ENTRY_HAS_BEEN_DELETED: Self = Self(86);
    pub const ITEM_DELETED: Self = Self(86);
    pub const ACCOUNT_LOGIN_DENIED_THROTTLE: Self = Self(87);
    pub const TWO_FACTOR_CODE_MISMATCH: Self = Self(88);
    pub const TWO_FACTOR_ACTIVATION_CODE_MISMATCH: Self = Self(89);
    // renamed to AccountAssociatedToMultiplePartners
    // pub const ACCOUNT_ASSOCIATED_TO_MULTIPLE_PLAYERS: Self = Self(90);
    pub const ACCOUNT_ASSOCIATED_TO_MULTIPLE_PARTNERS: Self = Self(90);
    pub const NOT_MODIFIED: Self = Self(91);
    // renamed to NoMobileDevice
    // pub const NO_MOBILE_DEVICE_AVAILABLE: Self = Self(92);
    pub const NO_MOBILE_DEVICE: Self = Self(92);
    // renamed to TimeNotSynced
    // pub const TIME_IS_OUT_OF_SYNC: Self = Self(93);
    pub const TIME_NOT_SYNCED: Self = Self(93);
    pub const SMS_CODE_FAILED: Self = Self(94);
    // renamed to AccountLimitExceeded
    // pub const TOO_MANY_ACCOUNTS_ACCESS_THIS_RESOURCE: Self = Self(95);
    pub const ACCOUNT_LIMIT_EXCEEDED: Self = Self(95);
    pub const ACCOUNT_ACTIVITY_LIMIT_EXCEEDED: Self = Self(96);
    pub const PHONE_ACTIVITY_LIMIT_EXCEEDED: Self = Self(97);
    pub const REFUND_TO_WALLET: Self = Self(98);
    pub const EMAIL_SEND_FAILURE: Self = Self(99);
    pub const NOT_SETTLED: Self = Self(100);
    pub const NEED_CAPTCHA: Self = Self(101);
    pub const GSLT_DENIED: Self = Self(102);
    pub const GS_OWNER_DENIED: Self = Self(103);
    pub const INVALID_ITEM_TYPE: Self = Self(104);
    pub const IP_BANNED: Self = Self(105);
    pub const GSLT_EXPIRED: Self = Self(106);
    pub const INSUFFICIENT_FUNDS: Self = Self(107);
    pub const TOO_MANY_PENDING: Self = Self(108);
    pub const NO_SITE_LICENSES_FOUND: Self = Self(109);
    pub const WG_NETWORK_SEND_EXCEEDED: Self = Self(110);
    pub const ACCOUNT_NOT_FRIENDS: Self = Self(111);
    pub const LIMITED_USER_ACCOUNT: Self = Self(112);
    pub const CANT_REMOVE_ITEM: Self = Self(113);
    // renamed to AccountDeleted
    // pub const ACCOUNT_HAS_BEEN_DELETED: Self = Self(114);
    pub const ACCOUNT_DELETED: Self = Self(114);
    // renamed to ExistingUserCancelledLicense
    // pub const ACCOUNT_HAS_AN_EXISTING_USER_CANCELLED_LICENSE: Self = Self(115);
    pub const EXISTING_USER_CANCELLED_LICENSE: Self = Self(115);
    // renamed to CommunityCooldown
    // pub const DENIED_DUE_TO_COMMUNITY_COOLDOWN: Self = Self(116);
    pub const COMMUNITY_COOLDOWN: Self = Self(116);
    pub const NO_LAUNCHER_SPECIFIED: Self = Self(117);
    pub const MUST_AGREE_TO_SSA: Self = Self(118);
    pub const LAUNCHER_MIGRATED: Self = Self(119);
    // renamed to SteamRealmMismatch
    // pub const CURRENT_STEAM_REALM_DOES_NOT_MATCH: Self = Self(120);
    pub const STEAM_REALM_MISMATCH: Self = Self(120);
    pub const INVALID_SIGNATURE: Self = Self(121);
    pub const PARSE_FAILURE: Self = Self(122);
    pub const NO_VERIFIED_PHONE: Self = Self(123);
    pub const INSUFFICIENT_BATTERY: Self = Self(124);
    pub const CHARGER_REQUIRED: Self = Self(125);
    pub const CACHED_CREDENTIAL_INVALID: Self = Self(126);
    pub const PHONE_NUMBER_IS_VOIP: Self = Self(127);
}

impl PartialEq<i32> for EResult {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<EResult> for i32 {
    fn eq(&self, other: &EResult) -> bool {
        *self == other.0
    }
}

impl Default for EResult {
    fn default() -> Self {
        Self::INVALID
    }
}
