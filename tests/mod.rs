mod arguments;
mod empty;
mod lexer;
mod matches;
mod messages;
mod options;
mod subcommands;
mod terminator;
mod test_lexer;
mod test_model;
mod test_parser;

const APP: &str = "clars";
const EMPTY_VALUES: Vec<Option<String>> = vec![];
const EMPTY_INPUT: Vec<&str> = vec![];
const VALUE_NONE: Option<String> = None;

use antex::Text;
use clars::*;

macro_rules! some {
  ($s:expr) => {
    Some($s.to_string())
  };
}

use some;

fn argument(name: &str, default_value: Option<String>, required: bool) -> ClarArgument {
  let mut argument = ClarArgument::new(name);
  if let Some(default_value) = default_value {
    argument = argument.default_value(default_value);
  }
  if required {
    argument = argument.required();
  }
  argument
}

fn eq_msg(expected: impl AsRef<str>, text: Text) {
  assert_eq!(indoc(expected.as_ref()), text.chars().collect::<String>());
}

fn indoc(input: &str) -> String {
  // Remove the first newline.
  let s = input.trim_start_matches('\n');
  // Find the minimum indentation across all non-empty lines.
  let indent = s
    .lines()
    .filter(|line| !line.trim().is_empty())
    .map(|line| line.len() - line.trim_start().len())
    .min()
    .unwrap_or(0);
  // Strip the indentation from the start of each non-empty line and whitespaces from the end of the last line.
  s.lines()
    .map(|line| if line.len() >= indent { &line[indent..] } else { line })
    .collect::<Vec<_>>()
    .join("\n")
    .trim_end_matches("  ")
    .to_string()
}
