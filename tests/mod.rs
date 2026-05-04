mod matches;
mod test_lexer;
mod test_model;
mod test_parser;
mod use_cases;

const EMPTY_VALUES: Vec<Option<String>> = vec![];
const EMPTY_INPUT: Vec<&str> = vec![];
const VALUE_NONE: Option<String> = None;

use clars::model::{CliArgument, CliOption, CliSubcommand};
use clars::{Clar, Result};

macro_rules! some {
  ($s:expr) => {
    Some($s.to_string())
  };
}

use some;

fn argument(name: &str, default_value: Option<String>, required: bool) -> CliArgument {
  CliArgument {
    name: name.to_string(),
    default_value,
    required,
  }
}
