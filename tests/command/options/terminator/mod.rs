//! Command -> Options-Terminator

use super::*;

#[test]
fn _0001() {
  // Given: Command -> Options-Terminator definition.
  //  When: Empty input provided.
  //  Then: Resolving should pass, no command, no option or terminator should be resolved.
  let matches = Clar::new(APP)
    .commands([ClarCommand::new("a").options_terminator([ClarOption::short("help", 'h')], ClarTerminator::new("terminator"))])
    .resolve(EMPTY_INPUT)
    .unwrap();
  assert!(!matches.is_present("a"));
  assert!(!matches.is_present(["a", "help"]));
  assert!(!matches.is_present(["a", "terminator"]));
}

#[test]
fn _0002() {
  // Given: Command -> Options-Terminator definition.
  //  When: Command with option and terminator appear on input.
  //  Then: Resolving should pass, one command, one option and one terminator should be resolved.
  let matches = Clar::new(APP)
    .commands([ClarCommand::new("a").options_terminator([ClarOption::short("help", 'h')], ClarTerminator::new("terminator"))])
    .resolve(["a", "-h", "--"])
    .unwrap();
  assert!(matches.is_present("a"));
  assert!(matches.is_present(["a", "help"]));
  assert!(matches.is_present(["a", "terminator"]));
}
