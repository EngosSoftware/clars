use crate::errors::ClarResult;
use crate::model::ClarOption;
use std::collections::VecDeque;

/// Long option prefix.
const LONG_PREFIX: &str = "--";

/// Short option prefix.
const SHORT_PREFIX: &str = "-";

/// Separator between option label and its associated value.
const VALUE_SEPARATOR: char = '=';

/// Parsed token.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
  /// Short option with optional value.
  ShortOption(char, Option<String>),
  /// Long option with optional value.
  LongOption(String, Option<String>),
  /// Argument.
  Argument(String),
  /// Option terminator with the following arguments.
  OptionTerminator(Vec<String>),
}

#[derive(Debug, Clone, Default)]
pub struct Lexer {
  tokens: VecDeque<Token>,
}

impl Lexer {
  /// Parses input arguments into a sequence of lexical tokens.
  pub fn parse<I, S>(mut self, items: I) -> ClarResult<VecDeque<Token>>
  where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
  {
    for item in items {
      self.parse_item(item.as_ref())?;
    }
    Ok(self.tokens)
  }

  /// Parses a single argument, classifying it as a long option, short option,
  /// or positional argument. If an option terminator (`--`) was already seen,
  /// the input is appended to it as a trailing value instead.
  fn parse_item(&mut self, input: &str) -> ClarResult<()> {
    if let Some(Token::OptionTerminator(values)) = self.tokens.back_mut() {
      values.push(input.to_string());
    } else if let Some(s) = input.strip_prefix(LONG_PREFIX) {
      self.parse_long_option(s)?;
    } else if let Some(s) = input.strip_prefix(SHORT_PREFIX) {
      self.parse_short_option(s)?;
    } else {
      self.tokens.push_back(Token::Argument(input.to_string()));
    }
    Ok(())
  }

  fn parse_long_option(&mut self, input: &str) -> ClarResult<()> {
    if input.is_empty() {
      // When the input is empty then encountered option terminator.
      self.tokens.push_back(Token::OptionTerminator(vec![]));
    } else {
      // Parse option name with optional associated value.
      let (label, value) = self.parse_value(input);
      ClarOption::validate_long_label(&label)?;
      self.tokens.push_back(Token::LongOption(label, value));
    }
    Ok(())
  }

  fn parse_short_option(&mut self, input: &str) -> ClarResult<()> {
    if input.is_empty() {
      self.tokens.push_back(Token::Argument(SHORT_PREFIX.to_string()));
    } else {
      // Parse option name with optional associated value.
      let (option, value) = self.parse_value(input);
      let mut chars = option.chars().peekable();
      loop {
        let label = chars.next().unwrap();
        ClarOption::validate_short_label(label)?;
        if chars.peek().is_none() {
          self.tokens.push_back(Token::ShortOption(label, value));
          break;
        } else {
          self.tokens.push_back(Token::ShortOption(label, None));
        }
      }
    }
    Ok(())
  }

  fn parse_value(&self, input: &str) -> (String, Option<String>) {
    match input.split_once(VALUE_SEPARATOR) {
      Some((before, after)) => (before.to_string(), Some(after.to_string())),
      None => (input.to_string(), None),
    }
  }
}
