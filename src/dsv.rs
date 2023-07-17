use std::ops::{Index, IndexMut};

pub use crate::parser::Options;
use crate::{error::Result, parser::Parser};

/// A table of delimiter-separated values.
///
/// A `Dsv` is made up of records, and a record is made up of fields. Fields are
/// stored as text and must be parsed into native data types separately.
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

    /// Parse a Dsv from an input string with options.
    pub fn from_str_with_options(text: &str, options: Options) -> Result<Dsv> {
        Parser::from_str_with_options(text, options)
    }
}

impl Index<usize> for Dsv {
    type Output = Vec<String>;

    /// Returns a reference to the record at the specified index.
    ///
    /// Panics if there is no record at the specified index.
    fn index(&self, index: usize) -> &Self::Output {
        &self.records[index]
    }
}

impl IndexMut<usize> for Dsv {
    /// Returns a mutable reference to the record at the specified index.
    ///
    /// Panics if there is no record at the specified index.
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.records[index]
    }
}
