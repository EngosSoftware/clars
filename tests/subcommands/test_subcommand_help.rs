use super::*;

fn clar() -> Clar {
  Clar::new(APP).options_subcommands(
    vec![ClarOption::new("help", 'h', "help").standalone()],
    vec![
      ClarCommand::new("a").options(vec![ClarOption::new("help", 'h', "help").standalone()]),
      ClarCommand::new("b").options(vec![ClarOption::new("verbose", 'v', "verbose").standalone()]),
      ClarCommand::new("c"),
    ],
  )
}

#[test]
fn _0001() {
  assert!(clar().resolve(["-h"]).unwrap().is_present("help"));
}

#[test]
fn _0002() {
  assert_eq!(
    "option '-h' must be used alone",
    clar().resolve(["-h", "-V"]).unwrap_err().to_string()
  );
}

#[test]
fn _0003() {
  assert!(clar().resolve(["a"]).unwrap().is_present("a"));
}

#[test]
fn _0004() {
  assert!(clar().resolve(["a", "-h"]).unwrap().is_present("a"));
  assert!(clar().resolve(["a", "-h"]).unwrap().is_present("a/help"));
}

#[test]
fn _0005() {
  assert!(clar().resolve(["b", "-v"]).unwrap().is_present("b"));
  assert!(clar().resolve(["b", "--verbose"]).unwrap().is_present("b/verbose"));
}

#[test]
fn _0006() {
  assert_eq!(
    "option '-h' must be used alone",
    clar().resolve(["a", "-h", "-v"]).unwrap_err().to_string()
  );
}

#[test]
fn _0007() {
  assert_eq!(
    "option '--verbose' must be used alone",
    clar().resolve(["b", "--verbose", "-h"]).unwrap_err().to_string()
  );
}
