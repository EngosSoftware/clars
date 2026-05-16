//! Command -> Arguments

mod terminator;

use super::*;

#[test]
fn _0001() {
  // Given: Definition for Command -> Arguments.
  //  When: Empty input provided.
  //  Then: Resolving should pass, no command or arguments should be resolved.
  let matches = Clar::new(APP)
    .commands([ClarCommand::new("a").arguments([ClarArgument::new("file")])])
    .resolve(EMPTY_INPUT)
    .unwrap();
  assert!(!matches.is_present("a"));
  assert!(!matches.is_present(["a", "file"]));
}

#[test]
fn _0002() {
  // Given: Definition for Command -> Arguments.
  //  When: Input contains one command and one argument.
  //  Then: Resolving should pass, one command and one argument should be resolved.
  let matches = Clar::new(APP)
    .commands([ClarCommand::new("a").arguments([ClarArgument::new("file")])])
    .resolve(["a", "file.txt"])
    .unwrap();
  assert!(matches.is_present("a"));
  assert!(matches.is_present(["a", "file"]));
  assert_eq!("file.txt", matches.get_first_value(["a", "file"]).unwrap());
}
