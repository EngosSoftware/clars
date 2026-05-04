use crate::model::CliItem;
use crate::parser::Value;
use crate::{Clar, ClarsError, Token};
use std::collections::VecDeque;

pub fn display_error_message(
  reason: &ClarsError,
  resolver: &Clar,
  _tokens: &VecDeque<Token>,
  _values: &[Value],
  _items: &[CliItem],
) {
  match reason {
    ClarsError::General(_) => {}
    ClarsError::MissingRequiredArgument(caption) => {
      println!("error: missing required argument: <{}>\n", caption);
      println!("Usage: {} ???\n", resolver.app());
    }
  }
}
