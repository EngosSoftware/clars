use crate::errors::*;
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
  /// Parses arguments into lexical tokens.
  pub fn parse<I, S>(mut self, items: I) -> ClarResult<VecDeque<Token>>
  where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
  {
    for item in items {
      let input = item.as_ref();
      if let Some(Token::OptionTerminator(values)) = self.tokens.back_mut() {
        values.push(input.to_string());
      } else if let Some(s) = input.strip_prefix(LONG_PREFIX) {
        self.parse_long_option(s)?;
      } else if let Some(s) = input.strip_prefix(SHORT_PREFIX) {
        self.parse_short_option(s)?
      } else {
        self.tokens.push_back(Token::Argument(input.to_string()));
      }
    }
    Ok(self.tokens)
  }

  fn parse_long_option(&mut self, input: &str) -> ClarResult<()> {
    if input.is_empty() {
      // When the input is empty then encountered option terminator.
      self.tokens.push_back(Token::OptionTerminator(vec![]));
    } else {
      // Parse option name with optional associated value.
      let (option, value) = self.parse_value(input);
      self.validate_long_label(&option)?;
      self.tokens.push_back(Token::LongOption(option, value));
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
        let ch = chars.next().unwrap();
        self.validate_short_label(ch)?;
        if chars.peek().is_none() {
          self.tokens.push_back(Token::ShortOption(ch, value));
          break;
        } else {
          self.tokens.push_back(Token::ShortOption(ch, None));
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

  fn validate_long_label(&self, label: &str) -> ClarResult<()> {
    for (index, ch) in label.chars().enumerate() {
      if index == 0 {
        if !ch.is_ascii_alphabetic() {
          return Err(err_long_option_must_start_with_letter(label.to_string()));
        }
      } else {
        if !(ch.is_ascii_alphanumeric() || ch == '-') {
          return Err(err_long_option_must_contain_letters_digits_hyphens(label.to_string()));
        }
      }
    }
    Ok(())
  }

  fn validate_short_label(&self, ch: char) -> ClarResult<()> {
    if !ch.is_ascii_alphanumeric() {
      return Err(err_short_option_must_be_letter_or_digit(ch));
    }
    Ok(())
  }
}
