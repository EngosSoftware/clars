use clars::parser::{
  ArgumentProperties, Evaluator, OptionProperties, SubcommandProperties, argument, long_option, one, sequence,
  short_option, subcommand, zero_or_more, zero_or_one,
};
use clars::{Lexer, Token};
use std::collections::VecDeque;

const FALSE: Option<bool> = Some(false);
const TRUE: Option<bool> = Some(true);

fn assert_ok<I, S>(input: I, evaluator: &Evaluator, expected: Option<bool>, remaining_length: usize)
where
  I: IntoIterator<Item = S>,
  S: AsRef<str>,
{
  let mut tokens: VecDeque<Token> = Lexer::default().parse(input).unwrap().iter().cloned().collect();
  let mut values = vec![];
  assert_eq!(expected, evaluator(&mut tokens, &mut values).unwrap());
  assert_eq!(remaining_length, tokens.len());
}

fn assert_err<I, S>(input: I, evaluator: &Evaluator, expected: &str, remaining_length: usize)
where
  I: IntoIterator<Item = S>,
  S: AsRef<str>,
{
  let mut tokens: VecDeque<Token> = Lexer::default().parse(input).unwrap().iter().cloned().collect();
  let mut values = vec![];
  assert_eq!(expected, evaluator(&mut tokens, &mut values).unwrap_err().to_string());
  assert_eq!(remaining_length, tokens.len());
}

const EMPTY: Vec<String> = vec![];

fn option_properties(
  name: &str,
  standalone: bool,
  valued: bool,
  default_missing_value: Option<String>,
) -> OptionProperties {
  OptionProperties {
    name: name.to_string(),
    standalone,
    default_missing_value,
    takes_value: valued,
  }
}

fn subcommand_properties(name: &str) -> SubcommandProperties {
  SubcommandProperties {
    name: name.to_string(),
    value: None,
  }
}

fn argument_properties(name: &str, required: bool) -> ArgumentProperties {
  ArgumentProperties {
    name: name.to_string(),
    required,
  }
}

#[test]
fn _0001() {
  let evaluator = short_option('h', option_properties("help", false, false, None));
  assert_ok(["-h"], &evaluator, TRUE, 0);
}

#[test]
fn _0002() {
  let evaluator = short_option('h', option_properties("version", false, false, None));
  assert_ok(["-V"], &evaluator, FALSE, 1);
}

#[test]
fn _0003() {
  let evaluator = short_option('h', option_properties("help", true, false, None));
  assert_ok(["-h"], &evaluator, TRUE, 0);
}

#[test]
fn _0004() {
  let evaluator = short_option('h', option_properties("help", true, false, None));
  assert_err(["-h", "-V"], &evaluator, "option must be used alone", 1);
}

#[test]
fn _0005() {
  let evaluator = short_option('h', option_properties("help", true, false, None));
  assert_err(["-h=1"], &evaluator, "option must not have a value", 0);
}

#[test]
fn _0006() {
  let evaluator = long_option("help", option_properties("help", false, false, None));
  assert_ok(["--help"], &evaluator, TRUE, 0);
}

#[test]
fn _0007() {
  let evaluator = long_option("help", option_properties("version", false, false, None));
  assert_ok(["--version"], &evaluator, FALSE, 1);
}

#[test]
fn _0008() {
  let evaluator = long_option("help", option_properties("help", true, false, None));
  assert_ok(["--help"], &evaluator, TRUE, 0);
}

#[test]
fn _0009() {
  let evaluator = long_option("help", option_properties("help", true, false, None));
  assert_err(["--help", "--version"], &evaluator, "option must be used alone", 1);
}

#[test]
fn _0010() {
  let evaluator = long_option("help", option_properties("help", true, false, None));
  assert_err(["--help=1"], &evaluator, "option must not have a value", 0);
}

#[test]
fn _0011() {
  let evaluators = vec![
    short_option('h', option_properties("help", true, false, None)),
    long_option("help", option_properties("help", true, false, None)),
    short_option('V', option_properties("version", true, false, None)),
    long_option("version", option_properties("version", true, false, None)),
  ];
  let evaluator = zero_or_one(evaluators);
  assert_ok(["-h"], &evaluator, TRUE, 0);
  assert_ok(["--help"], &evaluator, TRUE, 0);
  assert_ok(["-V"], &evaluator, TRUE, 0);
  assert_ok(["--version"], &evaluator, TRUE, 0);
  assert_ok(["-c"], &evaluator, FALSE, 1);
  assert_ok(["color"], &evaluator, FALSE, 1);
  assert_err(["-h", "-c"], &evaluator, "option must be used alone", 1);
  assert_err(["--help", "-c"], &evaluator, "option must be used alone", 1);
  assert_err(["-V", "-c"], &evaluator, "option must be used alone", 1);
}

