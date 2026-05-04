use crate::errors::*;

/// Long option prefix.
const LONG_PREFIX: &str = "--";

/// Short option prefix.
const SHORT_PREFIX: &str = "-";

/// Option value separator.
const VALUE_SEPARATOR: &str = "=";

/// Parsed token.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
  /// Short option with optional value.
  ShortOption(char, Option<String>),
  /// Long option with optional value.
  LongOption(String, Option<String>),
  /// Argument.
  Argument(String),
  /// Option terminator.
  OptionTerminator,
}

pub struct Lexer {
  tokens: Vec<Token>,
  consumed_option_terminator: bool,
}

impl Default for Lexer {
  fn default() -> Self {
    Self::new()
  }
}

impl Lexer {
  pub fn new() -> Self {
    Self {
      tokens: vec![],
      consumed_option_terminator: false,
    }
  }

  /// Parses arguments into lexical tokens.
  pub fn parse<I, S>(&mut self, input: I) -> Result<&[Token]>
  where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
  {
    for item in input {
      let text = item.as_ref();
      if self.consumed_option_terminator {
        self.tokens.push(Token::Argument(text.to_string()));
      } else if let Some(s) = text.strip_prefix(LONG_PREFIX) {
        self.parse_long_option(s)?;
      } else if let Some(s) = text.strip_prefix(SHORT_PREFIX) {
        self.parse_short_option(s)?
      } else {
        self.tokens.push(Token::Argument(text.to_string()));
      }
    }
    Ok(&self.tokens)
  }

  fn parse_long_option(&mut self, s: &str) -> Result<()> {
    if s.is_empty() {
      // When the string is empty then return option terminator.
      self.tokens.push(Token::OptionTerminator);
      self.consumed_option_terminator = true;
    } else {
      // Parse option name with possible value.
      let (option, value) = self.parse_parts(s)?;
      self.validate_long_name(&option)?;
      self.tokens.push(Token::LongOption(option, value));
    }
    Ok(())
  }

  fn parse_short_option(&mut self, s: &str) -> Result<()> {
    if s.is_empty() {
      self.tokens.push(Token::Argument(SHORT_PREFIX.to_string()));
    } else {
      // Parse option name with possible value.
      let (option, value) = self.parse_parts(s)?;
      let mut chars = option.chars().peekable();
      loop {
        let ch = chars.next().unwrap();
        self.validate_short_name(ch)?;
        if chars.peek().is_none() {
          self.tokens.push(Token::ShortOption(ch, value));
          break;
        } else {
          self.tokens.push(Token::ShortOption(ch, None));
        }
      }
    }
    Ok(())
  }

  fn parse_parts(&self, s: &str) -> Result<(String, Option<String>)> {
    // Split around the value separator to extract possible value for option.
    let parts = s
      .split(VALUE_SEPARATOR)
      .map(|part| part.to_string())
      .collect::<Vec<String>>();
    // Parts may not start or end with whitespaces.
    for part in &parts {
      if part.starts_with(" ") {
        return Err(ClarsError::new("whitespace before"));
      }
      if part.ends_with(" ") {
        return Err(ClarsError::new("whitespace after"));
      }
    }
    match parts.len() {
      1 => Ok((parts[0].clone(), None)),
      2 => Ok((parts[0].clone(), Some(parts[1].clone()))),
      _ => Err(ClarsError::new("too many equal signs")),
    }
  }

  fn validate_long_name(&self, s: &str) -> Result<()> {
    for (index, ch) in s.chars().enumerate() {
      if index == 0 {
        if !ch.is_ascii_alphabetic() {
          return Err(ClarsError::new("long option name must start with a letter"));
        }
      } else {
        if !(ch.is_ascii_alphanumeric() || ch == '-') {
          return Err(ClarsError::new(
            "long option name must contain letters, digits or hyphens",
          ));
        }
      }
    }
    Ok(())
  }

  fn validate_short_name(&self, ch: char) -> Result<()> {
    if !ch.is_ascii_alphanumeric() {
      return Err(ClarsError::new("short option must be a letter or digit"));
    }
    Ok(())
  }
}
