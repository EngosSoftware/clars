mod display;
mod errors;
mod lexer;
mod matches;
pub mod model;
pub mod parser;
mod resolver;

pub use errors::{ClarsError, Result};
pub use lexer::{Lexer, Token};
pub use matches::ClarMatches;
pub use model::CliArgument;
pub use resolver::Clar;
use std::path::Path;

pub fn get_args() -> (String, Vec<String>) {
  let mut args = std::env::args();
  let name = Path::new(&args.next().expect("expected at least one argument"))
    .file_name()
    .expect("expected file name")
    .display()
    .to_string();
  (name, args.collect())
}

#[test]
fn _0001() {
  assert!(get_args().0.starts_with("clars"));
}
