use super::*;

#[test]
fn _0001() {
  // Optional terminator.
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
fn _0002() {
  // Required terminator.
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
