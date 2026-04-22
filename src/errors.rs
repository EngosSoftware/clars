pub type Result<T, E = ClarsError> = std::result::Result<T, E>;

/// Error definition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClarsError(String);

impl std::fmt::Display for ClarsError {
  /// Implementation of [Display](std::fmt::Display) trait for [ClarsError].
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl ClarsError {
  /// Creates a new [ClarsError] with specified error message.
  pub fn new(message: impl AsRef<str>) -> Self {
    Self(message.as_ref().to_string())
  }
}
