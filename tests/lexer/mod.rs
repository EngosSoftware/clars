//! Edge cases for lexer

use crate::APP;
use clars::Clar;

#[test]
fn _0001() {
  // Short option label is an invalid character.
  assert_eq!(
    "short option must be a letter or digit, but '-$' found",
    Clar::new(APP).resolve(["-$"]).unwrap_err().to_string()
  );
}

#[test]
fn _0002() {
  // Long option label starts with invalid character.
  assert_eq!(
    "long option must start with a letter, but '--$' found",
    Clar::new(APP).resolve(["--$"]).unwrap_err().to_string()
  );
}

#[test]
fn _0003() {
  // Long option label starts with digit.
  assert_eq!(
    "long option must start with a letter, but '--0' found",
    Clar::new(APP).resolve(["--0"]).unwrap_err().to_string()
  );
}

#[test]
fn _0004() {
  // Long option label contains an invalid character.
  assert_eq!(
    "long option must contain letters, digits or hyphens but '--h$lp' found",
    Clar::new(APP).resolve(["--h$lp"]).unwrap_err().to_string()
  );
}
