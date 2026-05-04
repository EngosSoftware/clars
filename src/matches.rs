use crate::model::CliItem;
use crate::parser::Value;

#[derive(Debug)]
pub struct ClarMatches {
  pub(crate) values: Vec<Value>,
  pub(crate) items: Vec<CliItem>,
}

impl ClarMatches {
  /// Returns raw values resolved from command line input.
  pub fn raw_values(&self) -> &[Value] {
    &self.values
  }

  /// Returns `true` if an option, subcommand, or argument with the given
  /// name appears at least once in the command line.
  pub fn is_present(&self, name: impl AsRef<str>) -> bool {
    let name = name.as_ref();
    self.values.iter().any(|v| v.name() == name)
  }

  /// Returns the number of appearances of the option, subcommand or argument
  /// with the given name in the command line.
  pub fn get_count(&self, name: impl AsRef<str>) -> usize {
    let name = name.as_ref();
    self.values.iter().filter(|v| v.name() == name).count()
  }

  /// Returns values associated with option, subcommand or argument
  /// with the given name; values of all appearances are returned.
  pub fn get_values(&self, name: impl AsRef<str>) -> Vec<Option<String>> {
    let name = name.as_ref();
    let mut values = self
      .values
      .iter()
      .filter(|v| v.name() == name)
      .map(|v| v.value())
      .collect::<Vec<Option<String>>>();
    if values.is_empty()
      && let value @ Some(_) = self.default_value(name)
    {
      values.push(value);
    }
    values
  }

  fn default_value(&self, name: impl AsRef<str>) -> Option<String> {
    let name = name.as_ref();
    for item in &self.items {
      match item {
        CliItem::Options(options) => {
          for option in options {
            if option.name == name && option.takes_value {
              return option.default_value.clone();
            }
          }
        }
        CliItem::Subcommands(subcommands) => {
          for subcommand in subcommands {
            if subcommand.name == name {
              return subcommand.default_value.clone();
            }
          }
        }
        CliItem::Arguments(arguments) => {
          for argument in arguments {
            if argument.name == name {
              return argument.default_value.clone();
            }
          }
        }
      }
    }
    None
  }
}
