#[derive(Debug, Clone, Default)]
pub struct MsgClientJustStrings;

#[derive(Debug, Clone, new)]
pub struct MsgClientGenericResponse {
    pub result: TODO,
}

#[derive(Debug, Clone, new)]
pub struct MsgChannelEncryptRequest {
    #[new(default = "todo!()")]
    pub protocol_version: u32,
    #[new(default = "todo!()")]
    pub universe: TODO,
}

impl MsgChannelEncryptRequest {
    pub const PROTOCOL_VERSION: u32 = 1;
}

#[derive(Debug, Clone, new)]
pub struct MsgChannelEncryptResponse {
    #[new(default = "todo!()")]
    pub protocol_version: u32,
    #[new(default = "128")]
    pub key_size: u32,
}

#[derive(Debug, Clone, new)]
pub struct MsgChannelEncryptResult {
    #[new(default = "todo!()")]
    pub result: TODO,
}

#[deprecated]
#[derive(Debug, Clone, new)]
pub struct MsgClientNewLoginKey {
    pub unique_id: u32,
    pub login_key: [u8; 20],
}

#[deprecated]
#[derive(Debug, Clone, new)]
pub struct MsgClientNewLoginKeyAccepted {
    pub unique_id: u32,
}

#[derive(Debug, Clone, Default)]
pub struct MsgClientLogon;

impl MsgClientLogon {
    pub const OBFUSCATION_MASK: u32 = 0xBAADF00D;
    pub const CURRENT_PROTOCOL: u32 = 65580;
    pub const PROTOCOL_VER_MAJOR_MASK: u32 = 0xFFFF0000;
    pub const PROTOCOL_VER_MINOR_MASK: u32 = 0xFFFF;
    pub const PROTOCOL_VER_MINOR_MIN_GAME_SERVERS: u16 = 4;
    pub const PROTOCOL_VER_MINOR_MIN_FOR_SUPPORTING_E_MSG_MULTI: u16 = 12;
    pub const PROTOCOL_VER_MINOR_MIN_FOR_SUPPORTING_E_MSG_CLIENT_ENCRYPT_PCT: u16 = 14;
    pub const PROTOCOL_VER_MINOR_MIN_FOR_EXTENDED_MSG_HDR: u16 = 17;
    pub const PROTOCOL_VER_MINOR_MIN_FOR_CELL_ID: u16 = 18;
    pub const PROTOCOL_VER_MINOR_MIN_FOR_SESSION_ID_LAST: u16 = 19;
    pub const PROTOCOL_VER_MINOR_MIN_FOR_SERVER_AVAILABLITY_MSGS: u16 = 24;
    pub const PROTOCOL_VER_MINOR_MIN_CLIENTS: u16 = 25;
    pub const PROTOCOL_VER_MINOR_MIN_FOR_OS_TYPE: u16 = 26;
    pub const PROTOCOL_VER_MINOR_MIN_FOR_CEG_APPLY_PE_SIG: u16 = 27;
    pub const PROTOCOL_VER_MINOR_MIN_FOR_MARKETING_MESSAGES2: u16 = 27;
    pub const PROTOCOL_VER_MINOR_MIN_FOR_ANY_PROTO_BUF_MESSAGES: u16 = 28;
    pub const PROTOCOL_VER_MINOR_MIN_FOR_PROTO_BUF_LOGGED_OFF_MESSAGE: u16 = 28;
    pub const PROTOCOL_VER_MINOR_MIN_FOR_PROTO_BUF_MULTI_MESSAGES: u16 = 28;
    pub const PROTOCOL_VER_MINOR_MIN_FOR_SENDING_PROTOCOL_TO_UFS: u16 = 30;
    pub const PROTOCOL_VER_MINOR_MIN_FOR_MACHINE_AUTH: u16 = 33;
    pub const PROTOCOL_VER_MINOR_MIN_FOR_SESSION_ID_LAST_ANON: u16 = 36;
    pub const PROTOCOL_VER_MINOR_MIN_FOR_ENHANCED_APP_LIST: u16 = 40;
    pub const PROTOCOL_VER_MINOR_MIN_FOR_STEAM_GUARD_NOTIFICATION_UI: u16 = 41;
    pub const PROTOCOL_VER_MINOR_MIN_FOR_PROTO_BUF_SERVICE_MODULE_CALLS: u16 = 42;
    pub const PROTOCOL_VER_MINOR_MIN_FOR_GZIP_MULTI_MESSAGES: u16 = 43;
    pub const PROTOCOL_VER_MINOR_MIN_FOR_NEW_VOICE_CALL_AUTHORIZE: u16 = 44;
    pub const PROTOCOL_VER_MINOR_MIN_FOR_CLIENT_INSTANCE_I_DS: u16 = 44;
}

