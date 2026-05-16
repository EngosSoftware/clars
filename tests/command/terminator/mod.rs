//! Command -> Terminator

use super::*;

#[test]
fn _0001() {
  // Given: Command -> Terminator definition.
  //  When: Empty input provided.
  //  Then: Resolving should pass, no command or terminator should be resolved.
  let matches = Clar::new(APP)
    .commands([ClarCommand::new("a").terminator(ClarTerminator::new("terminator"))])
    .resolve(EMPTY_INPUT)
    .unwrap();
  assert!(!matches.is_present("a"));
  assert!(!matches.is_present(["a", "terminator"]));
}
