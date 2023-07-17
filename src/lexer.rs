use crate::{error::Result, parser::Quotes};

#[derive(Debug, PartialEq)]
pub enum Token {
    Delimiter,
    Value(String),
    Newline,
}

pub struct Lexer<'a> {
    text: &'a str,
    pos: usize,
    delimiter: u8,
    quotes: Quotes,
}

impl<'a> Lexer<'a> {
    pub fn new(text: &'a str, delimiter: u8, quotes: Quotes) -> Self {
        Self {
            text,
            pos: 0,
            delimiter,
            quotes,
        }
    }

    pub fn next(&mut self) -> Result<Option<Token>> {
        use Token::*;

        if self.pos == self.text.len() {
            return Ok(None);
        }

        if let Some(_) = self.scan_delim() {
            self.pos += 1;
            return Ok(Some(Delimiter));
        }

        if let Some(_) = self.scan_newline() {
            self.pos += 1;
            return Ok(Some(Newline));
        }

        if matches!(self.quotes, Quotes::Significant) {
            if let Some(len) = self.scan_quoted_value() {
                let value = &self.text[self.pos + 1..self.pos + 1 + len];
                let value = value.replace(r#""""#, r#"""#);
                self.pos += len + 2;
                return Ok(Some(Value(value.into())));
            }
        }

        let len = self.scan_value();
        let value = &self.text[self.pos..self.pos + len];
        self.pos += len;
        Ok(Some(Value(value.into())))
    }

    pub fn peek(&mut self) -> Result<Option<Token>> {
        let start = self.pos;
        let token = self.next();
        self.pos = start;
        token
    }

    fn scan_delim(&self) -> Option<()> {
        assert!(self.pos < self.text.len());
        let current = self.text.as_bytes()[self.pos];
        if current == self.delimiter {
            Some(())
        } else {
            None
        }
    }

    fn scan_newline(&self) -> Option<()> {
        assert!(self.pos < self.text.len());
        let current = self.text.as_bytes()[self.pos];
        if current == b'\n' {
            Some(())
        } else {
            None
        }
    }

    fn scan_quoted_value(&self) -> Option<usize> {
        assert!(self.pos < self.text.len());
        let bytes = self.text.as_bytes();
        let current = bytes[self.pos];
        if current != b'"' {
            return None;
        }
        let mut ix = self.pos + 1;
        let mut len = 0;
        while ix < bytes.len() {
            if bytes[ix] == b'"' {
                if ix + 1 < bytes.len() && bytes[ix + 1] == b'"' {
                    ix += 2;
                    len += 2;
                    continue;
                } else {
                    break;
                }
            }
            ix += 1;
            len += 1;
        }
        Some(len)
    }

    fn scan_value(&self) -> usize {
        assert!(self.pos < self.text.len());
        let bytes = self.text.as_bytes();
        let mut ix = self.pos;
        let mut len = 0;
        while ix < self.text.len() && bytes[ix] != self.delimiter && bytes[ix] != b'\n' {
            ix += 1;
            len += 1;
        }
        len
    }
}

#[cfg(test)]
mod tests {
    use super::{Token::*, *};

    #[test]
    fn peek() -> Result<()> {
        let text = ",";
        let mut lexer = Lexer::new(text, b',', Quotes::Significant);
        assert_eq!(lexer.peek()?, Some(Delimiter));
        assert_eq!(lexer.peek()?, Some(Delimiter));
        Ok(())
    }

    #[test]
    fn delim() {
        let text = ",";
        let token = Lexer::new(text, b',', Quotes::Significant).next().unwrap();
        assert_eq!(token, Some(Delimiter));
    }

    #[test]
    fn consecutive_delim() -> Result<()> {
        let text = ",,";
        let mut lexer = Lexer::new(text, b',', Quotes::Significant);
        assert_eq!(lexer.next()?, Some(Delimiter));
        assert_eq!(lexer.next()?, Some(Delimiter));
        Ok(())
    }

    #[test]
    fn newline() {
        let text = "\n";
        let token = Lexer::new(text, b',', Quotes::Significant).next().unwrap();
        assert_eq!(token, Some(Newline));
    }

    #[test]
    fn empty() {
        let text = "";
        let token = Lexer::new(text, b',', Quotes::Significant).next().unwrap();
        assert_eq!(token, None);
    }

    #[test]
    fn value() {
        let text = "foo";
        let token = Lexer::new(text, b',', Quotes::Significant).next().unwrap();
        assert_eq!(token, Some(Value("foo".into())));
    }

    #[test]
    fn value_with_spaces() {
        let text = "foo bar";
        let token = Lexer::new(text, b',', Quotes::Significant).next().unwrap();
        assert_eq!(token, Some(Value("foo bar".into())));
    }

    #[test]
    fn whitespace_values() -> Result<()> {
        let text = " , ";
        let mut lexer = Lexer::new(text, b',', Quotes::Significant);
        assert_eq!(lexer.next()?, Some(Value(" ".into())));
        assert_eq!(lexer.next()?, Some(Delimiter));
        assert_eq!(lexer.next()?, Some(Value(" ".into())));
        Ok(())
    }

    #[test]
    fn delim_values() -> Result<()> {
        let text = "foo,bar";
        let mut lexer = Lexer::new(text, b',', Quotes::Significant);
        assert_eq!(lexer.next()?, Some(Value("foo".into())));
        assert_eq!(lexer.next()?, Some(Delimiter));
        assert_eq!(lexer.next()?, Some(Value("bar".into())));
        Ok(())
    }

    #[test]
    fn quoted_values() -> Result<()> {
        let text = r#""foo","bar""#;
        let mut lexer = Lexer::new(text, b',', Quotes::Significant);
        assert_eq!(lexer.next()?, Some(Value("foo".into())));
        assert_eq!(lexer.next()?, Some(Delimiter));
        assert_eq!(lexer.next()?, Some(Value("bar".into())));
        Ok(())
    }

    #[test]
    fn quoted_value_with_newline() {
        let text = "\"foo\nbar\"";
        let token = Lexer::new(text, b',', Quotes::Significant).next().unwrap();
        assert_eq!(token, Some(Value("foo\nbar".into())));
    }

    #[test]
    fn quoted_value_with_delim() {
        let text = r#""foo,bar""#;
        let token = Lexer::new(text, b',', Quotes::Significant).next().unwrap();
        assert_eq!(token, Some(Value("foo,bar".into())));
    }

    #[test]
    fn quoted_value_insignificant() -> Result<()> {
        let text = r#""foo,bar""#;
        let mut lexer = Lexer::new(text, b',', Quotes::Insignificant);
        assert_eq!(lexer.next()?, Some(Value("\"foo".into())));
        assert_eq!(lexer.next()?, Some(Delimiter));
        assert_eq!(lexer.next()?, Some(Value("bar\"".into())));
        Ok(())
    }

    #[test]
    fn escaped_quote() {
        let text = r#""foo""""#;
        let token = Lexer::new(text, b',', Quotes::Significant).next().unwrap();
        assert_eq!(token, Some(Value("foo\"".into())));
    }

    #[test]
    fn consecutive_escaped_quotes() {
        let text = r#""foo""""""""#;
        let token = Lexer::new(text, b',', Quotes::Significant).next().unwrap();
        assert_eq!(token, Some(Value("foo\"\"\"".into())));
    }
}
