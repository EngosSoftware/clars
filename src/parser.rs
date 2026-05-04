use crate::errors::{ClarsError, Result, err, err_missing_required_argument};
use crate::lexer::Token;
use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
  /// Short option with optional value.
  ShortOption(
    /// Option name (not the short name).
    String,
    /// Optional value assigned to short option.
    Option<String>,
  ),
  /// Long option with optional value.
  LongOption(
    /// Option name (not the long name).
    String,
    /// Optional value assigned to long option.
    Option<String>,
  ),
  /// Subcommand.
  Subcommand(
    /// Subcommand name.
    String,
    /// Value associated with the subcommand.
    Option<String>,
  ),
  /// Argument.
  Argument(
    /// Argument name.
    String,
    /// Argument value.
    String,
  ),
}

impl Value {
  pub fn short(name: impl AsRef<str>, value: Option<String>) -> Self {
    Value::ShortOption(name.as_ref().to_string(), value)
  }

  pub fn long(name: impl AsRef<str>, value: Option<String>) -> Self {
    Value::LongOption(name.as_ref().to_string(), value)
  }

  pub fn subcommand(name: impl AsRef<str>, value: Option<String>) -> Self {
    Value::Subcommand(name.as_ref().to_string(), value)
  }

  pub fn argument(name: impl AsRef<str>, value: impl AsRef<str>) -> Self {
    Value::Argument(name.as_ref().to_string(), value.as_ref().to_string())
  }

  /// Returns the name.
  pub fn name(&self) -> &str {
    match self {
      Value::ShortOption(name, _) => name,
      Value::LongOption(name, _) => name,
      Value::Subcommand(name, _) => name,
      Value::Argument(name, _) => name,
    }
  }

  /// Returns the value.
  pub fn value(&self) -> Option<String> {
    match self {
      Value::ShortOption(_, value) => value.clone(),
      Value::LongOption(_, value) => value.clone(),
      Value::Argument(_, value) => Some(value.clone()),
      Value::Subcommand(_, value) => value.clone(),
    }
  }
}

#[derive(Debug, Default, Clone)]
pub struct OptionProperties {
  pub name: String,
  pub standalone: bool,
  pub default_missing_value: Option<String>,
  pub takes_value: bool,
}

#[derive(Debug, Default, Clone)]
pub struct SubcommandProperties {
  pub name: String,
  pub value: Option<String>,
}

#[derive(Debug, Default, Clone)]
pub struct ArgumentProperties {
  pub name: String,
  pub required: bool,
}

pub type Evaluator = Box<dyn Fn(&mut VecDeque<Token>, &mut Vec<Value>) -> Result<Option<bool>>>;

/// Returns an evaluator for a short option.
pub fn short_option(name: char, properties: OptionProperties) -> Evaluator {
  Box::new(move |tokens, values| -> Result<Option<bool>> {
    if tokens.is_empty() {
      Ok(Some(false))
    } else {
      if let Some(Token::ShortOption(short_name, mut value)) = tokens.front().cloned()
        && name == short_name
      {
        tokens.pop_front();
        if properties.standalone && !tokens.is_empty() {
          return err!("option must be used alone");
        }
        if !properties.takes_value && value.is_some() {
          return err!("option must not have a value");
        }
        if properties.takes_value && value.is_none() {
          if let Some(Token::Argument(argument_value)) = tokens.front().cloned() {
            tokens.pop_front();
            value = Some(argument_value);
          } else if properties.default_missing_value.is_some() {
            value = properties.default_missing_value.clone();
          } else {
            return err!("option must have a value");
          }
        }
        values.push(Value::short(properties.name.clone(), value));
        Ok(Some(true))
      } else {
        Ok(Some(false))
      }
    }
  })
}

/// Returns an evaluator for a long option.
pub fn long_option(name: impl AsRef<str>, properties: OptionProperties) -> Evaluator {
  let name = name.as_ref().to_string();
  Box::new(move |tokens, values| -> Result<Option<bool>> {
    if tokens.is_empty() {
      Ok(Some(false))
    } else {
      if let Some(Token::LongOption(long_name, mut value)) = tokens.front().cloned()
        && name == long_name
      {
        tokens.pop_front();
        if properties.standalone && !tokens.is_empty() {
          return err!("option must be used alone");
        }
        if !properties.takes_value && value.is_some() {
          return err!("option must not have a value");
        }
        if properties.takes_value && value.is_none() {
          if let Some(Token::Argument(argument_value)) = tokens.front().cloned() {
            tokens.pop_front();
            value = Some(argument_value);
          } else if properties.default_missing_value.is_some() {
            value = properties.default_missing_value.clone();
          } else {
            return err!("option must have a value");
          }
        }
        values.push(Value::long(properties.name.clone(), value));
        Ok(Some(true))
      } else {
        Ok(Some(false))
      }
    }
  })
}

/// Returns an evaluator for a subcommand.
pub fn subcommand(properties: SubcommandProperties) -> Evaluator {
  Box::new(move |tokens, values| -> Result<Option<bool>> {
    if tokens.is_empty() {
      Ok(Some(false))
    } else {
      if let Token::Argument(value) = &tokens[0]
        && properties.name.eq(value)
      {
        tokens.pop_front();
        values.push(Value::subcommand(properties.name.clone(), properties.value.clone()));
        Ok(Some(true))
      } else {
        Ok(Some(false))
      }
    }
  })
}

/// Returns an evaluator for any argument.
pub fn argument(properties: ArgumentProperties) -> Evaluator {
  Box::new(move |tokens, values| -> Result<Option<bool>> {
    if tokens.is_empty() {
      if properties.required {
        Err(err_missing_required_argument(properties.name.clone()))
      } else {
        Ok(Some(false))
      }
    } else {
      if let Token::Argument(value) = tokens[0].clone() {
        tokens.pop_front();
        values.push(Value::argument(properties.name.clone(), value));
        Ok(Some(true))
      } else {
        if properties.required {
          Err(err_missing_required_argument(properties.name.clone()))
        } else {
          Ok(Some(false))
        }
      }
    }
  })
}

/// Returns and evaluator of an alternative matched zero or one time.
pub fn zero_or_one(evaluators: Vec<Evaluator>) -> Evaluator {
  Box::new(move |tokens, values| -> Result<Option<bool>> {
    for evaluator in &evaluators {
      if let Some(true) = evaluator(tokens, values)? {
        return Ok(Some(true));
      }
    }
    Ok(Some(false))
  })
}

/// Returns and evaluator of an alternative matched zero or more times.
pub fn zero_or_more(evaluators: Vec<Evaluator>) -> Evaluator {
  Box::new(move |tokens, values| -> Result<Option<bool>> {
    let mut consumed = false;
    loop {
      let mut matched = false;
      for evaluator in &evaluators {
        if let Some(true) = evaluator(tokens, values)? {
          matched = true;
          consumed = true;
        }
      }
      if !matched {
        break;
      }
    }
    Ok(Some(consumed))
  })
}

/// Returns and evaluator of an alternative matched exactly one time.
pub fn one(evaluators: Vec<Evaluator>) -> Evaluator {
  Box::new(move |tokens, values| -> Result<Option<bool>> {
    for evaluator in &evaluators {
      if let Some(true) = evaluator(tokens, values)? {
        return Ok(Some(true));
      }
    }
    err!("no match")
  })
}

pub fn sequence(evaluators: Vec<Evaluator>) -> Evaluator {
  Box::new(move |tokens, values| -> Result<Option<bool>> {
    for evaluator in &evaluators {
      evaluator(tokens, values)?;
    }
    Ok(None)
  })
}
