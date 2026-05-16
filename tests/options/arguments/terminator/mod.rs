//! # Options-Arguments-Terminator

use super::*;

#[test]
fn _0001() {
  // Given: Options-Arguments-Terminator definition.
  //  When: Empty input provided.
  //  Then: Resolving should pass, no option, no argument and no terminator should be resolved.
  let matches = Clar::new(APP)
    .options_arguments_terminator(
      [ClarOption::new("color", 'c', "color").short_label('c')],
      [ClarArgument::new("file")],
      ClarTerminator::new("terminator"),
    )
    .resolve(EMPTY_INPUT)
    .unwrap();
  assert!(!matches.is_present("color"));
  assert!(!matches.is_present("file"));
  assert!(!matches.is_present("terminator"));
}

#[test]
fn _0002() {
  // Given: Options-Arguments-Terminator definition.
  //  When: One option, one argument and one trminator provided on input.
  //  Then: Resolving should pass, one option, one argument and one terminator should be resolved.
  let matches = Clar::new(APP)
    .options_arguments_terminator(
      [ClarOption::new("color", 'c', "color").short_label('c')],
      [ClarArgument::new("file")],
      ClarTerminator::new("terminator"),
    )
    .resolve(["-c", "file.txt", "--"])
    .unwrap();
  assert!(matches.is_present("color"));
  assert!(matches.is_present("file"));
  assert_eq!("file.txt", matches.get_first_value("file").unwrap());
  assert!(matches.is_present("terminator"));
}
