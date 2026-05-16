mod arguments;
mod command;
mod commands;
mod empty;
mod helpers;
mod lexer;
mod matches;
mod messages;
mod options;
mod terminator;

const APP: &str = "clars";
const EMPTY_VALUES: Vec<Option<String>> = vec![];
const EMPTY_INPUT: Vec<&str> = vec![];
const VALUE_NONE: Option<String> = None;

use antex::{ColorMode, Text};
use clars::*;

macro_rules! some {
  ($s:expr) => {
    Some($s.to_string())
  };
}

use some;

fn clar() -> Clar {
  Clar::new(APP)
}

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

fn eq_text(expected: impl AsRef<str>, text: Text) {
  assert_eq!(indoc(expected.as_ref()), text.characters());
}

fn eq_diag(expected_msg: impl AsRef<str>, expected_text: impl AsRef<str>, diagnostic: ClarDiagnostic) {
  assert_eq!(expected_msg.as_ref(), diagnostic.to_owned().to_string());
  assert_eq!(indoc(expected_text.as_ref()), diagnostic.text().characters());
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
