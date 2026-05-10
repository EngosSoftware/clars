use antex::{Color, ColorMode, StyledText, Text};
use std::fmt;
use std::fmt::Display;
use std::ops::Deref;

/// Type alias for command line resolver result.
pub type ClarResult<T, E = ClarError> = Result<T, E>;

/// Type alias for command line resolver result with colored text.
pub type ClarDiagnosticResult<T, E = ClarDiagnostic> = Result<T, E>;

/// Command line resolver diagnostic data.
#[derive(Debug)]
pub struct ClarDiagnostic(ClarError, Text);

impl Display for ClarDiagnostic {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0.as_text(ColorMode::Off))
  }
}

impl Deref for ClarDiagnostic {
  type Target = ClarError;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl ClarDiagnostic {
  pub fn new(error: ClarError, text: Text) -> Self {
    Self(error, text)
  }

  pub fn text(&self) -> &Text {
    &self.1
  }
}

/// Command line resolver errors enumeration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClarError {
  MissingRequiredArgument(String, String),
  MissingRequiredOptionTerminator(String),
  UnexpectedShortOption(String, char),
  UnexpectedLongOption(String, String),
  UnexpectedArgument(String, String),
  UnexpectedOptionTerminator(String),
  ShortOptionMustBeUsedAlone(String, char),
  LongOptionMustBeUsedAlone(String, String),
  ShortOptionDoesNotAcceptValue(String, char),
  LongOptionDoesNotAcceptValue(String, String),
  ShortOptionRequiresValue(String, char, String),
  LongOptionRequiresValue(String, String, String),
  ShortOptionMustBeLetterOrDigit(String, char),
  LongOptionMustStartWithLetter(String, String),
  LongOptionMustContainLettersDigitsHyphens(String, String),
}

impl Display for ClarError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.as_text(ColorMode::Off))
  }
}

impl ClarError {
  /// Returns a colored error message.
  pub fn as_text(&self, cm: ColorMode) -> Text {
    const COLOR_QUOTED: Color = Color::Yellow;
    const COLOR_BRACKETS: Color = Color::Cyan;
    match self {
      ClarError::MissingRequiredArgument(_, caption) => Text::new(cm)
        .s("missing required argument ")
        .color(COLOR_BRACKETS)
        .bold()
        .s('<')
        .s(caption)
        .s('>')
        .reset(),
      ClarError::MissingRequiredOptionTerminator(_) => Text::new(cm)
        .s("missing required option terminator '")
        .color(COLOR_QUOTED)
        .bold()
        .s("--")
        .reset()
        .s("'"),
      ClarError::UnexpectedShortOption(_, label) => Text::new(cm)
        .s("unexpected option '")
        .color(COLOR_QUOTED)
        .bold()
        .s('-')
        .s(label)
        .reset()
        .s("' found"),
      ClarError::UnexpectedLongOption(_, label) => Text::new(cm)
        .s("unexpected option '")
        .color(COLOR_QUOTED)
        .bold()
        .s("--")
        .s(label)
        .reset()
        .s("' found"),
      ClarError::UnexpectedArgument(_, value) => Text::new(cm)
        .s("unexpected argument '")
        .color(COLOR_QUOTED)
        .bold()
        .s(value)
        .reset()
        .s("' found"),
      ClarError::UnexpectedOptionTerminator(_) => Text::new(cm)
        .s("unexpected option terminator '")
        .color(COLOR_QUOTED)
        .bold()
        .s("--")
        .reset()
        .s("' found"),
      ClarError::ShortOptionMustBeUsedAlone(_, label) => Text::new(cm)
        .s("option '")
        .color(COLOR_QUOTED)
        .bold()
        .s('-')
        .s(label)
        .reset()
        .s("' must be used alone"),
      ClarError::LongOptionMustBeUsedAlone(_, label) => Text::new(cm)
        .s("option '")
        .color(COLOR_QUOTED)
        .bold()
        .s("--")
        .s(label)
        .reset()
        .s("' must be used alone"),
      ClarError::ShortOptionDoesNotAcceptValue(_, label) => Text::new(cm)
        .s("option '")
        .color(COLOR_QUOTED)
        .bold()
        .s('-')
        .s(label)
        .reset()
        .s("' does not accept a value"),
      ClarError::LongOptionDoesNotAcceptValue(_, label) => Text::new(cm)
        .s("option '")
        .color(COLOR_QUOTED)
        .bold()
        .s("--")
        .s(label)
        .reset()
        .s("' does not accept a value"),
      ClarError::ShortOptionRequiresValue(_, label, caption) => Text::new(cm)
        .s("a value is required for '")
        .color(COLOR_QUOTED)
        .bold()
        .s("-")
        .s(label)
        .s(" <")
        .s(caption)
        .s(">")
        .reset()
        .s("' but none was supplied"),
      ClarError::LongOptionRequiresValue(_, label, caption) => Text::new(cm)
        .s("a value is required for '")
        .color(COLOR_QUOTED)
        .bold()
        .s("--")
        .s(label)
        .s(" <")
        .s(caption)
        .s(">")
        .reset()
        .s("' but none was supplied"),
      ClarError::ShortOptionMustBeLetterOrDigit(_, label) => Text::new(cm)
        .s("short option must be a letter or digit, but '")
        .color(COLOR_QUOTED)
        .bold()
        .s('-')
        .s(label)
        .reset()
        .s("' found"),
      ClarError::LongOptionMustStartWithLetter(_, label) => Text::new(cm)
        .s("long option must start with a letter, but '")
        .color(COLOR_QUOTED)
        .bold()
        .s("--")
        .s(label)
        .reset()
        .s("' found"),
      ClarError::LongOptionMustContainLettersDigitsHyphens(_, label) => Text::new(cm)
        .s("long option must contain letters, digits or hyphens but '")
        .color(COLOR_QUOTED)
        .bold()
        .s("--")
        .s(label)
        .reset()
        .s("' found"),
    }
  }

