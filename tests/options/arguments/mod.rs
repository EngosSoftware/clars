mod terminator;

use super::*;

#[test]
fn _0001() {
  // Given: Options and arguments definition.
  //  When: Empty input provided.
  //  Then: Resolving should pass, no option or argument should be resolved.
  let matches = Clar::new(APP)
    .options_arguments([ClarOption::new("color", 'c', "color").short_label('c')], [ClarArgument::new("file")])
    .resolve(EMPTY_INPUT)
    .unwrap();
  assert!(!matches.is_present("color"));
  assert!(!matches.is_present("file"));
}

#[test]
fn _0002() {
  // Given: Options and arguments definition.
  //  When: One option and one argument provided on input.
  //  Then: Resolving should pass, one option and one argument should be resolved.
  let matches = Clar::new(APP)
    .options_arguments([ClarOption::new("color", 'c', "color").short_label('c')], [ClarArgument::new("file")])
    .resolve(["-c", "file.txt"])
    .unwrap();
  assert!(matches.is_present("color"));
  assert!(matches.is_present("file"));
  assert_eq!("file.txt", matches.get_first_value("file").unwrap());
}