#[derive(Debug, Clone, new)]
pub struct MsgClientVACBanStatus {
    pub num_bans: u32,
}

#[derive(Debug, Clone, new)]
pub struct MsgClientAppUsageEvent {
    pub app_usage_event: TODO,
    pub game_id: u64,
    pub offline: u16,
}

#[deprecated]
#[derive(Debug, Clone, new)]
pub struct MsgClientEmailAddrInfo {
    pub password_strength: u32,
    pub flags_account_security_policy: u32,
    pub validated: u8,
}

#[derive(Debug, Clone, new)]
pub struct MsgClientUpdateGuestPassesList {
    pub result: TODO,
    pub count_guest_passes_to_give: i32,
    pub count_guest_passes_to_redeem: i32,
}

#[deprecated]
#[derive(Debug, Clone, new)]
pub struct MsgClientRequestedClientStats {
    pub count_stats: i32,
}

#[derive(Debug, Clone, new)]
pub struct MsgClientP2PIntroducerMessage {
    pub steam_id: u64,
    pub routing_type: TODO,
    pub data: [u8; 1450],
    pub data_len: u32,
}

#[derive(Debug, Clone, new)]
pub struct MsgClientOGSBeginSession {
    pub account_type: u8,
    pub account_id: u64,
    pub app_id: u32,
    pub time_started: u32,
}

#[derive(Debug, Clone, new)]
pub struct MsgClientOGSBeginSessionResponse {
    pub result: TODO,
    pub collecting_any: u8,
    pub collecting_details: u8,
    pub session_id: u64,
}

#[derive(Debug, Clone, new)]
pub struct MsgClientOGSEndSession {
    pub session_id: u64,
    pub time_ended: u32,
    pub reason_code: i32,
    pub count_attributes: i32,
}

#[derive(Debug, Clone, new)]
pub struct MsgClientOGSEndSessionResponse {
    pub result: TODO,
}

#[derive(Debug, Clone, new)]
pub struct MsgClientOGSWriteRow {
    pub session_id: u64,
    pub count_attributes: i32,
}

#[derive(Debug, Clone, new)]
pub struct MsgClientGetFriendsWhoPlayGame {
    pub game_id: u64,
}

#[derive(Debug, Clone, new)]
pub struct MsgClientGetFriendsWhoPlayGameResponse {
    pub result: TODO,
    pub game_id: u64,
    pub count_friends: u32,
}

#[derive(Debug, Clone, new)]
pub struct MsgGSPerformHardwareSurvey {
    pub flags: u32,
}

#[derive(Debug, Clone, new)]
pub struct MsgGSGetPlayStatsResponse {
    pub result: TODO,
    pub rank: i32,
    pub lifetime_connects: u32,
    pub lifetime_minutes_played: u32,
}

#[derive(Debug, Clone, new)]
pub struct MsgGSGetReputationResponse {
    pub result: TODO,
    pub reputation_score: u32,
    pub banned: u8,
    pub banned_ip: u32,
    pub banned_port: u16,
    pub banned_game_id: u64,
    pub time_ban_expires: u32,
}

#[derive(Debug, Clone, new)]
pub struct MsgGSDeny {
    pub steam_id: u64,
    pub deny_reason: TODO,
}

#[derive(Debug, Clone, new)]
pub struct MsgGSApprove {
    pub steam_id: u64,
}

#[derive(Debug, Clone, new)]
pub struct MsgGSKick {
    pub steam_id: u64,
    pub deny_reason: TODO,
    pub wait_til_map_change: i32,
}

#[derive(Debug, Clone, new)]
pub struct MsgGSGetUserGroupStatus {
    pub steam_id_user: u64,
    pub steam_id_group: u64,
}

#[derive(Debug, Clone, new)]
pub struct MsgGSGetUserGroupStatusResponse {
    pub steam_id_user: u64,
    pub steam_id_group: u64,
    pub clan_relationship: TODO,
    pub clan_rank: TODO,
}

