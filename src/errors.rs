pub type Result<T, E = ClarsError> = std::result::Result<T, E>;

/// Error definition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClarsError {
  General(String),
  MissingRequiredArgument(String),
}

impl std::fmt::Display for ClarsError {
  /// Implementation of [Display](std::fmt::Display) trait for [ClarsError].
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::General(s) => write!(f, "{}", s),
      Self::MissingRequiredArgument(caption) => write!(f, "missing required argument: <{}>", caption),
    }
  }
}

impl ClarsError {
  /// Creates a new [ClarsError] with specified error message.
  pub fn new(message: impl AsRef<str>) -> Self {
    Self::General(message.as_ref().to_string())
  }
}

pub fn err_missing_required_argument(caption: impl AsRef<str>) -> ClarsError {
  ClarsError::MissingRequiredArgument(caption.as_ref().to_string())
}

macro_rules! err {
  ($($arg:tt)*) => {{
    Err(ClarsError::new(format!($($arg)*)))
  }};
}

pub(crate) use err;
