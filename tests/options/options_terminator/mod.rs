use super::*;

#[test]
fn _0001() {
  // Given: Option with optional terminator.
  //  When: Empty input provided.
  //  Then: Resolving should pass, no option or terminator should be resolved.
  let matches = Clar::new(APP)
    .options_terminator([ClarOption::new("color", 'c', "color").short_label('c')], ClarTerminator::new("terminator"))
    .resolve(EMPTY_INPUT)
    .unwrap();
  assert!(!matches.is_present("color"));
  assert!(!matches.is_present("terminator"));
}

#[test]
fn _0002() {
  // Given: Option with optional terminator.
  //  When: Input contains option and terminator.
  //  Then: Resolving should pass, option and terminator should be resolved properly.
  let matches = Clar::new(APP)
    .options_terminator([ClarOption::new("color", 'c', "color").short_label('c')], ClarTerminator::new("terminator"))
    .resolve(["-c", "--"])
    .unwrap();
  assert!(matches.is_present("color"));
  assert!(matches.is_present("terminator"));
}
