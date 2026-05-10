use super::*;

#[test]
fn _0001() {
  // Resolver accepts one optional argument named 'file',
  // and exactly one argument is provided.
  let matches = Clar::new(APP)
    .arguments(vec![argument("file", None, false)])
    .resolve(["file.json"])
    .unwrap();
  assert!(matches.is_present("file"));
  assert_eq!(1, matches.get_count("file"));
  assert_eq!(vec![some!("file.json")], matches.get_values("file"));
}

#[test]
fn _0002() {
  // Resolver accepts one optional argument named 'file',
  // but an option appears in the command line.
  assert_eq!(
    "unexpected option '-h' found",
    Clar::new(APP)
      .arguments(vec![argument("file", None, false)])
      .resolve(["-h"])
      .unwrap_err()
      .to_string()
  );
}

#[test]
fn _0003() {
  // Resolver accepts one optional argument named 'file',
  // exactly one argument is provided, but checks are done
  // for argument named 'music'.
  let matches = Clar::new(APP)
    .arguments(vec![argument("file", None, false)])
    .resolve(["file.txt"])
    .unwrap();
  assert!(!matches.is_present("music"));
  assert_eq!(0, matches.get_count("music"));
  assert_eq!(EMPTY_VALUES, matches.get_values("music"));
}

#[test]
fn _0004() {
  // Resolver accepts one optional argument named 'file' with default value,
  // and no argument appears in the command line.
  let matches = Clar::new(APP)
    .arguments(vec![argument("file", some!("file.txt"), false)])
    .resolve(EMPTY_INPUT)
    .unwrap();
  assert_eq!(vec![Some("file.txt".to_string())], matches.get_values("file"));
}

#[test]
fn _0005() {
  // Argument does not appear in command line, and has no default value.
  let matches = Clar::new(APP)
    .arguments(vec![argument("file", None, false)])
    .resolve(EMPTY_INPUT)
    .unwrap();
  assert_eq!(EMPTY_VALUES, matches.get_values("file"));
}
