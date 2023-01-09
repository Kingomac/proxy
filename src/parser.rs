use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct ParseError;

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parseerror ocurred")
    }
}

pub trait Parser<T> {
    fn from_bytes(bytes: &Vec<u8>) -> Result<T, ParseError>;
    fn from_str(text: &str) -> Result<T, ParseError>;
    fn from_lines(lines: &Vec<String>) -> Result<T, ParseError>;
}

pub mod http_request_parser;
