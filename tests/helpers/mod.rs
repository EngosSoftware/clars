use super::*;

#[test]
fn get_args_should_work() {
  assert!(get_app_and_args().0.starts_with("mod-"));
}

#[test]
fn get_more_info_hint_should_work() {
  assert_eq!("For more information, try '-h'", get_more_info_hint(ColorMode::Off, &["-h"]).characters());
  assert_eq!(
    "For more information, try '--help'",
    get_more_info_hint(ColorMode::Off, &["--help"]).chars().collect::<String>()
  );
  assert_eq!(
    "For more information, try '-h' or '--help'",
    get_more_info_hint(ColorMode::Off, &["-h", "--help"]).chars().collect::<String>()
  );
}
