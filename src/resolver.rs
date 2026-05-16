use crate::ClarResult;
use crate::errors::{ClarDiagnostic, ClarDiagnosticResult, err_exceeded_max_option_occurrences};
use crate::evaluator::{
  Evaluator, Value, ev_argument, ev_command_and_then, ev_consumed, ev_long_option, ev_one, ev_option_terminator, ev_sequence, ev_short_option, ev_stop_on_one,
  ev_zero_or_more_options,
};
use crate::lexer::{Lexer, Token};
use crate::matches::ClarMatches;
use crate::messages::{get_cfg_error_text, get_error_text};
use crate::model::{ClarArgument, ClarCommand, ClarDefinition, ClarItem, ClarOption, ClarTerminator};
use crate::path::ClarPath;
use antex::ColorMode;
use std::collections::{HashMap, VecDeque};

/// Command-line arguments resolver.
#[derive(Debug, Clone)]
pub struct Clar {
  /// Application name.
  app: String,
  /// Application description.
  description: Option<String>,
  /// Command-line definition.
  definition: ClarDefinition,
  /// Color mode.
  cm: ColorMode,
}

impl Clar {
  /// Creates a new arguments resolver with given application name.
  pub fn new(app: impl AsRef<str>) -> Self {
    Self {
      app: app.as_ref().to_string(),
      description: None,
      definition: ClarDefinition::default(),
      cm: ColorMode::default(),
    }
  }

  /// Sets application description.
  pub fn description(mut self, description: impl AsRef<str>) -> Self {
    self.description = Some(description.as_ref().to_string());
    self
  }

  /// Returns a resolver that recognizes an option terminator.
  pub fn terminator(mut self, option_terminator: ClarTerminator) -> Self {
    self.definition.terminator(option_terminator);
    self
  }

  /// Returns a resolver that recognizes options.
  pub fn options<O>(mut self, options: O) -> Self
  where
    O: IntoIterator<Item = ClarOption>,
  {
    self.definition.options(options);
    self
  }

  /// Returns a resolver that recognizes options followed by option terminator.
  pub fn options_terminator<O>(mut self, options: O, option_terminator: ClarTerminator) -> Self
  where
    O: IntoIterator<Item = ClarOption>,
  {
    self.definition.options_terminator(options, option_terminator);
    self
  }

  /// Returns a resolver that recognizes arguments.
  pub fn arguments<A>(mut self, arguments: A) -> Self
  where
    A: IntoIterator<Item = ClarArgument>,
  {
    self.definition.arguments(arguments);
    self
  }

  /// Returns a resolver that recognizes arguments followed by option terminator.
  pub fn arguments_terminator<A>(mut self, arguments: A, option_terminator: ClarTerminator) -> Self
  where
    A: IntoIterator<Item = ClarArgument>,
  {
    self.definition.arguments_terminator(arguments, option_terminator);
    self
  }

  /// Returns a resolver that recognizes options followed by arguments.
  pub fn options_arguments<O, A>(mut self, options: O, arguments: A) -> Self
  where
    O: IntoIterator<Item = ClarOption>,
    A: IntoIterator<Item = ClarArgument>,
  {
    self.definition.options_arguments(options, arguments);
    self
  }

  /// Returns a resolver that recognizes options followed by arguments and option terminator.
  pub fn options_arguments_terminator<O, A>(mut self, options: O, arguments: A, option_terminator: ClarTerminator) -> Self
  where
    O: IntoIterator<Item = ClarOption>,
    A: IntoIterator<Item = ClarArgument>,
  {
    self.definition.options_arguments_terminator(options, arguments, option_terminator);
    self
  }

  /// Returns a resolver that recognizes commands.
  pub fn commands<C>(mut self, commands: C) -> Self
  where
    C: IntoIterator<Item = ClarCommand>,
  {
    self.definition.commands(commands);
    self
  }

  /// Returns a resolver that recognizes options followed by commands.
  pub fn options_commands<O, C>(mut self, options: O, commands: C) -> Self
  where
    O: IntoIterator<Item = ClarOption>,
    C: IntoIterator<Item = ClarCommand>,
  {
    self.definition.options_commands(options, commands);
    self
  }

