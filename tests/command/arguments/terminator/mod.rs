//! Command -> Arguments-Terminator

use super::*;

#[test]
fn _0001() {
  // Given: Definition for Command -> Arguments-Terminator.
  //  When: Empty input provided.
  //  Then: Resolving should pass, no command, no arguments and no terminator should be resolved.
  let matches = Clar::new(APP)
    .commands([ClarCommand::new("a").arguments_terminator([ClarArgument::new("file")], ClarTerminator::new("terminator"))])
    .resolve(EMPTY_INPUT)
    .unwrap();
  assert!(!matches.is_present("a"));
  assert!(!matches.is_present(["a", "file"]));
  assert!(!matches.is_present(["a", "terminator"]));
}

#[test]
fn _0002() {
  // Given: Definition for Command -> Arguments-Terminator.
  //  When: Input contains one command, one argument and one terminator.
  //  Then: Resolving should pass, one command, one argument and one terminator should be resolved.
  let matches = Clar::new(APP)
    .commands([ClarCommand::new("a").arguments_terminator([ClarArgument::new("file")], ClarTerminator::new("terminator"))])
    .resolve(["a", "file.txt", "--"])
    .unwrap();
  assert!(matches.is_present("a"));
  assert!(matches.is_present(["a", "file"]));
  assert_eq!("file.txt", matches.get_first_value(["a", "file"]).unwrap());
  assert!(matches.is_present(["a", "terminator"]));
}
