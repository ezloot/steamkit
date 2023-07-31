use steamkit_lang::enums::{EUniverse, EAccountType};

#[derive(Default, Debug, Eq, PartialEq, Clone, Copy)]
#[repr(u32)]
pub enum Instance {
    #[default]
    All = 0,
    Desktop = 1,
    Console = 2,
    Web = 3,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Id {
    pub universe: EUniverse,
    pub type_: EAccountType,
    pub instance: Instance,
    pub account: u64,
}
