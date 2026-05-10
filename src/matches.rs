use crate::evaluator::Value;
use crate::messages;
use crate::model::ClarItem;
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
  items: Vec<ClarItem>,
  /// Color mode.
  cm: ColorMode,
}

impl ClarMatches {
  pub(crate) fn new(
    app: String,
    description: Option<String>,
    values: Vec<Value>,
    items: Vec<ClarItem>,
    cm: ColorMode,
  ) -> Self {
    ClarMatches {
      app,
      description,
      values,
      items,
      cm,
    }
  }

  /// Returns raw values resolved from command line input.
  pub fn raw_values(&self) -> &[Value] {
    &self.values
  }

  /// Returns `true` if an option, subcommand, or argument with the given
  /// name appears at least once in the command line.
  pub fn is_present(&self, path: impl AsRef<str>) -> bool {
    let name = path.as_ref();
    self
      .values
      .iter()
      .any(|value| value.path() == name && value.is_present())
  }

  /// Returns `true` if option with the given name
  /// appears in the **short** form in the command line.
  pub fn is_short(&self, name: impl AsRef<str>) -> bool {
    let name = name.as_ref();
    self
      .values
      .iter()
      .any(|v| matches!(v, Value::ShortOption(option_name, _) if option_name == name))
  }

  /// Returns `true` if option with the given name
  /// appears in the **long** form in the command line.
  pub fn is_long(&self, name: impl AsRef<str>) -> bool {
    let name = name.as_ref();
    self
      .values
      .iter()
      .any(|v| matches!(v, Value::LongOption(option_name, _) if option_name == name))
  }

  /// Returns the number of appearances of the option, subcommand or argument
  /// with the given name in the command line.
  pub fn get_count(&self, name: impl AsRef<str>) -> usize {
    let name = name.as_ref();
    self
      .values
      .iter()
      .filter(|value| value.path() == name && value.is_present())
      .count()
  }

  /// Returns values associated with option, subcommand or argument
  /// with the given name; values of all appearances are returned.
  pub fn get_values(&self, name: impl AsRef<str>) -> Vec<Option<String>> {
    let name = name.as_ref();
    let mut values = self
      .values
      .iter()
      .filter(|v| v.path() == name)
      .flat_map(|v| v.value())
      .collect::<Vec<Option<String>>>();
    if values.is_empty()
      && let value @ Some(_) = self.default_value(name)
    {
      values.push(value);
    }
    values
  }

  /// Returns paths of all matched subcommands, options and arguments.
  pub fn get_paths(&self) -> Vec<String> {
    self.values.iter().map(|value| value.path().to_string()).collect()
  }

  pub fn get_help(&self) -> Text {
    messages::get_help(&self.app, &self.description, &self.items, false, self.cm)
  }

  pub fn get_help_long(&self) -> Text {
    messages::get_help(&self.app, &self.description, &self.items, true, self.cm)
  }

  pub fn get_help_command(&self, path: impl AsRef<str>) -> Text {
    messages::get_help_command(&self.app, path.as_ref(), &self.items, false, self.cm)
  }

  pub fn get_help_long_command(&self, path: impl AsRef<str>) -> Text {
    messages::get_help_command(&self.app, path.as_ref(), &self.items, true, self.cm)
  }

  fn default_value(&self, name: impl AsRef<str>) -> Option<String> {
    let name = name.as_ref();
    for item in &self.items {
      match item {
        ClarItem::Options(options) => {
          for option in options {
            if option.get_name() == name && option.get_takes_value().is_some() {
              return option.get_default_value().clone();
            }
          }
        }
        ClarItem::Arguments(arguments) => {
          for argument in arguments {
            if argument.get_name() == name {
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
