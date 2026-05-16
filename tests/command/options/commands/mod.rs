//! Command -> Options-Commands

use super::*;

#[test]
fn _0001() {
  // Given: Definition for Command -> Options-Commands.
  //  When: Empty input provided.
  //  Then: Resolving should pass, no command, no options and no subcommand should be resolved.
  let matches = Clar::new(APP)
    .commands([ClarCommand::new("a").options_commands([ClarOption::short("color", 'c')], [ClarCommand::new("b1"), ClarCommand::new("b2")])])
    .resolve(EMPTY_INPUT)
    .unwrap();
  assert!(!matches.is_present("a"));
  assert!(!matches.is_present(["a", "color"]));
  assert!(!matches.is_present(["a", "b1"]));
  assert!(!matches.is_present(["a", "b2"]));
}

#[test]
fn _0002() {
  // Given: Definition for Command -> Options-Commands.
  //  When: Input contains one command with one option and one subcommand.
  //  Then: Resolving should pass, one command, one option and one subcommand should be resolved.
  let matches = Clar::new(APP)
    .commands([ClarCommand::new("a").options_commands([ClarOption::short("color", 'c')], [ClarCommand::new("b1"), ClarCommand::new("b2")])])
    .resolve(["a", "-c", "b1"])
    .unwrap();
  assert!(!matches.is_present("a"));
  assert!(matches.is_present(["a", "color"]));
  assert!(matches.is_present(["a", "b1"]));
  assert!(!matches.is_present(["a", "b2"]));
}
