//! Command -> Options-Arguments-Terminator

use super::*;

#[test]
fn _0001() {
  // Given: Definition for Command -> Options-Arguments-Terminator.
  //  When: Empty input provided.
  //  Then: Resolving should pass, no command, no options, no arguments and no terminator should be resolved.
  let matches = Clar::new(APP)
    .commands([ClarCommand::new("a").options_arguments_terminator(
      [ClarOption::short("color", 'c')],
      [ClarArgument::new("file")],
      ClarTerminator::new("terminator"),
    )])
    .resolve(EMPTY_INPUT)
    .unwrap();
  assert!(!matches.is_present("a"));
  assert!(!matches.is_present(["a", "color"]));
  assert!(!matches.is_present(["a", "file"]));
  assert!(!matches.is_present(["a", "terminator"]));
}

#[test]
fn _0002() {
  // Given: Definition for Command -> Options-Arguments-Terminator.
  //  When: Input contains one command, one option, one argument and one terminator.
  //  Then: Resolving should pass, one command, one option, one argument and one terminator should be resolved.
  let matches = Clar::new(APP)
    .commands([ClarCommand::new("a").options_arguments_terminator(
      [ClarOption::short("color", 'c')],
      [ClarArgument::new("file")],
      ClarTerminator::new("terminator"),
    )])
    .resolve(["a", "-c", "file.txt", "--"])
    .unwrap();
  assert!(matches.is_present("a"));
  assert!(matches.is_present(["a", "color"]));
  assert!(matches.is_present(["a", "file"]));
  assert_eq!("file.txt", matches.get_first_value(["a", "file"]).unwrap());
  assert!(matches.is_present(["a", "terminator"]));
}
