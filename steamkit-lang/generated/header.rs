#[derive(Debug, Clone, new)]
pub struct MsgHdr {
    #[new(default = "todo!()")]
    pub msg: TODO,
    #[new(default = "todo!()")]
    pub target_job_id: u64,
    #[new(default = "todo!()")]
    pub source_job_id: u64,
}

impl MsgHdr {
}

#[derive(Debug, Clone, new)]
pub struct ExtendedClientMsgHdr {
    #[new(default = "todo!()")]
    pub msg: TODO,
    #[new(default = "todo!()")]
    pub header_size: u8,
    #[new(default = "todo!()")]
    pub header_version: u16,
    #[new(default = "todo!()")]
    pub target_job_id: u64,
    #[new(default = "todo!()")]
    pub source_job_id: u64,
    #[new(default = "todo!()")]
    pub header_canary: u8,
    pub steam_id: u64,
    pub session_id: i32,
}

impl ExtendedClientMsgHdr {
}

#[derive(Debug, Clone, new)]
pub struct MsgHdrProtoBuf {
    #[new(default = "todo!()")]
    pub msg: TODO,
    pub header_length: i32,
    pub proto: TODO,
}

impl MsgHdrProtoBuf {
}

