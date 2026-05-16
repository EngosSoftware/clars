//! Empty CLI definition

use super::*;

#[test]
fn _0001() {
  // Given: Resolver without any definition.
  //  When: Retrieved the help message.
  //  Then: The expected message should contain only usage.
  eq_text(
    r#"
    Usage: clars
  "#,
    Clar::new(APP).resolve(EMPTY_INPUT).unwrap().get_help(),
  );
}

#[test]
fn _0002() {
  // Given: Resolver without any definition but with application description.
  //  When: Retrieved the help message.
  //  Then: The expected message should contain description and usage.
  eq_text(
    r#"
    Command line arguments resolver

    Usage: clars
  "#,
    Clar::new(APP)
      .description("Command line arguments resolver")
      .resolve(EMPTY_INPUT)
      .unwrap()
      .get_help(),
  );
}

#[test]
fn _0003() {
  // No CLI definition but a short option given as an input.
  assert_eq!("unexpected option '-h' found", Clar::new(APP).resolve(["-h"]).unwrap_err().to_string());
}

#[test]
fn _0004() {
  // No CLI definition but a long option given as an input.
  assert_eq!("unexpected option '--help' found", Clar::new(APP).resolve(["--help"]).unwrap_err().to_string());
}

#[test]
fn _0005() {
  // No CLI definition but an argument given as an input.
  assert_eq!(
    "unexpected argument 'file.txt' found",
    Clar::new(APP).resolve(["file.txt"]).unwrap_err().to_string()
  );
}

#[test]
fn _0006() {
  // No CLI definition but an option terminator given as an input.
  assert_eq!(
    "unexpected option terminator '--' found",
    Clar::new(APP).resolve(["--"]).unwrap_err().to_string()
  );
}