#[test]
fn _0012() {
  let evaluators = vec![
    short_option('s', option_properties("style", false, false, None)),
    long_option("style", option_properties("style", false, false, None)),
    short_option('l', option_properties("label", false, false, None)),
    long_option("label", option_properties("label", false, false, None)),
    short_option('r', option_properties("separator", false, false, None)),
    long_option("separator", option_properties("separator", false, false, None)),
    short_option('n', option_properties("no-percent-sign", false, false, None)),
    long_option(
      "no-percent-sign",
      option_properties("no-percent-sign", false, false, None),
    ),
    short_option('c', option_properties("collapse", false, false, None)),
    long_option("collapse", option_properties("collapse", false, false, None)),
    short_option('t', option_properties("tag", false, false, None)),
    long_option("tag", option_properties("tag", false, false, None)),
    short_option('f', option_properties("file", false, false, None)),
    long_option("file", option_properties("file", false, false, None)),
  ];
  let evaluator = zero_or_more(evaluators);
  assert_ok(["-h"], &evaluator, FALSE, 1);
  assert_ok(["--help"], &evaluator, FALSE, 1);
  assert_ok(["-s"], &evaluator, TRUE, 0);
  assert_ok(["-s", "-l", "-r", "--no-percent-sign"], &evaluator, TRUE, 0);
}

#[test]
fn _0013() {
  let evaluators = vec![
    subcommand(subcommand_properties("srv")),
    subcommand(subcommand_properties("edm")),
    subcommand(subcommand_properties("edt")),
    subcommand(subcommand_properties("efe")),
    subcommand(subcommand_properties("pdm")),
    subcommand(subcommand_properties("pdt")),
    subcommand(subcommand_properties("pfe")),
    subcommand(subcommand_properties("tdm")),
    subcommand(subcommand_properties("tdt")),
    subcommand(subcommand_properties("tfe")),
    subcommand(subcommand_properties("xdm")),
    subcommand(subcommand_properties("xdt")),
    subcommand(subcommand_properties("xfe")),
    subcommand(subcommand_properties("exs")),
  ];
  let evaluator = one(evaluators);
  assert_ok(["srv"], &evaluator, TRUE, 0);
  assert_ok(["edm"], &evaluator, TRUE, 0);
  assert_ok(["edt"], &evaluator, TRUE, 0);
  assert_ok(["efe"], &evaluator, TRUE, 0);
  assert_err(["alfa"], &evaluator, "no match", 1);
}

#[test]
fn _0014() {
  let options = vec![
    short_option('s', option_properties("style", false, false, None)),
    long_option("style", option_properties("style", false, false, None)),
    short_option('l', option_properties("label", false, false, None)),
    long_option("label", option_properties("label", false, false, None)),
    short_option('r', option_properties("separator", false, false, None)),
    long_option("separator", option_properties("separator", false, false, None)),
    short_option('n', option_properties("no-percent-sign", false, false, None)),
    long_option(
      "no-percent-sign",
      option_properties("no-percent-sign", false, false, None),
    ),
    short_option('c', option_properties("collapse", false, false, None)),
    long_option("collapse", option_properties("collapse", false, false, None)),
    short_option('t', option_properties("tag", false, false, None)),
    long_option("tag", option_properties("tag", false, false, None)),
    short_option('f', option_properties("file", false, false, None)),
    long_option("file", option_properties("file", false, false, None)),
  ];
  let zero_or_more_options = zero_or_more(options);
  let zero_or_one_argument = zero_or_one(vec![argument(argument_properties("FILE", false))]);
  let evaluator = sequence(vec![zero_or_more_options, zero_or_one_argument]);
  assert_ok(EMPTY, &evaluator, None, 0);
  assert_ok(["-s"], &evaluator, None, 0);
  assert_ok(["-s", "report.json"], &evaluator, None, 0);
  assert_ok(["-s", "--tag", "report.json"], &evaluator, None, 0);
  assert_ok(["report.json"], &evaluator, None, 0);
}

#[test]
fn _0015() {
  let evaluator = short_option('c', option_properties("color", false, true, None));
  assert_ok(["-c=always"], &evaluator, TRUE, 0);
}

#[test]
fn _0016() {
  let evaluator = short_option('c', option_properties("color", false, true, None));
  assert_ok(["-c", "always"], &evaluator, TRUE, 0);
}

#[test]
fn _0017() {
  let evaluator = short_option('c', option_properties("color", false, true, Some("always".to_string())));
  assert_ok(["-c"], &evaluator, TRUE, 0);
}

#[test]
fn _0018() {
  let evaluator = short_option('c', option_properties("color", false, true, None));
  assert_err(["-c"], &evaluator, "option must have a value", 0);
}

#[test]
fn _0019() {
  let evaluator = long_option("color", option_properties("color", false, true, None));
  assert_ok(["--color=always"], &evaluator, TRUE, 0);
}

#[test]
fn _0020() {
  let evaluator = long_option("color", option_properties("color", false, true, None));
  assert_ok(["--color", "always"], &evaluator, TRUE, 0);
}

#[test]
fn _0021() {
  let evaluator = long_option(
    "color",
    option_properties("color", false, true, Some("always".to_string())),
  );
  assert_ok(["--color"], &evaluator, TRUE, 0);
}

#[test]
fn _0022() {
  let evaluator = long_option("color", option_properties("color", false, true, None));
  assert_err(["--color"], &evaluator, "option must have a value", 0);
}
