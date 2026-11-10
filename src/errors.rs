use antex::{Color, ColorMode, StyledText, Text};
use std::fmt;
use std::fmt::Display;
use std::ops::Deref;

const COLOR_FIRST: Color = Color::Yellow;

const COLOR_SECOND: Color = Color::Cyan;

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
    #[must_use]
    pub const fn new(error: ClarError, text: Text) -> Self {
        Self(error, text)
    }

    #[must_use]
    pub const fn text(&self) -> &Text {
        &self.1
    }
}

/// Command line resolver errors enumeration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClarError {
    MissingRequiredArgument(String),
    MissingRequiredTerminator,
    UnexpectedShortOption(char),
    UnexpectedLongOption(String),
    UnexpectedArgument(String),
    UnexpectedOptionTerminator,
    ShortOptionMustBeUsedAlone(char),
    LongOptionMustBeUsedAlone(String),
    ShortOptionDoesNotAcceptValue(char),
    LongOptionDoesNotAcceptValue(String),
    ShortOptionRequiresValue(char, String),
    LongOptionRequiresValue(String, String),
    ShortOptionMustBeLetterOrDigit(char),
    LongOptionMustStartWithLetter(String),
    LongOptionMustContainLettersDigitsHyphens(String),
    CommandMustStartWithLetter(String),
    CommandMustContainLettersDigitsHyphens(String),
    DefaultValueForRequiredArgument(String),
    ExceededMaxOptionOccurrences(String, usize, usize),
    DefaultValueForFlagOption(String),
    DefaultMissingValueForFlagOption(String),
    PossibleValuesForFlagOption(String),
    InvalidDefaultValueForOption(String, String, Vec<String>),
    InvalidDefaultMissingValueForOption(String, String, Vec<String>),
    InvalidValueForOption(String, String, Vec<String>),
}

impl Display for ClarError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_text(ColorMode::Off))
    }
}

