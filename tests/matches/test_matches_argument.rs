use super::*;

#[test]
fn _0001() {
  let mut clar = Clar::new("clars");
  clar.add_arguments(vec![argument("FILE", None, false)]);

  // Argument appears in command line.
  let matches = clar.clone().resolve(["file.json"]).unwrap();
  assert!(matches.is_present("FILE"));
  assert_eq!(1, matches.get_count("FILE"));
  assert_eq!(vec![some!("file.json")], matches.get_values("FILE"));

  // Argument does not appear in command line.
  let matches = clar.clone().resolve(["-h"]).unwrap();
  assert!(!matches.is_present("help"));
  assert_eq!(0, matches.get_count("help"));
  assert_eq!(EMPTY_VALUES, matches.get_values("help"));

  // Argument is not configured in command line.
  let matches = clar.clone().resolve(["music"]).unwrap();
  assert!(!matches.is_present("music"));
  assert_eq!(0, matches.get_count("music"));
  assert_eq!(EMPTY_VALUES, matches.get_values("music"));
}

#[test]
fn _0002() {
  let mut clar = Clar::new("clars");
  clar.add_arguments(vec![argument("FILE", some!("file.txt"), false)]);
  let matches = clar.resolve(["-c"]).unwrap();

  // Argument does not appear in command line, but has a default value.
  assert_eq!(vec![Some("file.txt".to_string())], matches.get_values("FILE"));
}
