/// Error type for the dsv crate.
#[derive(Debug)]
pub enum Error {
    Parse,
}

/// Result type for the dsv crate.
pub type Result<T> = std::result::Result<T, Error>;
