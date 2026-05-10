//! # Two arguments

use super::*;

const NAME1: &str = "file1";
const NAME2: &str = "file2";

fn clar(config1: (bool, Option<String>), config2: (bool, Option<String>)) -> Clar {
  Clar::new(APP).arguments(vec![
    argument(NAME1, config1.1, config1.0),
    argument(NAME2, config2.1, config2.0),
  ])
}

fn pass<I, S>(
  config1: (bool, Option<String>),
  config2: (bool, Option<String>),
  input: I,
  expected1: (bool, usize, Vec<Option<String>>),
  expected2: (bool, usize, Vec<Option<String>>),
) where
  I: IntoIterator<Item = S>,
  S: AsRef<str>,
{
  let matches = clar(config1, config2).resolve(input).unwrap();
  assert_eq!(expected1.0, matches.is_present(NAME1));
  assert_eq!(expected1.1, matches.get_count(NAME1));
  assert_eq!(expected1.2, matches.get_values(NAME1));
  assert_eq!(expected2.0, matches.is_present(NAME2));
  assert_eq!(expected2.1, matches.get_count(NAME2));
  assert_eq!(expected2.2, matches.get_values(NAME2));
}

fn fail<I, S>(config1: (bool, Option<String>), config2: (bool, Option<String>), input: I, expected: &str)
where
  I: IntoIterator<Item = S>,
  S: AsRef<str>,
{
  assert_eq!(expected, clar(config1, config2).resolve(input).unwrap_err().to_string());
}

#[test]
fn _0001() {
  pass(
    (true, None),
    (true, None),
    ["X", "Y"],
    (true, 1, vec![some!("X")]),
    (true, 1, vec![some!("Y")]),
  );
}

#[test]
fn _0002() {
  fail((true, None), (true, None), ["X"], "missing required argument <FILE2>");
}