  /// Resolves command line arguments.
  pub fn resolve<I, S>(mut self, input: I) -> ClarDiagnosticResult<ClarMatches>
  where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
  {
    // Validate the command-line definition.
    self.definition.validate().map_err(|reason| {
      let text = get_cfg_error_text(&reason, self.cm);
      ClarDiagnostic::new(reason, text)
    })?;
    // Parse command-line arguments into tokens.
    let mut tokens: VecDeque<Token> = Lexer::default().parse(input).map_err(|reason| {
      let text = get_error_text(&reason, &self.app, &self.definition, &[], self.cm);
      ClarDiagnostic::new(reason, text)
    })?;
    // Prepare evaluator for command-line arguments based on definitions.
    let evaluator = ev_consumed(self.build_evaluator(&self.definition));
    // Resolve command-line arguments using evaluator.
    let mut values = vec![];
    let result = evaluator(&mut tokens, &mut values);
    // Find the resolved command names to adjust usage message.
    let names = if let Some(Value::Command(path)) = values.iter().find(|value| matches!(value, Value::Command(_))) {
      path.parent_names()
    } else {
      &[]
    };
    match result {
      Ok(_) => {
        self.validate_values(&values).map_err(|reason| {
          let text = get_error_text(&reason, &self.app, &self.definition, names, self.cm);
          ClarDiagnostic::new(reason, text)
        })?;
        Ok(ClarMatches::new(self.app, self.description, values, self.definition, self.cm))
      }
      Err(reason) => {
        let text = get_error_text(&reason, &self.app, &self.definition, names, self.cm);
        Err(ClarDiagnostic::new(reason, text))
      }
    }
  }

  /// Returns the evaluator for the command line arguments.
  fn build_evaluator(&self, definition: &ClarDefinition) -> Evaluator {
    let mut evaluators = vec![];
    for item in definition.items() {
      match item {
        ClarItem::Options(options) => {
          let mut options_evaluators = vec![];
          let mut standalone_options_evaluators = vec![];
          for option in options {
            if option.is_standalone() {
              if let Some(short_label) = option.get_short_label() {
                standalone_options_evaluators.push(ev_short_option(*short_label, option.into()))
              }
              if let Some(long_label) = option.get_long_label() {
                standalone_options_evaluators.push(ev_long_option(long_label, option.into()))
              }
            } else {
              if let Some(short_label) = option.get_short_label() {
                options_evaluators.push(ev_short_option(*short_label, option.into()))
              }
              if let Some(long_label) = option.get_long_label() {
                options_evaluators.push(ev_long_option(long_label, option.into()))
              }
            }
          }
          evaluators.push(ev_stop_on_one(standalone_options_evaluators));
          evaluators.push(ev_zero_or_more_options(options_evaluators));
        }
        ClarItem::Commands(commands) => {
          let mut command_evaluators = vec![];
          for command in commands {
            command_evaluators.push(ev_command_and_then(command.into(), self.build_evaluator(command.get_definition())));
          }
          evaluators.push(ev_one(command_evaluators));
        }
        ClarItem::Arguments(arguments) => {
          for argument in arguments {
            evaluators.push(ev_argument(argument.into()));
          }
        }
        ClarItem::Terminator(option_terminator) => evaluators.push(ev_option_terminator(option_terminator.into())),
      }
    }
    ev_sequence(evaluators)
  }

  fn validate_values(&self, values: &[Value]) -> ClarResult<()> {
    self.expect_max_option_occurrences(values)?;
    Ok(())
  }

  fn expect_max_option_occurrences(&self, values: &[Value]) -> ClarResult<()> {
    let mut totals: HashMap<ClarPath, usize> = HashMap::new();
    let mut paths = vec![];
    for value in values {
      match value {
        Value::ShortOption(path, _) | Value::LongOption(path, _) => {
          totals.entry(path.clone()).and_modify(|count| *count += 1).or_insert(1);
          paths.push(path.clone());
        }
        _ => {}
      }
    }
    let options = self.definition.get_options();
    for path in &paths {
      let occurrences = *totals.get(path).unwrap(); // This is safe, path MUST exist in totals.
      let option = options.get(path).unwrap(); // This is safe, option MUST exist for path.
      let max = option.get_max_occurrences();
      if occurrences > max {
        return Err(err_exceeded_max_option_occurrences(option.get_synopsis(), max, occurrences));
      }
    }
    Ok(())
  }
}
