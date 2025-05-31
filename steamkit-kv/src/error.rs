pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to parse input")]
    Parse,
    #[error("Only {variant} can be converted to {output_type}")]
    InvalidConversion {
        variant: &'static str,
        output_type: &'static str,
    },
}