#[derive(Debug, Clone, new)]
pub struct MsgClientJoinChat {
    pub steam_id_chat: u64,
    pub is_voice_speaker: u8,
}

#[derive(Debug, Clone, new)]
pub struct MsgClientChatEnter {
    pub steam_id_chat: u64,
    pub steam_id_friend: u64,
    pub chat_room_type: TODO,
    pub steam_id_owner: u64,
    pub steam_id_clan: u64,
    pub chat_flags: u8,
    pub enter_response: TODO,
    pub num_members: i32,
}

#[derive(Debug, Clone, new)]
pub struct MsgClientChatMsg {
    pub steam_id_chatter: u64,
    pub steam_id_chat_room: u64,
    pub chat_msg_type: TODO,
}

#[derive(Debug, Clone, new)]
pub struct MsgClientChatMemberInfo {
    pub steam_id_chat: u64,
    pub type_: TODO,
}

#[derive(Debug, Clone, new)]
pub struct MsgClientChatAction {
    pub steam_id_chat: u64,
    pub steam_id_user_to_act_on: u64,
    pub chat_action: TODO,
}

#[derive(Debug, Clone, new)]
pub struct MsgClientChatActionResult {
    pub steam_id_chat: u64,
    pub steam_id_user_acted_on: u64,
    pub chat_action: TODO,
    pub action_result: TODO,
}

#[derive(Debug, Clone, new)]
pub struct MsgClientChatRoomInfo {
    pub steam_id_chat: u64,
    pub type_: TODO,
}

#[derive(Debug, Clone, new)]
pub struct MsgClientSetIgnoreFriend {
    pub my_steam_id: u64,
    pub steam_id_friend: u64,
    pub ignore: u8,
}

#[derive(Debug, Clone, new)]
pub struct MsgClientSetIgnoreFriendResponse {
    pub friend_id: u64,
    pub result: TODO,
}

#[derive(Debug, Clone, new)]
pub struct MsgClientLoggedOff {
    pub result: TODO,
    pub sec_min_reconnect_hint: i32,
    pub sec_max_reconnect_hint: i32,
}

#[derive(Debug, Clone, new)]
pub struct MsgClientLogOnResponse {
    pub result: TODO,
    pub out_of_game_heartbeat_rate_sec: i32,
    pub in_game_heartbeat_rate_sec: i32,
    pub client_supplied_steam_id: u64,
    pub ip_public: u32,
    pub server_real_time: u32,
}

#[deprecated]
#[derive(Debug, Clone, new)]
pub struct MsgClientSendGuestPass {
    pub gift_id: u64,
    pub gift_type: u8,
    pub account_id: u32,
}

#[deprecated]
#[derive(Debug, Clone, new)]
pub struct MsgClientSendGuestPassResponse {
    pub result: TODO,
}

#[derive(Debug, Clone, new)]
pub struct MsgClientServerUnavailable {
    pub jobid_sent: u64,
    pub e_msg_sent: u32,
    pub e_server_type_unavailable: TODO,
}

#[derive(Debug, Clone, new)]
pub struct MsgClientCreateChat {
    pub chat_room_type: TODO,
    pub game_id: u64,
    pub steam_id_clan: u64,
    pub permission_officer: TODO,
    pub permission_member: TODO,
    pub permission_all: TODO,
    pub members_max: u32,
    pub chat_flags: u8,
    pub steam_id_friend_chat: u64,
    pub steam_id_invited: u64,
}

#[derive(Debug, Clone, new)]
pub struct MsgClientCreateChatResponse {
    pub result: TODO,
    pub steam_id_chat: u64,
    pub chat_room_type: TODO,
    pub steam_id_friend_chat: u64,
}

#[derive(Debug, Clone, new)]
pub struct MsgClientMarketingMessageUpdate2 {
    pub marketing_message_update_time: u32,
    pub count: u32,
}

#[derive(Debug, Clone, new)]
pub struct MsgClientGetLegacyGameKey {
    pub app_id: u32,
}

#[derive(Debug, Clone, new)]
pub struct MsgClientGetLegacyGameKeyResponse {
    pub app_id: u32,
    pub result: TODO,
    pub length: u32,
}

