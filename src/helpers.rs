use antex::{ColorMode, StyledText, Text};
use std::path::Path;

/// Returns the application name and command line arguments.
///
/// # Examples
///
/// ```
/// use clars::get_app_and_args;
///
/// let (app, args) = get_app_and_args();
/// let expected_app_name = "rust_out";
/// assert_eq!(expected_app_name, app);
/// let expected_app_args: Vec<String> = vec![];
/// assert_eq!(expected_app_args, args);
///
/// ```
pub fn get_app_and_args() -> (String, Vec<String>) {
  let mut args = std::env::args();
  let name = Path::new(&args.next().expect("expected at least one argument"))
    .file_name()
    .expect("expected file name")
    .display()
    .to_string();
  (name, args.collect())
}

/// Returns a colored text with more information hint.
pub fn get_more_info_hint(cm: ColorMode, options: &[impl AsRef<str>]) -> Text {
  let mut text = Text::new(cm).s("For more information, try ");
  for (index, option) in options.iter().enumerate() {
    if index > 0 {
      text += " or "
    }
    text += Text::new(cm).s("'").bright_cyan().bold().s(option.as_ref()).reset().s("'");
  }
  text
}
