use anyhow::bail;

use crate::result::ParserResult;

#[derive(Debug, PartialEq)]
enum NumericLiteral {
    Integer(String),
    Float(String),
    Hexadecimal(String),
}

pub struct QueryParser<'a> {
    input: &'a str,
    char_indices: std::str::CharIndices<'a>,
    current_index: usize,
    parsed: Option<&'a str>,
}

impl<'a> QueryParser<'a> {
    pub fn new(input: &'a str) -> Self {
        QueryParser {
            input,
            char_indices: input.char_indices(),
            current_index: 0,
            parsed: None,
        }
    }

    pub fn run(&mut self) -> ParserResult<&'a str> {
        while let Some(c) = self.advance() {
            dbg!(c, self.current_slice());
        }
        bail!("Not implemented");
    }

    fn advance(&mut self) -> Option<char> {
        if let Some((byte_index, ch)) = self.char_indices.next() {
            self.current_index = byte_index;
            self.parsed = Some(&self.input[..self.current_index]);
            Some(ch)
        } else {
            None
        }
    }

    fn current_slice(&self) -> &'a str {
        &self.input[self.current_index..]
    }
}
