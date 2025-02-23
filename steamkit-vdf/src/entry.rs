#[derive(Debug, Clone)]
pub enum Entry {
    Integer(i32),
    Float(f32),
    Boolean(bool),
    String(String),
    Group(Vec<Entry>),
}
