use crate::errors::*;
use crate::lexer::Token;
use crate::path::ClarPath;
use crate::{ClarArgument, ClarCommand, ClarOption, ClarTerminator};
use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
  /// Short option with optional value.
  ShortOption(
    /// Path in the definition tree.
    ClarPath,
    /// Optional value assigned to short option.
    Option<String>,
  ),
  /// Long option with optional value.
  LongOption(
    /// Path in the definition tree.
    ClarPath,
    /// Optional value assigned to long option.
    Option<String>,
  ),
  /// Command.
  Command(
    /// Path in the definition tree.
    ClarPath,
  ),
  /// Argument.
  Argument(
    /// Path in the definition tree.
    ClarPath,
    /// Argument value.
    String,
    /// Indicates if the argument was present in the input,
    /// or its value was taken from the default value.
    bool,
  ),
  OptionTerminator(
    /// Path in the definition tree.
    ClarPath,
    /// Arguments after option terminator.
    Vec<String>,
  ),
}

impl Value {
  pub fn short(path: ClarPath, value: Option<String>) -> Self {
    Value::ShortOption(path, value)
  }

  pub fn long(path: ClarPath, value: Option<String>) -> Self {
    Value::LongOption(path, value)
  }

  pub fn command(path: ClarPath) -> Self {
    Value::Command(path)
  }

  pub fn argument(path: ClarPath, value: String, present: bool) -> Self {
    Value::Argument(path, value, present)
  }

  pub fn terminator(path: ClarPath, values: Vec<String>) -> Self {
    Value::OptionTerminator(path, values)
  }

  /// Returns the path in definition tree.
  pub fn path(&self) -> &ClarPath {
    match self {
      Value::ShortOption(path, _) => path,
      Value::LongOption(path, _) => path,
      Value::Command(path) => path,
      Value::Argument(path, _, _) => path,
      Value::OptionTerminator(path, _) => path,
    }
  }

  /// Returns the value.
  pub fn value(&self) -> Vec<Option<String>> {
    match self {
      Value::ShortOption(_, value) => vec![value.clone()],
      Value::LongOption(_, value) => vec![value.clone()],
      Value::Argument(_, value, _) => vec![Some(value.clone())],
      Value::Command(_) => vec![],
      Value::OptionTerminator(_, value) => value.iter().map(|s| Some(s.clone())).collect(),
    }
  }

  /// Returns `true` if the value for option, argument or subcommand was present in the input
  /// and was not taken from the default value in definition.
  pub fn is_present(&self) -> bool {
    match self {
      Value::ShortOption(_, _) => true,
      Value::LongOption(_, _) => true,
      Value::Argument(_, _, present) => *present,
      Value::Command(_) => true,
      Value::OptionTerminator(_, _) => true,
    }
  }
}

#[derive(Debug, Default, Clone)]
pub struct EvalOption {
  path: ClarPath,
  standalone: bool,
  takes_value: Option<String>,
  default_missing_value: Option<String>,
  possible_values: Vec<String>,
  synopsis: String,
}

impl From<&ClarOption> for EvalOption {
  fn from(value: &ClarOption) -> Self {
    Self {
      path: value.get_path().clone(),
      standalone: value.is_standalone(),
      takes_value: value.get_takes_value().clone(),
      default_missing_value: value.get_default_missing_value().clone(),
      possible_values: value.get_possible_values().clone(),
      synopsis: value.get_synopsis().clone(),
    }
  }
}

#[derive(Debug, Default, Clone)]
pub struct EvalCommand {
  name: String,
  path: ClarPath,
}

impl From<&ClarCommand> for EvalCommand {
  fn from(value: &ClarCommand) -> Self {
    Self {
      name: value.get_name().to_string(),
      path: value.get_path().clone(),
    }
  }
}

#[derive(Debug, Default, Clone)]
pub struct EvalArgument {
  path: ClarPath,
  caption: String,
  required: bool,
}

impl From<&ClarArgument> for EvalArgument {
  fn from(value: &ClarArgument) -> Self {
    Self {
      path: value.get_path().clone(),
      caption: value.get_caption().to_string(),
      required: value.is_required(),
    }
  }
}

#[derive(Debug, Default, Clone)]
pub struct EvalOptionTerminator {
  path: ClarPath,
  required: bool,
}

