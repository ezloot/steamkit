#[derive(Debug, Clone, new)]
pub struct MsgGCHdrProtoBuf {
    #[new(default = "todo!()")]
    pub msg: u32,
    pub header_length: i32,
    pub proto: TODO,
}

impl MsgGCHdrProtoBuf {
}

#[derive(Debug, Clone, new)]
pub struct MsgGCHdr {
    #[new(default = "todo!()")]
    pub header_version: u16,
    #[new(default = "todo!()")]
    pub target_job_id: u64,
    #[new(default = "todo!()")]
    pub source_job_id: u64,
}

impl MsgGCHdr {
}

