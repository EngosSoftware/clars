use crate::display::display_error_message;
use crate::errors::Result;
use crate::matches::ClarMatches;
use crate::model::{CliArgument, CliItem, CliOption, CliSubcommand};
use crate::parser::{
  ArgumentProperties, Evaluator, OptionProperties, SubcommandProperties, argument, long_option, sequence, short_option,
  subcommand, zero_or_more, zero_or_one,
};
use crate::{Lexer, Token};
use std::collections::VecDeque;

/// Command line arguments resolver.
#[derive(Debug, Clone)]
pub struct Clar {
  /// Name of the application that resolves command line arguments.
  app: String,
  /// Command line items (options, subcommands, arguments) to be resolved.
  items: Vec<CliItem>,
}

impl Clar {
  /// Creates a new command line arguments resolver with given application name.
  pub fn new(app: impl AsRef<str>) -> Self {
    Self {
      app: app.as_ref().to_string(),
      items: vec![],
    }
  }

  /// Returns the name of the application that resolves command line arguments.
  pub fn app(&self) -> &str {
    &self.app
  }

  /// Adds multiple options as an item to be resolved.
  pub fn add_options<I>(&mut self, options: I)
  where
    I: IntoIterator<Item = CliOption>,
  {
    self.items.push(CliItem::Options(options.into_iter().collect()));
  }

  /// Adds multiple arguments as an item to be resolved.
  pub fn add_arguments<I>(&mut self, arguments: I)
  where
    I: IntoIterator<Item = CliArgument>,
  {
    self.items.push(CliItem::Arguments(arguments.into_iter().collect()));
  }

  /// Adds multiple subcommands as an item to be resolved.
  pub fn add_subcommands<I>(&mut self, subcommands: I)
  where
    I: IntoIterator<Item = CliSubcommand>,
  {
    self.items.push(CliItem::Subcommands(subcommands.into_iter().collect()));
  }

  /// Resolves command line arguments into values.
  pub fn resolve<I, S>(self, input: I) -> Result<ClarMatches>
  where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
  {
    // Parse arguments provided from the command line into tokens.
    let mut tokens: VecDeque<Token> = Lexer::default().parse(input)?.iter().cloned().collect();
    // Prepare evaluator for command line arguments to be resolved.
    let evaluator = self.build_evaluator();
    // Resolve command line arguments.
    let mut values = vec![];
    match evaluator(&mut tokens, &mut values) {
      Ok(_) => Ok(ClarMatches {
        values,
        items: self.items,
      }),
      Err(reason) => {
        display_error_message(&reason, &self, &tokens, &values, &self.items);
        Err(reason)
      }
    }
  }

  /// Returns the evaluator for the command line arguments.
  fn build_evaluator(&self) -> Evaluator {
    let mut evaluators = vec![];
    for cli_item in &self.items {
      match cli_item {
        CliItem::Options(cli_options) => {
          let mut options_evaluators = vec![];
          let mut standalone_options_evaluators = vec![];
          for cli_option in cli_options {
            let properties = OptionProperties {
              name: cli_option.name.clone(),
              standalone: cli_option.standalone,
              default_missing_value: cli_option.default_missing_value.clone(),
              takes_value: cli_option.takes_value,
            };
            if cli_option.standalone {
              if let Some(name) = cli_option.short_name {
                standalone_options_evaluators.push(short_option(name, properties.clone()))
              }
              if let Some(name) = &cli_option.long_name {
                standalone_options_evaluators.push(long_option(name, properties))
              }
            } else {
              if let Some(name) = cli_option.short_name {
                options_evaluators.push(short_option(name, properties.clone()))
              }
              if let Some(name) = &cli_option.long_name {
                options_evaluators.push(long_option(name, properties))
              }
            }
          }
          evaluators.push(zero_or_one(standalone_options_evaluators));
          evaluators.push(zero_or_more(options_evaluators));
        }
        CliItem::Subcommands(cli_subcommands) => {
          let mut subcommand_evaluators = vec![];
          for cli_subcommand in cli_subcommands {
            let properties = SubcommandProperties {
              name: cli_subcommand.name.clone(),
              value: cli_subcommand.value.clone(),
            };
            subcommand_evaluators.push(subcommand(properties));
          }
          evaluators.push(zero_or_one(subcommand_evaluators));
        }
        CliItem::Arguments(cli_arguments) => {
          for cli_argument in cli_arguments {
            let properties = ArgumentProperties {
              name: cli_argument.name.clone(),
              required: cli_argument.required,
            };
            evaluators.push(argument(properties));
          }
        }
      }
    }
    sequence(evaluators)
  }
}
