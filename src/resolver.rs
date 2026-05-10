use crate::errors::{ClarDiagnostic, ClarDiagnosticResult};
use crate::evaluator::{
  Evaluator, ev_and_then, ev_argument, ev_consumed, ev_long_option, ev_one, ev_option_terminator, ev_sequence,
  ev_short_option, ev_stop_on_one, ev_subcommand, ev_zero_or_more_options,
};
use crate::lexer::{Lexer, Token};
use crate::matches::ClarMatches;
use crate::messages::get_error;
use crate::model::{ClarArgument, ClarCommand, ClarItem, ClarOption, ClarTerminator, update_paths};
use antex::ColorMode;
use std::collections::VecDeque;

/// Command line arguments resolver.
#[derive(Debug, Clone)]
pub struct Clar {
  /// Application name.
  app: String,
  /// Application description.
  description: Option<String>,
  /// Command line items (options, subcommands, arguments) to be resolved.
  items: Vec<ClarItem>,
  /// Color mode.
  cm: ColorMode,
}

impl Clar {
  /// Creates a new arguments resolver with given application name.
  pub fn new(app: impl AsRef<str>) -> Self {
    Self {
      app: app.as_ref().to_string(),
      description: None,
      items: vec![],
      cm: ColorMode::default(),
    }
  }

  /// Sets application description.
  pub fn description(mut self, description: impl AsRef<str>) -> Self {
    self.description = Some(description.as_ref().to_string());
    self
  }

  /// Returns resolver that recognizes an **option terminator**.
  pub fn terminator(mut self, t: ClarTerminator) -> Self {
    self.items.clear();
    self.items.push(ClarItem::Terminator(t));
    self
  }

  /// Returns resolver that recognizes **options**.
  pub fn options<O>(mut self, o: O) -> Self
  where
    O: IntoIterator<Item = ClarOption>,
  {
    self.items.clear();
    self.items.push(ClarItem::Options(o.into_iter().collect()));
    self
  }

  /// Returns resolver that recognizes **options** followed by **option terminator**.
  pub fn options_t<O>(&mut self, o: O, t: ClarTerminator)
  where
    O: IntoIterator<Item = ClarOption>,
  {
    self.items.clear();
    self.items.push(ClarItem::Options(o.into_iter().collect()));
    self.items.push(ClarItem::Terminator(t));
  }

  /// Returns resolver that recognizes **arguments**.
  pub fn arguments<A>(mut self, a: A) -> Self
  where
    A: IntoIterator<Item = ClarArgument>,
  {
    self.items.clear();
    self.items.push(ClarItem::Arguments(a.into_iter().collect()));
    self
  }

  /// Returns resolver that recognizes **arguments** followed by **option terminator**.
  pub fn arguments_t<A>(mut self, a: A, t: ClarTerminator) -> Self
  where
    A: IntoIterator<Item = ClarArgument>,
  {
    self.items.clear();
    self.items.push(ClarItem::Arguments(a.into_iter().collect()));
    self.items.push(ClarItem::Terminator(t));
    self
  }

  /// Returns resolver that recognizes **options** followed by **arguments**.
  pub fn options_arguments<O, A>(mut self, o: O, a: A) -> Self
  where
    O: IntoIterator<Item = ClarOption>,
    A: IntoIterator<Item = ClarArgument>,
  {
    self.items.clear();
    self.items.push(ClarItem::Options(o.into_iter().collect()));
    self.items.push(ClarItem::Arguments(a.into_iter().collect()));
    self
  }

  /// Returns resolver that recognizes **options** followed by **arguments** and **option terminator**.
  pub fn options_arguments_t<O, A>(mut self, o: O, a: A, t: ClarTerminator) -> Self
  where
    O: IntoIterator<Item = ClarOption>,
    A: IntoIterator<Item = ClarArgument>,
  {
    self.items.clear();
    self.items.push(ClarItem::Options(o.into_iter().collect()));
    self.items.push(ClarItem::Arguments(a.into_iter().collect()));
    self.items.push(ClarItem::Terminator(t));
    self
  }

  /// Returns resolver that recognizes **subcommands**.
  pub fn subcommands<S>(mut self, s: S) -> Self
  where
    S: IntoIterator<Item = ClarCommand>,
  {
    self.items.clear();
    self.items.push(ClarItem::Commands(s.into_iter().collect()));
    self
  }

  /// Returns resolver that recognizes **options** followed by **subcommands**.
  pub fn options_subcommands<O, S>(mut self, options: O, subcommands: S) -> Self
  where
    O: IntoIterator<Item = ClarOption>,
    S: IntoIterator<Item = ClarCommand>,
  {
    self.items.clear();
    self.items.push(ClarItem::Options(options.into_iter().collect()));
    self.items.push(ClarItem::Commands(subcommands.into_iter().collect()));
    self
  }

  /// Resolves command line arguments.
  pub fn resolve<I, S>(mut self, input: I) -> ClarDiagnosticResult<ClarMatches>
  where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
  {
    // Update paths to command line elements in definitions tree.
    update_paths(&mut self.items, &mut vec![]);
    // Parse command line arguments into tokens.
    let mut tokens: VecDeque<Token> = Lexer::default().parse(input).map_err(|reason| {
      let text = get_error(&reason, &self.app, &self.items, self.cm);
      ClarDiagnostic::new(reason, text)
    })?;
    // Prepare evaluator for command line arguments basing definitions.
    let evaluator = ev_consumed(self.build_evaluator(&self.items));
    // Resolve command line arguments using evaluator.
    let mut values = vec![];
    match evaluator(&mut tokens, &mut values) {
      Ok(_) => Ok(ClarMatches::new(
        self.app,
        self.description,
        values,
        self.items,
        self.cm,
      )),
      Err(reason) => {
        let text = get_error(&reason, &self.app, &self.items, self.cm);
        Err(ClarDiagnostic::new(reason, text))
      }
    }
  }

  /// Returns the evaluator for the command line arguments.
  fn build_evaluator(&self, items: &[ClarItem]) -> Evaluator {
    let mut evaluators = vec![];
    for item in items {
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
        ClarItem::Commands(subcommands) => {
          let mut subcommand_evaluators = vec![];
          for subcommand in subcommands {
            subcommand_evaluators.push(ev_and_then(
              ev_subcommand(subcommand.into()),
              self.build_evaluator(subcommand.get_items()),
            ));
          }
          evaluators.push(ev_one(subcommand_evaluators));
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
}
