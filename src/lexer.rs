use crate::error::Result;

#[derive(Debug, PartialEq)]
pub enum Token {
    Delimiter,
    Value(String),
    Newline,
}

pub struct Lexer<'a> {
    text: &'a str,
    pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(text: &'a str) -> Self {
        Self { text, pos: 0 }
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
        if current == b',' {
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

    fn scan_value(&self) -> usize {
        assert!(self.pos < self.text.len());
        let bytes = self.text.as_bytes();
        let mut ix = self.pos;
        let mut len = 0;
        while ix < self.text.len() && !matches!(bytes[ix], b',' | b'\n') {
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
        let mut lexer = Lexer::new(text);
        assert_eq!(lexer.peek()?, Some(Delimiter));
        assert_eq!(lexer.peek()?, Some(Delimiter));
        Ok(())
    }

    #[test]
    fn delim() {
        let text = ",";
        let token = Lexer::new(text).next().unwrap();
        assert_eq!(token, Some(Delimiter));
    }

    #[test]
    fn consecutive_delim() -> Result<()> {
        let text = ",,";
        let mut lexer = Lexer::new(text);
        assert_eq!(lexer.next()?, Some(Delimiter));
        assert_eq!(lexer.next()?, Some(Delimiter));
        Ok(())
    }

    #[test]
    fn newline() {
        let text = "\n";
        let token = Lexer::new(text).next().unwrap();
        assert_eq!(token, Some(Newline));
    }

    #[test]
    fn empty() {
        let text = "";
        let token = Lexer::new(text).next().unwrap();
        assert_eq!(token, None);
    }

    #[test]
    fn value() {
        let text = "foo";
        let token = Lexer::new(text).next().unwrap();
        assert_eq!(token, Some(Value("foo".into())));
    }

    #[test]
    fn value_with_spaces() {
        let text = "foo bar";
        let token = Lexer::new(text).next().unwrap();
        assert_eq!(token, Some(Value("foo bar".into())));
    }

    #[test]
    fn whitespace_values() -> Result<()> {
        let text = " , ";
        let mut lexer = Lexer::new(text);
        assert_eq!(lexer.next()?, Some(Value(" ".into())));
        assert_eq!(lexer.next()?, Some(Delimiter));
        assert_eq!(lexer.next()?, Some(Value(" ".into())));
        Ok(())
    }

    #[test]
    fn delim_values() -> Result<()> {
        let text = "foo,bar";
        let mut lexer = Lexer::new(text);
        assert_eq!(lexer.next()?, Some(Value("foo".into())));
        assert_eq!(lexer.next()?, Some(Delimiter));
        assert_eq!(lexer.next()?, Some(Value("bar".into())));
        Ok(())
    }
}