impl ClarError {
    /// Returns a colored error message.
    #[must_use]
    #[allow(clippy::too_many_lines)]
    pub fn as_text(&self, cm: ColorMode) -> Text {
        match self {
            Self::MissingRequiredArgument(caption) => Text::new(cm)
                .s("missing required argument ")
                .color(COLOR_SECOND)
                .bold()
                .s('<')
                .s(caption)
                .s('>')
                .reset(),
            Self::MissingRequiredTerminator => Text::new(cm)
                .s("missing required option terminator '")
                .color(COLOR_FIRST)
                .bold()
                .s("--")
                .reset()
                .s("'"),
            Self::UnexpectedShortOption(label) => Text::new(cm)
                .s("unexpected option '")
                .color(COLOR_FIRST)
                .bold()
                .s('-')
                .s(label)
                .reset()
                .s("' found"),
            Self::UnexpectedLongOption(label) => Text::new(cm)
                .s("unexpected option '")
                .color(COLOR_FIRST)
                .bold()
                .s("--")
                .s(label)
                .reset()
                .s("' found"),
            Self::UnexpectedArgument(value) => Text::new(cm)
                .s("unexpected argument '")
                .color(COLOR_FIRST)
                .bold()
                .s(value)
                .reset()
                .s("' found"),
            Self::UnexpectedOptionTerminator => Text::new(cm)
                .s("unexpected option terminator '")
                .color(COLOR_FIRST)
                .bold()
                .s("--")
                .reset()
                .s("' found"),
            Self::ShortOptionMustBeUsedAlone(label) => Text::new(cm)
                .s("option '")
                .color(COLOR_FIRST)
                .bold()
                .s('-')
                .s(label)
                .reset()
                .s("' must be used alone"),
            Self::LongOptionMustBeUsedAlone(label) => Text::new(cm)
                .s("option '")
                .color(COLOR_FIRST)
                .bold()
                .s("--")
                .s(label)
                .reset()
                .s("' must be used alone"),
            Self::ShortOptionDoesNotAcceptValue(label) => Text::new(cm)
                .s("option '")
                .color(COLOR_FIRST)
                .bold()
                .s('-')
                .s(label)
                .reset()
                .s("' does not accept a value"),
            Self::LongOptionDoesNotAcceptValue(label) => Text::new(cm)
                .s("option '")
                .color(COLOR_FIRST)
                .bold()
                .s("--")
                .s(label)
                .reset()
                .s("' does not accept a value"),
            Self::ShortOptionRequiresValue(label, caption) => Text::new(cm)
                .s("a value is required for '")
                .color(COLOR_FIRST)
                .bold()
                .s("-")
                .s(label)
                .s(" <")
                .s(caption)
                .s(">")
                .reset()
                .s("' but none was supplied"),
            Self::LongOptionRequiresValue(label, caption) => Text::new(cm)
                .s("a value is required for '")
                .color(COLOR_FIRST)
                .bold()
                .s("--")
                .s(label)
                .s(" <")
                .s(caption)
                .s(">")
                .reset()
                .s("' but none was supplied"),
            Self::ShortOptionMustBeLetterOrDigit(label) => Text::new(cm)
                .s("short option must be a letter or digit, but '")
                .color(COLOR_FIRST)
                .bold()
                .s('-')
                .s(label)
                .reset()
                .s("' found"),
            Self::LongOptionMustStartWithLetter(label) => Text::new(cm)
                .s("long option must start with a letter, but '")
                .color(COLOR_FIRST)
                .bold()
                .s("--")
                .s(label)
                .reset()
                .s("' found"),
            Self::LongOptionMustContainLettersDigitsHyphens(label) => Text::new(cm)
                .s("long option must contain letters, digits or hyphens but '")
                .color(COLOR_FIRST)
                .bold()
                .s("--")
                .s(label)
                .reset()
                .s("' found"),
            Self::CommandMustStartWithLetter(name) => Text::new(cm)
                .s("command name must start with a letter, but '")
                .color(COLOR_FIRST)
                .bold()
                .s(name)
                .reset()
                .s("' found"),
            Self::CommandMustContainLettersDigitsHyphens(name) => Text::new(cm)
                .s("command name must contain letters, digits or hyphens but '")
                .color(COLOR_FIRST)
                .bold()
                .s(name)
                .reset()
                .s("' found"),
            Self::DefaultValueForRequiredArgument(name) => Text::new(cm)
                .s("default value for required argument '")
                .color(COLOR_FIRST)
                .bold()
                .s(name)
                .reset()
                .s("' is not allowed"),
            Self::ExceededMaxOptionOccurrences(name, max, occurrences) => Text::new(cm)
                .s("option '")
                .color(COLOR_FIRST)
                .bold()
                .s(name)
                .reset()
                .s("' can occur at most ")
                .s(max)
                .plural(" time", *max)
                .s(", found ")
                .s(occurrences),
            Self::DefaultValueForFlagOption(name) => Text::new(cm)
                .s("using default value for flag option '")
                .color(COLOR_FIRST)
                .bold()
                .s(name)
                .reset()
                .s("' is not allowed"),
            Self::DefaultMissingValueForFlagOption(name) => Text::new(cm)
                .s("using default missing value for flag option '")
                .color(COLOR_FIRST)
                .bold()
                .s(name)
                .reset()
                .s("' is not allowed"),
            Self::PossibleValuesForFlagOption(name) => Text::new(cm)
                .s("using possible values for flag option '")
                .color(COLOR_FIRST)
                .bold()
                .s(name)
                .reset()
                .s("' is not allowed"),
            Self::InvalidDefaultValueForOption(label, value, possible_values) => {
                text_invalid_value(
                    " default ",
                    label.to_owned(),
                    value.to_owned(),
                    possible_values.to_owned(),
                    cm,
                )
            }
            Self::InvalidDefaultMissingValueForOption(label, value, possible_values) => {
                text_invalid_value(
                    " default missing ",
                    label.to_owned(),
                    value.to_owned(),
                    possible_values.to_owned(),
                    cm,
                )
            }
            Self::InvalidValueForOption(label, value, possible_values) => text_invalid_value(
                " ",
                label.to_owned(),
                value.to_owned(),
                possible_values.to_owned(),
                cm,
            ),
        }
    }
}

//-----------------------------------------------------------------------------
// Error functions
//-----------------------------------------------------------------------------

pub fn err_missing_required_argument(caption: &str) -> ClarError {
    ClarError::MissingRequiredArgument(caption.to_string())
}

pub const fn err_missing_required_terminator() -> ClarError {
    ClarError::MissingRequiredTerminator
}

pub const fn err_unexpected_short_option(label: char) -> ClarError {
    ClarError::UnexpectedShortOption(label)
}