impl From<&ClarTerminator> for EvalOptionTerminator {
  fn from(value: &ClarTerminator) -> Self {
    Self {
      path: value.get_path().clone(),
      required: value.is_required(),
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Status {
  NoMatch,
  Match,
  Stop,
}

pub type Evaluator = Box<dyn Fn(&mut VecDeque<Token>, &mut Vec<Value>) -> ClarResult<Status>>;

/// Returns an evaluator for a short option.
pub fn ev_short_option(label: char, option: EvalOption) -> Evaluator {
  Box::new(move |tokens, values| -> ClarResult<Status> {
    if tokens.is_empty() {
      Ok(Status::NoMatch)
    } else {
      if let Some(Token::ShortOption(short_label, mut value)) = tokens.front().cloned()
        && label == short_label
      {
        tokens.pop_front();
        if option.standalone && !tokens.is_empty() {
          return Err(err_short_option_must_be_used_alone(short_label));
        }
        if option.takes_value.is_none() && value.is_some() {
          return Err(err_short_option_does_not_accept_value(short_label));
        }
        if let Some(caption) = &option.takes_value
          && value.is_none()
        {
          if let Some(Token::Argument(argument_value)) = tokens.front().cloned() {
            tokens.pop_front();
            value = Some(argument_value);
          } else if option.default_missing_value.is_some() {
            value = option.default_missing_value.clone();
          } else {
            return Err(err_short_option_requires_value(short_label, caption.clone()));
          }
        }
        if !option.possible_values.is_empty()
          && let Some(value) = &value
          && !option.possible_values.contains(value)
        {
          return Err(err_invalid_value_for_option(
            option.synopsis.clone(),
            value.clone(),
            option.possible_values.clone(),
          ));
        }
        values.push(Value::short(option.path.clone(), value));
        Ok(Status::Match)
      } else {
        Ok(Status::NoMatch)
      }
    }
  })
}

/// Returns an evaluator for a long option.
pub fn ev_long_option(label: impl AsRef<str>, option: EvalOption) -> Evaluator {
  let label = label.as_ref().to_string();
  Box::new(move |tokens, values| -> ClarResult<Status> {
    if tokens.is_empty() {
      Ok(Status::NoMatch)
    } else {
      if let Some(Token::LongOption(long_label, mut value)) = tokens.front().cloned()
        && label == long_label
      {
        tokens.pop_front();
        if option.standalone && !tokens.is_empty() {
          return Err(err_long_option_must_be_used_alone(long_label));
        }
        if option.takes_value.is_none() && value.is_some() {
          return Err(err_long_option_does_not_accept_value(long_label));
        }
        if let Some(caption) = &option.takes_value
          && value.is_none()
        {
          if let Some(Token::Argument(argument_value)) = tokens.front().cloned() {
            tokens.pop_front();
            value = Some(argument_value);
          } else if option.default_missing_value.is_some() {
            value = option.default_missing_value.clone();
          } else {
            return Err(err_long_option_requires_value(long_label, caption.clone()));
          }
        }
        if !option.possible_values.is_empty()
          && let Some(value) = &value
          && !option.possible_values.contains(value)
        {
          return Err(err_invalid_value_for_option(
            option.synopsis.clone(),
            value.clone(),
            option.possible_values.clone(),
          ));
        }
        values.push(Value::long(option.path.clone(), value));
        Ok(Status::Match)
      } else {
        Ok(Status::NoMatch)
      }
    }
  })
}

/// Returns an evaluator for a command.
pub fn ev_command(command: EvalCommand) -> Evaluator {
  Box::new(move |tokens, values| -> ClarResult<Status> {
    if tokens.is_empty() {
      Ok(Status::NoMatch)
    } else {
      if let Token::Argument(value) = &tokens[0]
        && command.name.eq(value)
      {
        tokens.pop_front();
        // Remove all parent commands from already resolved values.
        // Only the 'leaf' command should be left in resolved values
        if let Some(parent_path) = command.path.parent() {
          values.retain(|value| {
            if let Value::Command(path) = value {
              return parent_path.ne(path);
            }
            true
          });
        }
        // A new command is resolved.
        values.push(Value::command(command.path.clone()));
        Ok(Status::Match)
      } else {
        Ok(Status::NoMatch)
      }
    }
  })
}

/// Returns an evaluator for an argument.
pub fn ev_argument(argument: EvalArgument) -> Evaluator {
  Box::new(move |tokens, values| -> ClarResult<Status> {
    let err_on_required = |e| {
      if argument.required { Err(e) } else { Ok(Status::NoMatch) }
    };
    match tokens.front().cloned() {
      Some(token) => match token {
        Token::Argument(value) => {
          tokens.pop_front();
          values.push(Value::argument(argument.path.clone(), value, true));
          Ok(Status::Match)
        }
        Token::ShortOption(label, _) => err_on_required(err_unexpected_short_option(label)),
        Token::LongOption(label, _) => err_on_required(err_unexpected_long_option(label)),
        Token::OptionTerminator(_) => err_on_required(err_unexpected_option_terminator()),
      },
      None => {
        if argument.required {
          Err(err_missing_required_argument(argument.caption.clone()))
        } else {
          Ok(Status::NoMatch)
        }
      }
    }
  })
}

/// Returns an evaluator for an option terminator.
pub fn ev_option_terminator(terminator: EvalOptionTerminator) -> Evaluator {
  Box::new(move |tokens, values| -> ClarResult<Status> {
    let err_on_required = |e| {
      if terminator.required { Err(e) } else { Ok(Status::NoMatch) }
    };
    match tokens.front().cloned() {
      Some(token) => match token {
        Token::OptionTerminator(arguments) => {
          tokens.pop_front();
          values.push(Value::terminator(terminator.path.clone(), arguments));
          Ok(Status::Match)
        }
        Token::ShortOption(label, _) => err_on_required(err_unexpected_short_option(label)),
        Token::LongOption(label, _) => err_on_required(err_unexpected_long_option(label)),
        Token::Argument(value) => err_on_required(err_unexpected_argument(value)),
      },
      None => {
        if terminator.required {
          Err(err_missing_required_terminator())
        } else {
          Ok(Status::NoMatch)
        }
      }
    }
  })
}

/// Returns and evaluator of an alternative matched zero or one time.
pub fn ev_stop_on_one(evaluators: Vec<Evaluator>) -> Evaluator {
  Box::new(move |tokens, values| -> ClarResult<Status> {
    for evaluator in &evaluators {
      if let Status::Match = evaluator(tokens, values)? {
        return Ok(Status::Stop);
      }
    }
    Ok(Status::NoMatch)
  })
}

/// Returns and evaluator of an alternative of options that can be matched zero or more times.
pub fn ev_zero_or_more_options(evaluators: Vec<Evaluator>) -> Evaluator {
  Box::new(move |tokens, values| -> ClarResult<Status> {
    let mut status = Status::NoMatch;
    // Evaluate all options in tokens.
    loop {
      let mut matched = false;
      for evaluator in &evaluators {
        if let Status::Match = evaluator(tokens, values)? {
          matched = true;
          status = Status::Match;
        }
      }
      if !matched {
        break;
      }
    }
    // There should be no more options in tokens left.
    if let Some(Token::ShortOption(short_label, _)) = tokens.front() {
      return Err(err_unexpected_short_option(*short_label));
    }
    if let Some(Token::LongOption(long_label, _)) = tokens.front() {
      return Err(err_unexpected_long_option(long_label.clone()));
    }
    Ok(status)
  })
}

/// Returns and evaluator of an alternative matched exactly one time.
pub fn ev_one(evaluators: Vec<Evaluator>) -> Evaluator {
  Box::new(move |tokens, values| -> ClarResult<Status> {
    let mut status = Status::NoMatch;
    for evaluator in &evaluators {
      status = status.max(evaluator(tokens, values)?);
      if status >= Status::Match {
        return Ok(status);
      }
    }
    expect_no_more_tokens(tokens)?;
    Ok(status)
  })
}

pub fn ev_sequence(evaluators: Vec<Evaluator>) -> Evaluator {
  Box::new(move |tokens, values| -> ClarResult<Status> {
    let mut status = Status::NoMatch;
    for evaluator in &evaluators {
      status = status.max(evaluator(tokens, values)?);
      if status == Status::Stop {
        return Ok(status);
      }
    }
    Ok(status)
  })
}

pub fn ev_command_and_then(command: EvalCommand, evaluator_then: Evaluator) -> Evaluator {
  let evaluator = ev_command(command);
  Box::new(move |tokens, values| -> ClarResult<Status> {
    let mut status = evaluator(tokens, values).unwrap(); // Unwrap is safe, ev_command does not return errors.
    if status == Status::Match {
      status = status.max(evaluator_then(tokens, values)?);
    }
    Ok(status)
  })
}

pub fn ev_consumed(evaluator: Evaluator) -> Evaluator {
  Box::new(move |tokens, values| -> ClarResult<Status> {
    let status = evaluator(tokens, values)?;
    expect_no_more_tokens(tokens)?;
    Ok(status)
  })
}

fn expect_no_more_tokens(tokens: &VecDeque<Token>) -> ClarResult<()> {
  match tokens.front() {
    Some(token) => match token {
      Token::ShortOption(short_label, _) => Err(err_unexpected_short_option(*short_label)),
      Token::LongOption(long_label, _) => Err(err_unexpected_long_option(long_label.clone())),
      Token::Argument(value) => Err(err_unexpected_argument(value.clone())),
      Token::OptionTerminator(_) => Err(err_unexpected_option_terminator()),
    },
    None => Ok(()),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn status_ordering_should_be_preserved() {
    assert!(Status::Stop > Status::Match);
    assert!(Status::Match > Status::NoMatch);
    assert_eq!(Status::Stop, Status::Match.max(Status::Stop));
    assert_eq!(Status::Match, Status::NoMatch.max(Status::Match));
  }
}
