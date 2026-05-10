use super::*;

#[test]
fn optional_terminator_help_should_work() {
  assert_eq!(
    "Usage: clars [-- [ARG]...]\n",
    Clar::new(APP)
      .terminator(ClarTerminator::new("term"))
      .resolve(EMPTY_INPUT)
      .unwrap()
      .get_help()
      .chars()
      .collect::<String>()
  )
}

#[test]
fn required_terminator_help_should_work() {
  assert_eq!(
    "Usage: clars -- [ARG]...\n",
    Clar::new(APP)
      .terminator(ClarTerminator::new("term").required())
      .resolve(["--"])
      .unwrap()
      .get_help()
      .chars()
      .collect::<String>()
  )
}