pub const fn err_unexpected_long_option(label: String) -> ClarError {
    ClarError::UnexpectedLongOption(label)
}

pub const fn err_unexpected_argument(value: String) -> ClarError {
    ClarError::UnexpectedArgument(value)
}

pub const fn err_unexpected_option_terminator() -> ClarError {
    ClarError::UnexpectedOptionTerminator
}

pub const fn err_short_option_must_be_used_alone(label: char) -> ClarError {
    ClarError::ShortOptionMustBeUsedAlone(label)
}

pub const fn err_long_option_must_be_used_alone(label: String) -> ClarError {
    ClarError::LongOptionMustBeUsedAlone(label)
}

pub const fn err_short_option_does_not_accept_value(label: char) -> ClarError {
    ClarError::ShortOptionDoesNotAcceptValue(label)
}

pub const fn err_long_option_does_not_accept_value(label: String) -> ClarError {
    ClarError::LongOptionDoesNotAcceptValue(label)
}

pub const fn err_short_option_requires_value(label: char, caption: String) -> ClarError {
    ClarError::ShortOptionRequiresValue(label, caption)
}

pub const fn err_long_option_requires_value(label: String, caption: String) -> ClarError {
    ClarError::LongOptionRequiresValue(label, caption)
}

pub const fn err_short_option_must_be_letter_or_digit(label: char) -> ClarError {
    ClarError::ShortOptionMustBeLetterOrDigit(label)
}

pub const fn err_long_option_must_start_with_letter(label: String) -> ClarError {
    ClarError::LongOptionMustStartWithLetter(label)
}

pub const fn err_long_option_must_contain_letters_digits_hyphens(label: String) -> ClarError {
    ClarError::LongOptionMustContainLettersDigitsHyphens(label)
}

pub const fn err_command_must_start_with_letter(name: String) -> ClarError {
    ClarError::CommandMustStartWithLetter(name)
}

pub const fn err_command_must_contain_letters_digits_hyphens(name: String) -> ClarError {
    ClarError::CommandMustContainLettersDigitsHyphens(name)
}

pub const fn err_default_value_for_required_argument(name: String) -> ClarError {
    ClarError::DefaultValueForRequiredArgument(name)
}

pub const fn err_exceeded_max_option_occurrences(
    name: String,
    max: usize,
    occurrences: usize,
) -> ClarError {
    ClarError::ExceededMaxOptionOccurrences(name, max, occurrences)
}

pub const fn err_default_value_for_flag_option(name: String) -> ClarError {
    ClarError::DefaultValueForFlagOption(name)
}

pub const fn err_default_missing_value_for_flag_option(name: String) -> ClarError {
    ClarError::DefaultMissingValueForFlagOption(name)
}

pub const fn err_possible_values_for_flag_option(name: String) -> ClarError {
    ClarError::PossibleValuesForFlagOption(name)
}

pub const fn err_invalid_default_value_for_option(
    label: String,
    value: String,
    possible_values: Vec<String>,
) -> ClarError {
    ClarError::InvalidDefaultValueForOption(label, value, possible_values)
}

pub const fn err_invalid_default_missing_value_for_option(
    label: String,
    value: String,
    possible_values: Vec<String>,
) -> ClarError {
    ClarError::InvalidDefaultMissingValueForOption(label, value, possible_values)
}

pub const fn err_invalid_value_for_option(
    label: String,
    value: String,
    possible_values: Vec<String>,
) -> ClarError {
    ClarError::InvalidValueForOption(label, value, possible_values)
}

//-----------------------------------------------------------------------------
// Utility text functions
//-----------------------------------------------------------------------------

fn text_invalid_value(
    infix: &str,
    label: String,
    value: String,
    possible_values: Vec<String>,
    cm: ColorMode,
) -> Text {
    let mut prefix = "";
    let mut possible_values_text = Text::new(cm);
    for possible_value in possible_values {
        possible_values_text +=
            prefix + Text::new(cm).bright_cyan().bold().s(possible_value).reset();
        prefix = ", ";
    }
    Text::new(cm)
        .s("invalid")
        .s(infix)
        .s("value '")
        .color(COLOR_FIRST)
        .bold()
        .s(value)
        .reset()
        .s("' for '")
        .bright_cyan()
        .bold()
        .s(label)
        .reset()
        .s("'\n  [possible values: ")
        + possible_values_text
        + "]"
}
