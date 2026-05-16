//! Command -> Options-Arguments

mod terminator;

use super::*;

#[test]
fn _0001() {
  // Given: Definition for Command -> Options-Arguments.
  //  When: Empty input provided.
  //  Then: Resolving should pass, no command, no options and no arguments should be resolved.
  let matches = Clar::new(APP)
    .commands([ClarCommand::new("a").options_arguments([ClarOption::short("color", 'c')], [ClarArgument::new("file")])])
    .resolve(EMPTY_INPUT)
    .unwrap();
  assert!(!matches.is_present("a"));
  assert!(!matches.is_present(["a", "color"]));
  assert!(!matches.is_present(["a", "file"]));
}

#[test]
fn _0002() {
  // Given: Definition for Command -> Options-Arguments.
  //  When: Input contains one command with one option and one argument.
  //  Then: Resolving should pass, one command, one option and one argument should be resolved.
  let matches = Clar::new(APP)
    .commands([ClarCommand::new("a").options_arguments([ClarOption::short("color", 'c')], [ClarArgument::new("file")])])
    .resolve(["a", "-c", "file.txt"])
    .unwrap();
  assert!(matches.is_present("a"));
  assert!(matches.is_present(["a", "color"]));
  assert!(matches.is_present(["a", "file"]));
  assert_eq!("file.txt", matches.get_first_value(["a", "file"]).unwrap());
}
