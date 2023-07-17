use crate::{error::Result, parser::Parser};

#[derive(Default, PartialEq, Debug)]
pub struct Dsv {
    pub(crate) records: Vec<Vec<String>>,
}

impl Dsv {
    /// Create a new Dsv.
    pub fn new() -> Dsv {
        Dsv {
            ..Default::default()
        }
    }

    /// Parse a Dsv from an input string.
    pub fn from_str(text: &str) -> Result<Dsv> {
        Parser::from_str(text)
    }
}
