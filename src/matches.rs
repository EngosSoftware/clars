use crate::evaluator::Value;
use crate::messages;
use crate::model::{ClarDefinition, ClarItem};
use crate::path::{ClarPath, IntoClarPath};
use antex::{ColorMode, Text};

/// Command line arguments matches.
#[derive(Debug)]
pub struct ClarMatches {
  /// Application name.
  app: String,
  /// Application description.
  description: Option<String>,
  /// Resolved values.
  values: Vec<Value>,
  /// Command line items.
  definition: ClarDefinition,
  /// Color mode.
  cm: ColorMode,
}

impl ClarMatches {
  pub(crate) fn new(app: String, description: Option<String>, values: Vec<Value>, definition: ClarDefinition, cm: ColorMode) -> Self {
    ClarMatches {
      app,
      description,
      values,
      definition,
      cm,
    }
  }

  /// Returns `true` if an option, command or argument with the given path
  /// appears at least once in the command line.
  pub fn is_present(&self, path: impl IntoClarPath) -> bool {
    let path: ClarPath = path.into_clar_path();
    self.values.iter().any(|value| path.eq(value.path()) && value.is_present())
  }

  /// Returns `true` if an option with the given path
  /// appears in the short form in the command line.
  pub fn is_short(&self, path: impl IntoClarPath) -> bool {
    let path: ClarPath = path.into_clar_path();
    self
      .values
      .iter()
      .any(|v| matches!(v, Value::ShortOption(option_path, _) if path.eq(option_path)))
  }

  /// Returns `true` if option with the given path
  /// appears in the long form in the command line.
  pub fn is_long(&self, path: impl IntoClarPath) -> bool {
    let path: ClarPath = path.into_clar_path();
    self
      .values
      .iter()
      .any(|v| matches!(v, Value::LongOption(option_path, _) if path.eq(option_path)))
  }

  /// Returns the number of appearances of the option, command
  /// or argument with the given path in the command line.
  pub fn get_count(&self, path: impl IntoClarPath) -> usize {
    let path: ClarPath = path.into_clar_path();
    self.values.iter().filter(|value| path.eq(value.path()) && value.is_present()).count()
  }

  /// Returns values associated with option, command or argument
  /// with the given path; values of all appearances are returned.
  pub fn get_values(&self, path: impl IntoClarPath) -> Vec<Option<String>> {
    let path: ClarPath = path.into_clar_path();
    let mut values = self
      .values
      .iter()
      .filter(|v| path.eq(v.path()))
      .flat_map(|v| v.value())
      .collect::<Vec<Option<String>>>();
    println!("DDD: c = {}", values.is_empty());
    if values.is_empty()
      && let value @ Some(_) = self.default_value(&path)
    {
      values.push(value);
    }
    values
  }

  /// Returns the value of the first resolved option, command or argument
  /// with the given path.
  pub fn get_first_value(&self, path: impl IntoClarPath) -> Option<String> {
    println!("DDD: b");
    self.get_values(path).first().cloned().flatten()
  }

  pub fn get_help(&self) -> Text {
    messages::get_help_text(&self.app, &self.description, &self.definition, false, self.cm)
  }

  pub fn get_help_long(&self) -> Text {
    messages::get_help_text(&self.app, &self.description, &self.definition, true, self.cm)
  }

  pub fn get_help_command(&self, path: impl IntoClarPath) -> Text {
    messages::get_help_text_for_command(&self.app, &path.into_clar_path(), &self.definition, false, self.cm)
  }

  pub fn get_help_long_command(&self, path: impl IntoClarPath) -> Text {
    messages::get_help_text_for_command(&self.app, &path.into_clar_path(), &self.definition, true, self.cm)
  }

  fn default_value(&self, path: &ClarPath) -> Option<String> {
    println!("DDD: a");
    for item in self.definition.items() {
      match item {
        ClarItem::Options(options) => {
          for option in options {
            if option.get_path() == path && option.get_takes_value().is_some() {
              return option.get_default_value().clone();
            }
          }
        }
        ClarItem::Arguments(arguments) => {
          for argument in arguments {
            if argument.get_path() == path {
              return argument.get_default_value().clone();
            }
          }
        }
        _ => {}
      }
    }
    None
  }
}
