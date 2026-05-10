#![doc = include_str!("../docs/README.md")]

mod errors;
mod evaluator;
mod helpers;
mod lexer;
mod matches;
mod messages;
mod model;
mod resolver;

pub use antex;
pub use errors::{ClarDiagnostic, ClarDiagnosticResult, ClarError, ClarResult};
pub use evaluator::Value;
pub use helpers::{get_app_and_args, get_first_value, get_more_info_hint};
pub use matches::ClarMatches;
pub use model::{ClarArgument, ClarCommand, ClarOption, ClarTerminator, display_tree};
pub use resolver::Clar;
