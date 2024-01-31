include!(concat!(env!("OUT_DIR"), "/generated/mod.rs"));

pub trait HasEMsg {
    fn emsg() -> emsg::EMsg;
}

pub trait EnumValues<T> {
    fn values() -> &'static [T];
}