  pub fn source(&self) -> &str {
    match self {
      ClarError::MissingRequiredArgument(source, _) => source,
      ClarError::MissingRequiredOptionTerminator(source) => source,
      ClarError::UnexpectedShortOption(source, _) => source,
      ClarError::UnexpectedLongOption(source, _) => source,
      ClarError::UnexpectedArgument(source, _) => source,
      ClarError::UnexpectedOptionTerminator(source) => source,
      ClarError::ShortOptionMustBeUsedAlone(source, _) => source,
      ClarError::LongOptionMustBeUsedAlone(source, _) => source,
      ClarError::ShortOptionDoesNotAcceptValue(source, _) => source,
      ClarError::LongOptionDoesNotAcceptValue(source, _) => source,
      ClarError::ShortOptionRequiresValue(source, _, _) => source,
      ClarError::LongOptionRequiresValue(source, _, _) => source,
      ClarError::ShortOptionMustBeLetterOrDigit(source, _) => source,
      ClarError::LongOptionMustStartWithLetter(source, _) => source,
      ClarError::LongOptionMustContainLettersDigitsHyphens(source, _) => source,
    }
  }
}

fn source() -> String {
  "".to_string()
}

pub fn err_missing_required_argument(caption: impl AsRef<str>) -> ClarError {
  ClarError::MissingRequiredArgument(source(), caption.as_ref().to_string())
}

pub fn err_missing_required_option_terminator() -> ClarError {
  ClarError::MissingRequiredOptionTerminator(source())
}

pub fn err_unexpected_short_option(label: char) -> ClarError {
  ClarError::UnexpectedShortOption(source(), label)
}

pub fn err_unexpected_long_option(label: String) -> ClarError {
  ClarError::UnexpectedLongOption(source(), label)
}

pub fn err_unexpected_argument(value: String) -> ClarError {
  ClarError::UnexpectedArgument(source(), value)
}

pub fn err_unexpected_option_terminator() -> ClarError {
  ClarError::UnexpectedOptionTerminator(source())
}

pub fn err_short_option_must_be_used_alone(label: char) -> ClarError {
  ClarError::ShortOptionMustBeUsedAlone(source(), label)
}

pub fn err_long_option_must_be_used_alone(label: String) -> ClarError {
  ClarError::LongOptionMustBeUsedAlone(source(), label)
}

pub fn err_short_option_does_not_accept_a_value(label: char) -> ClarError {
  ClarError::ShortOptionDoesNotAcceptValue(source(), label)
}

pub fn err_long_option_does_not_accept_a_value(label: String) -> ClarError {
  ClarError::LongOptionDoesNotAcceptValue(source(), label)
}

pub fn err_short_option_requires_value(source: String, label: char, caption: String) -> ClarError {
  ClarError::ShortOptionRequiresValue(source, label, caption)
}

pub fn err_long_option_requires_value(source: String, label: String, caption: String) -> ClarError {
  ClarError::LongOptionRequiresValue(source, label, caption)
}

pub fn err_short_option_must_be_letter_or_digit(label: char) -> ClarError {
  ClarError::ShortOptionMustBeLetterOrDigit(source(), label)
}

pub fn err_long_option_must_start_with_letter(label: String) -> ClarError {
  ClarError::LongOptionMustStartWithLetter(source(), label)
}

pub fn err_long_option_must_contain_letters_digits_hyphens(label: String) -> ClarError {
  ClarError::LongOptionMustContainLettersDigitsHyphens(source(), label)
}
