use crate::error::{Error, Result};
use crate::lexer::Token;
use crate::{lexer::Lexer, Dsv};

/// Treatment of double quotes.
pub enum Quotes {
    /// Quotes have no special meaning and are considered part of the value.
    Insignificant,
    /// Quotes can be used to escape delimiters and newlines and are not
    /// considered part of the value.
    Significant,
}

/// Options for parsing a DSV file.
pub struct Options {
    /// The delimiter that separates fields.
    ///
    /// Default: `,`.
    delimiter: u8,
    /// Treatment of double quotes.
    ///
    /// Default: `Quotes::Significant`.
    quotes: Quotes,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            delimiter: b',',
            quotes: Quotes::Significant,
        }
    }
}

pub struct Parser<'a> {
    lexer: Lexer<'a>,
}

impl<'a> Parser<'a> {
    pub fn from_str(text: &'a str) -> Result<Dsv> {
        Parser::from_str_with_options(text, Options::default())
    }

    pub fn from_str_with_options(text: &'a str, options: Options) -> Result<Dsv> {
        let lexer = Lexer::new(text, options.delimiter, options.quotes);
        let mut parser = Self { lexer };
        parser.dsv()
    }

    fn dsv(&mut self) -> Result<Dsv> {
        let mut dsv = Dsv::new();
        let mut num_expected_fields = None;

        while let Some(_token) = self.lexer.peek()? {
            let record = self.record()?;
            match num_expected_fields {
                None => num_expected_fields = Some(record.len()),
                Some(len) if record.len() != len => return Err(Error::Parse),
                _ => {}
            }
            dsv.records.push(record);
        }

        Ok(dsv)
    }

    fn record(&mut self) -> Result<Vec<String>> {
        let mut record = vec![];
        while let Some(token) = self.lexer.next()? {
            match (token, self.lexer.peek()?) {
                (Token::Newline, _) => break,
                // disallow consecutive values
                (Token::Value(_), Some(Token::Value(_))) => return Err(Error::Parse),
                (Token::Value(value), _) => record.push(value),
                // infer empty value between consecutive delimiters
                (Token::Delimiter, Some(Token::Delimiter)) => record.push(String::new()),
                // infer empty value at newline or end of file
                (Token::Delimiter, Some(Token::Newline)) | (Token::Delimiter, None) => {
                    record.push(String::new())
                }
                // infer empty value at beginning of record
                (Token::Delimiter, _) if record.is_empty() => record.push(String::new()),
                _ => {}
            }
        }
        Ok(record)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_value() {
        let text = "foo";
        let dsv = Parser::from_str(text).unwrap();
        let mut expected = Dsv::new();
        expected.records.push(vec!["foo".into()]);
        assert_eq!(dsv, expected);
    }

    #[test]
    fn two_values() {
        let text = "foo,bar";
        let dsv = Parser::from_str(text).unwrap();
        let mut expected = Dsv::new();
        expected.records.push(vec!["foo".into(), "bar".into()]);
        assert_eq!(dsv, expected);
    }

    #[test]
    fn multiline() {
        let text = "foo,bar\nbaz,bux";
        let dsv = Parser::from_str(text).unwrap();
        let mut expected = Dsv::new();
        expected.records.push(vec!["foo".into(), "bar".into()]);
        expected.records.push(vec!["baz".into(), "bux".into()]);
        assert_eq!(dsv, expected);
    }

    #[test]
    fn empty_inner_value() {
        let text = "foo,,bar";
        let dsv = Parser::from_str(text).unwrap();
        let mut expected = Dsv::new();
        expected
            .records
            .push(vec!["foo".into(), "".into(), "bar".into()]);
        assert_eq!(dsv, expected);
    }

    #[test]
    fn empty_trailing_value() {
        let text = "bar,";
        let dsv = Parser::from_str(text).unwrap();
        let mut expected = Dsv::new();
        expected.records.push(vec!["bar".into(), "".into()]);
        assert_eq!(dsv, expected);
    }

    #[test]
    fn empty_leading_value() {
        let text = ",foo";
        let dsv = Parser::from_str(text).unwrap();
        let mut expected = Dsv::new();
        expected.records.push(vec!["".into(), "foo".into()]);
        assert_eq!(dsv, expected);
    }

    #[test]
    fn mismatched_field_count() {
        let text = "foo,bar\nbaz";
        let dsv = Parser::from_str(text);
        assert!(dsv.is_err());
    }

    #[test]
    fn tab_delim() {
        let text = "foo\tbar";
        let dsv = Parser::from_str_with_options(
            text,
            Options {
                delimiter: b'\t',
                ..Default::default()
            },
        )
        .unwrap();
        let mut expected = Dsv::new();
        expected.records.push(vec!["foo".into(), "bar".into()]);
        assert_eq!(dsv, expected);
    }
}
