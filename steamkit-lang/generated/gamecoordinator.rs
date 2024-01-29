#[derive(Debug, Clone, new)]
pub struct MsgGCHdrProtoBuf {
    #[new(default = "0")]
    pub msg: u32,
    pub header_length: i32,
    pub proto: TODO,
}

#[derive(Debug, Clone, new)]
pub struct MsgGCHdr {
    #[new(default = "1")]
    pub header_version: u16,
    #[new(default = "todo!()")]
    pub target_job_id: u64,
    #[new(default = "todo!()")]
    pub source_job_id: u64,
}

