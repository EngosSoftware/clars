use super::*;

#[test]
fn _0001() {
  // Option terminator is not required and not provided in command line.
  let matches = Clar::new(APP).terminator(ClarTerminator::new("term")).resolve(EMPTY_INPUT).unwrap();
  assert_eq!(EMPTY_VALUES, matches.get_values("term"));
  assert_eq!(0, matches.get_count("term"));
  assert!(!matches.is_present("term"));
}

#[test]
fn _0002() {
  // Option terminator is not required and provided on command line without following arguments.
  let matches = Clar::new(APP).terminator(ClarTerminator::new("term")).resolve(["--"]).unwrap();
  assert_eq!(EMPTY_VALUES, matches.get_values("term"));
  assert_eq!(1, matches.get_count("term"));
  assert!(matches.is_present("term"));
}

#[test]
fn _0003() {
  // Option terminator is not required and provided on command line with one argument.
  assert_eq!(
    vec![some!("A")],
    Clar::new(APP)
      .terminator(ClarTerminator::new("term"))
      .resolve(["--", "A"])
      .unwrap()
      .get_values("term")
  );
}

#[test]
fn _0004() {
  // Option terminator is not required and provided on command line with multiple arguments.
  assert_eq!(
    vec![some!("A"), some!("B"), some!("C")],
    Clar::new(APP)
      .terminator(ClarTerminator::new("term"))
      .resolve(["--", "A", "B", "C"])
      .unwrap()
      .get_values("term")
  );
}

#[test]
fn _0005() {
  // Required option terminator is not provided.
  assert_eq!(
    "missing required option terminator '--'",
    Clar::new(APP)
      .terminator(ClarTerminator::new("term").required())
      .resolve(EMPTY_INPUT)
      .unwrap_err()
      .to_string()
  );
}

#[test]
fn _0006() {
  // Required option terminator is provided without arguments.
  assert_eq!(
    EMPTY_VALUES,
    Clar::new(APP)
      .terminator(ClarTerminator::new("term").required())
      .resolve(["--"])
      .unwrap()
      .get_values("term")
  );
}

#[test]
fn _0007() {
  // Required option terminator is provided with one argument.
  assert_eq!(
    vec![some!("A")],
    Clar::new(APP)
      .terminator(ClarTerminator::new("term").required())
      .resolve(["--", "A"])
      .unwrap()
      .get_values("term")
  );
}

#[test]
fn _0008() {
  // Required option terminator is provided with multiple arguments.
  assert_eq!(
    vec![some!("A"), some!("B")],
    Clar::new(APP)
      .terminator(ClarTerminator::new("term").required())
      .resolve(["--", "A", "B"])
      .unwrap()
      .get_values("term")
  );
}

#[test]
fn _0009() {
  // Given: A resolver that accepts only option terminator. Option terminator is optional.
  //  When: Instead of option terminator an option or argument appears on input.
  //  Then: Report an error.
  let terminator = ClarTerminator::new("term");
  assert_eq!(
    "unexpected option '-h' found",
    Clar::new(APP).terminator(terminator.clone()).resolve(["-h"]).unwrap_err().to_string()
  );
  assert_eq!(
    "unexpected option '--help' found",
    Clar::new(APP).terminator(terminator.clone()).resolve(["--help"]).unwrap_err().to_string()
  );
  assert_eq!(
    "unexpected argument 'help' found",
    Clar::new(APP).terminator(terminator).resolve(["help"]).unwrap_err().to_string()
  );
}

#[test]
fn _0010() {
  // Given: A resolver that accepts only option terminator.
  //        Option terminator is required and must appear on input.
  //  When: Instead of option terminator an option or argument appears on input.
  //  Then: Report an error.
  let terminator = ClarTerminator::new("term").required();
  assert_eq!(
    "unexpected option '-h' found",
    Clar::new(APP).terminator(terminator.clone()).resolve(["-h"]).unwrap_err().to_string()
  );
  assert_eq!(
    "unexpected option '--help' found",
    Clar::new(APP).terminator(terminator.clone()).resolve(["--help"]).unwrap_err().to_string()
  );
  assert_eq!(
    "unexpected argument 'help' found",
    Clar::new(APP).terminator(terminator).resolve(["help"]).unwrap_err().to_string()
  );
}
