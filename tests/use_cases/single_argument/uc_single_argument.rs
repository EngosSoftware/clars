//! # Single argument
//!
//! ```text
//!
//! Usage: command [FILE]
//!
//! Usage: command <FILE>
//!

use super::*;

const NAME: &str = "FILE";

fn clar(config: (bool, Option<String>)) -> Clar {
  let mut clar = Clar::new("command");
  clar.add_arguments(vec![argument(NAME, config.1, config.0)]);
  clar
}

fn pass<I, S>(config: (bool, Option<String>), input: I, expected: (bool, usize, Vec<Option<String>>))
where
  I: IntoIterator<Item = S>,
  S: AsRef<str>,
{
  let matches = clar(config).resolve(input).unwrap();
  assert_eq!(expected.0, matches.is_present(NAME));
  assert_eq!(expected.1, matches.get_count(NAME));
  assert_eq!(expected.2, matches.get_values(NAME));
}

fn fail<I, S>(config: (bool, Option<String>), input: I, expected: &str)
where
  I: IntoIterator<Item = S>,
  S: AsRef<str>,
{
  assert_eq!(expected, clar(config).resolve(input).unwrap_err().to_string());
}

#[test]
fn _0001() {
  pass((false, None), EMPTY_INPUT, (false, 0, EMPTY_VALUES));
}

#[test]
fn _0002() {
  pass((false, None), ["X"], (true, 1, vec![some!("X")]));
}

#[test]
fn _0003() {
  pass((false, some!("A")), EMPTY_INPUT, (false, 0, vec![some!("A")]));
}

#[test]
fn _0004() {
  pass((false, some!("A")), ["X"], (true, 1, vec![some!("X")]));
}

#[test]
fn _0005() {
  // Argument is not provided.
  fail((true, None), EMPTY_INPUT, "missing required argument: <FILE>");
  // Some other token is on input.
  fail((true, None), ["-h"], "missing required argument: <FILE>");
}

#[test]
fn _0006() {
  pass((true, None), ["X"], (true, 1, vec![some!("X")]));
}

#[test]
fn _0007() {
  // Argument is not provided.
  fail((true, some!("A")), EMPTY_INPUT, "missing required argument: <FILE>");
  // Some other token is on input.
  fail((true, some!("A")), ["-h"], "missing required argument: <FILE>");
}

#[test]
fn _0008() {
  pass((true, some!("A")), ["X"], (true, 1, vec![some!("X")]));
}
