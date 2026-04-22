use crate::errors::*;

/// Long option prefix.
const LONG_PREFIX: &str = "--";

/// Short option prefix.
const SHORT_PREFIX: &str = "-";

/// Option value separator.
const VALUE_SEPARATOR: &str = "=";

/// Parsed token.
#[derive(Debug, PartialEq)]
pub enum Token {
  ShortOption(char),
  LongOption(String),
  Value(String),
  OptionTerminator,
}

/// Parses arguments into lexical tokens.
pub fn parse<I, S>(input: I) -> Result<Vec<Token>>
where
  I: IntoIterator<Item = S>,
  S: AsRef<str>,
{
  let mut tokens = vec![];
  for item in input {
    let mut s = item.as_ref();
    if s.starts_with(LONG_PREFIX) {
      // Remove the prefix of the long option.
      s = s.strip_prefix(LONG_PREFIX).unwrap();
      if s.is_empty() {
        // When the string is empty then return option terminator.
        tokens.push(Token::OptionTerminator);
      } else {
        // Split long option around the equal sign to extract possible value definition.
        let parts = s.split(VALUE_SEPARATOR).map(|part| part.to_string()).collect::<Vec<String>>();
        // Parts may not start or end with whitespace.
        for part in &parts {
          if part.starts_with(" ") {
            return Err(ClarsError::new("whitespace before"));
          }
          if part.ends_with(" ") {
            return Err(ClarsError::new("whitespace after"));
          }
        }
        // The valid number of parts is 1 or 2.
        match parts.len() {
          1 => {
            tokens.push(Token::LongOption(parts[0].to_owned()));
          }
          2 => {
            tokens.push(Token::LongOption(parts[0].to_owned()));
            tokens.push(Token::Value(parts[1].to_owned()));
          }
          _ => return Err(ClarsError::new("too many equal signs")),
        }
      }
    } else if s.starts_with(SHORT_PREFIX) {
      // Remove the prefix of the short option.
      s = s.strip_prefix(SHORT_PREFIX).unwrap();
      if s.is_empty() {
        tokens.push(Token::Value(SHORT_PREFIX.to_string()));
      } else {
        // Split long option around the equal sign to extract possible value definition.
        let parts = s.split(VALUE_SEPARATOR).map(|part| part.to_string()).collect::<Vec<String>>();
        // Parts may not start or end with whitespace.
        for part in &parts {
          if part.starts_with(" ") {
            return Err(ClarsError::new("whitespace before"));
          }
          if part.ends_with(" ") {
            return Err(ClarsError::new("whitespace after"));
          }
        }
        // The valid number of parts is 1 or 2.
        match parts.len() {
          1 => {
            for ch in parts[0].chars() {
              tokens.push(Token::ShortOption(ch));
            }
          }
          2 => {
            for ch in parts[0].chars() {
              tokens.push(Token::ShortOption(ch));
            }
            tokens.push(Token::Value(parts[1].to_owned()));
          }
          _ => return Err(ClarsError::new("too many equal signs")),
        }
      }
    } else {
      tokens.push(Token::Value(s.to_string()));
    }
  }
  Ok(tokens)
}
