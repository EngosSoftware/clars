//! # Single argument

use super::*;

const NAME: &str = "file";

fn clar(config: (bool, Option<String>)) -> Clar {
  let mut argument = ClarArgument::new(NAME).help("File short").help_long("File long");
  if config.0 {
    argument = argument.required();
  }
  if let Some(value) = config.1 {
    argument = argument.default_value(value);
  }
  Clar::new(APP).arguments(vec![argument])
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
  fail((true, None), EMPTY_INPUT, "missing required argument <FILE>");
}

#[test]
fn _0006() {
  pass((true, None), ["X"], (true, 1, vec![some!("X")]));
}

#[test]
fn _0007() {
  pass((true, some!("A")), EMPTY_INPUT, (false, 0, vec![some!("A")]));
}

#[test]
fn _0008() {
  pass((true, some!("A")), ["X"], (true, 1, vec![some!("X")]));
}

#[test]
fn _0009() {
  let expected = "unexpected argument 'Y' found";
  fail((false, None), ["X", "Y"], expected);
  fail((false, some!("A")), ["X", "Y"], expected);
  fail((true, None), ["X", "Y"], expected);
  fail((true, some!("A")), ["X", "Y"], expected);
}

#[test]
fn _0010() {
  let expected = "unexpected option '-h' found";
  fail((false, None), ["-h"], expected);
  fail((false, some!("A")), ["-h"], expected);
  fail((true, None), ["-h"], expected);
  fail((true, some!("A")), ["-h"], expected);
}

#[test]
fn _0011() {
  let expected = "unexpected option '--help' found";
  fail((false, None), ["--help"], expected);
  fail((false, some!("A")), ["--help"], expected);
  fail((true, None), ["--help"], expected);
  fail((true, some!("A")), ["--help"], expected);
}

#[test]
fn _0012() {
  let expected = "unexpected option terminator '--' found";
  fail((false, None), ["--"], expected);
  fail((false, some!("A")), ["--"], expected);
  fail((true, None), ["--"], expected);
  fail((true, some!("A")), ["--"], expected);
}

#[test]
fn _0013() {
  let expected = "unexpected option terminator '--' found";
  fail((false, None), ["--", "a"], expected);
  fail((false, some!("A")), ["--", "a"], expected);
  fail((true, None), ["--", "a"], expected);
  fail((true, some!("A")), ["--", "a"], expected);
}

#[test]
fn getting_help_should_work() {
  assert_eq!(
    r#"
Usage: clars  [ARGS]

Arguments:
  [FILE]  File short
"#
    .trim_start(),
    clar((false, None))
      .resolve(EMPTY_INPUT)
      .unwrap()
      .get_help()
      .chars()
      .collect::<String>()
  )
}

#[test]
fn hyphen_is_a_valid_argument() {
  // Single hyphen is a valid argument, usually used to indicate
  // that input should be read from stdin.
  pass((false, None), ["-"], (true, 1, vec![some!("-")]));
  pass((false, some!("A")), ["-"], (true, 1, vec![some!("-")]));
  pass((true, None), ["-"], (true, 1, vec![some!("-")]));
  pass((true, some!("A")), ["-"], (true, 1, vec![some!("-")]));
}
