use super::*;

#[test]
fn _0001() {
  // Given: Arguments and option terminator definition.
  //  When: Empty input provided.
  //  Then: Resolving should pass, no argument or terminator should be resolved.
  let matches = Clar::new(APP)
    .arguments_terminator([ClarArgument::new("file")], ClarTerminator::new("terminator"))
    .resolve(EMPTY_INPUT)
    .unwrap();
  assert!(!matches.is_present("file"));
  assert!(!matches.is_present("terminator"));
}

#[test]
fn _0002() {
  // Given: Arguments and option terminator definition.
  //  When: One argument followed by terminator is iven on input.
  //  Then: Resolving should pass, one argument and terminator should be resolved.
  let matches = Clar::new(APP)
    .arguments_terminator([ClarArgument::new("file")], ClarTerminator::new("terminator"))
    .resolve(["file.txt", "--"])
    .unwrap();
  assert!(matches.is_present("file"));
  assert_eq!("file.txt", matches.get_first_value("file").unwrap());
  assert!(matches.is_present("terminator"));
}
