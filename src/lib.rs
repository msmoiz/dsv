//! This crate provides methods for working with delimiter-separated values
//! (DSV) files, including CSVs, TSVs, and other tabular files delimited by a
//! single character.
//!
//! # Usage
//!
//! The basic supported use case is to parse an input string into a list of
//! records and iterate over the records.
//!
//! ```
//! use dsv::Dsv;
//!
//! let groceries = ["apples,2", "oranges,3", "bananas,4"].join("\n");
//! let dsv = Dsv::from_str(&groceries).unwrap();
//! for record in dsv.records() {
//!     println!("Fruit {}, count {}", record[0], record[1]);
//! }
//! ```
//!
//! # Examples
//!
//! To parse a DSV from an input string, use `Dsv::from_str`.
//!
//! ```
//! use dsv::Dsv;
//!
//! let text = ["hello,sun", "hello,moon"].join("\n");
//! let dsv = Dsv::from_str(&text).unwrap();
//!
//! assert_eq!(dsv[0][0], "hello");
//! assert_eq!(dsv[0][1], "sun");
//! assert_eq!(dsv[1][0], "hello");
//! assert_eq!(dsv[1][1], "moon");
//! ```
//!
//! To configure the parser, use `Dsv::from_str_with_options` instead.
//!
//! ```
//! use dsv::Dsv;
//! use dsv::Options;
//!
//! let text = ["hello;sun", "hello;moon"].join("\n");
//! let options = Options { delimiter: b';', ..Default::default() };
//! let dsv = Dsv::from_str_with_options(&text, options).unwrap();
//!
//! assert_eq!(dsv[0][0], "hello");
//! assert_eq!(dsv[0][1], "sun");
//! assert_eq!(dsv[1][0], "hello");
//! assert_eq!(dsv[1][1], "moon");
//! ```
//!
//! # Options
//!
//! The following parser options are available.
//!
//! * `delimiter`: The delimiter that separates fields. Default: `,`. Can be
//!   used to delimit using tabs, semicolons, or another character.
//! * `quotes`: Treatment of double quotes. Default: `Quotes::Significant`. If
//!   set to `Quotes::Insignificant`, double quotes are treated like any other
//!   character, and characters that would otherwise be valid within quotes
//!   (such as the delimiter) can not be used in values.
//!
//! # Additional considerations
//!
//! Each record in a DSV is expected to contain the same number of fields. The
//! expected length is based on the length of the first record in the table.

mod dsv;
mod error;
mod lexer;
mod parser;

pub use crate::dsv::Dsv;
pub use crate::parser::Options;
