mod arguments;
mod options_terminator;

use super::*;

#[test]
fn valid_short_option_labels_should_work() {
  let valid_labels = ['a'..='z', 'A'..='Z', '0'..='9'].into_iter().flatten();
  for label in valid_labels {
    let matches = Clar::new(APP)
      .options(vec![ClarOption::short("option", label)])
      .resolve([format!("-{}", label)])
      .unwrap();
    assert!(matches.is_present("option"));
  }
}

#[test]
fn valid_long_option_labels_should_work() {
  let labels = ["a", "a0", "a0-b"];
  for label in labels {
    let matches = Clar::new(APP)
      .options(vec![ClarOption::long("option", label)])
      .resolve([format!("--{}", label)])
      .unwrap();
    assert!(matches.is_present("option"));
  }
}

#[test]
fn joined_short_options_should_work() {
  let matches = Clar::new(APP)
    .options(vec![
      ClarOption::short("A", 'a'),
      ClarOption::short("B", 'b'),
      ClarOption::short("C", 'c'),
      ClarOption::short("D", 'd'),
    ])
    .resolve(["-abcd"])
    .unwrap();
  assert!(matches.is_present("A"));
  assert!(matches.is_present("B"));
  assert!(matches.is_present("C"));
  assert!(matches.is_present("D"));
}

#[test]
fn short_option_flag_should_not_accept_values() {
  let diagnostic = Clar::new(APP).options(vec![ClarOption::short("help", 'h')]).resolve(["-h=A"]).unwrap_err();
  assert_eq!("option '-h' does not accept a value", diagnostic.to_owned().to_string());
  eq_text(
    r#"
    error: option '-h' does not accept a value

    Usage: clars [OPTIONS]
  "#,
    diagnostic.text().clone(),
  );
}

#[test]
fn long_option_flag_should_not_accept_values() {
  let diagnostic = Clar::new(APP)
    .options(vec![ClarOption::long("help", "help")])
    .resolve(["--help=A"])
    .unwrap_err();
  assert_eq!("option '--help' does not accept a value", diagnostic.to_owned().to_string());
  eq_text(
    r#"
    error: option '--help' does not accept a value

    Usage: clars [OPTIONS]
  "#,
    diagnostic.text().clone(),
  );
}

#[test]
fn short_option_may_require_a_value() {
  eq_diag(
    "a value is required for '-c <WHEN>' but none was supplied",
    r#"
    error: a value is required for '-c <WHEN>' but none was supplied

    Usage: clars [OPTIONS]
  "#,
    Clar::new(APP)
      .options(vec![ClarOption::short("color", 'c').takes_value("WHEN")])
      .resolve(["-c"])
      .unwrap_err(),
  );
}

#[test]
fn long_option_may_require_a_value() {
  eq_diag(
    "a value is required for '--color <WHEN>' but none was supplied",
    r#"
    error: a value is required for '--color <WHEN>' but none was supplied

    Usage: clars [OPTIONS]
  "#,
    clar()
      .options(vec![ClarOption::long("color", "color").takes_value("WHEN")])
      .resolve(["--color"])
      .unwrap_err(),
  );
}

#[test]
fn _0001() {
  // Given: Short option.
  //  When: No input provided.
  //  Then: Nothing should be resolved.
  let matches = clar().options(vec![ClarOption::short("color", 'c')]).resolve(EMPTY_INPUT).unwrap();
  assert!(!matches.is_short("color"));
  assert!(!matches.is_long("color"));
  assert!(!matches.is_present("color"));
  assert_eq!(0, matches.get_count("color"));
  assert_eq!(None, matches.get_first_value("color"));
}

#[test]
fn _0002() {
  // Short option accepts value given after separator.
  let matches = clar()
    .options(vec![ClarOption::short("color", 'c').takes_value("WHEN")])
    .resolve(["-c=always"])
    .unwrap();
  assert_eq!("always", matches.get_first_value("color").unwrap());
  assert!(matches.is_short("color"));
  assert!(!matches.is_long("color"));
}

#[test]
fn _0003() {
  // Short option accepts value given as next argument.
  let matches = Clar::new(APP)
    .options(vec![ClarOption::short("color", 'c').takes_value("WHEN")])
    .resolve(["-c", "always"])
    .unwrap();
  assert_eq!("always", matches.get_first_value("color").unwrap());
}

#[test]
fn _0004() {
  // Long option accepts value given after separator.
  let matches = Clar::new(APP)
    .options(vec![ClarOption::long("color", "color").takes_value("WHEN")])
    .resolve(["--color=always"])
    .unwrap();
  assert_eq!("always", matches.get_first_value("color").unwrap());
  assert!(!matches.is_short("color"));
  assert!(matches.is_long("color"));
}

#[test]
fn _0005() {
  // Long option accepts value given as next argument.
  let matches = Clar::new(APP)
    .options(vec![ClarOption::long("color", "color").takes_value("WHEN")])
    .resolve(["--color", "always"])
    .unwrap();
  assert_eq!("always", matches.get_first_value("color").unwrap());
}

#[test]
fn _0006() {
  // Given: A resolver that accepts only options.
  //  When: Undefined option is provided on input.
  //  Then: Report an error.
  assert_eq!(
    "unexpected option '--version' found",
    Clar::new(APP)
      .options(vec![ClarOption::long("help", "help")])
      .resolve(["--help", "--version"])
      .unwrap_err()
      .to_string()
  );
  assert_eq!(
    "unexpected option '-v' found",
    Clar::new(APP)
      .options(vec![ClarOption::long("help", "help")])
      .resolve(["--help", "-v"])
      .unwrap_err()
      .to_string()
  );
  assert_eq!(
    "unexpected option '--version' found",
    Clar::new(APP)
      .options(vec![ClarOption::short("help", 'h')])
      .resolve(["-h", "--version"])
      .unwrap_err()
      .to_string()
  );
  assert_eq!(
    "unexpected option '-v' found",
    Clar::new(APP)
      .options(vec![ClarOption::short("help", 'h')])
      .resolve(["-h", "-v"])
      .unwrap_err()
      .to_string()
  );
}

#[test]
fn _0007() {
  // Given: Short option.
  //  When: Created as short and added long label.
  //  Then: Both labels on input should be properly resolved.
  assert_eq!(
    1,
    Clar::new(APP)
      .options([ClarOption::short("color", 'c').long_label("color")])
      .resolve(["-c"])
      .unwrap()
      .get_count("color")
  );
  assert_eq!(
    1,
    Clar::new(APP)
      .options([ClarOption::short("color", 'c').long_label("color")])
      .resolve(["--color"])
      .unwrap()
      .get_count("color")
  );
}

#[test]
fn _0008() {
  // Given: Short option.
  //  When: Created as long and added short label.
  //  Then: Both labels on input should be properly resolved.
  assert_eq!(
    1,
    Clar::new(APP)
      .options([ClarOption::long("color", "color").short_label('c')])
      .resolve(["-c"])
      .unwrap()
      .get_count("color")
  );
  assert_eq!(
    1,
    Clar::new(APP)
      .options([ClarOption::long("color", "color").short_label('c')])
      .resolve(["--color"])
      .unwrap()
      .get_count("color")
  );
}

#[test]
fn _0009() {
  // Given: Short option with standalone flag.
  //  When: The option is given on input.
  //  Then: One option is resolved.
  let matches = Clar::new(APP).options([ClarOption::short("help", 'h').standalone()]).resolve(["-h"]).unwrap();
  assert!(matches.is_present("help"));
  assert!(matches.is_short("help"));
  assert!(!matches.is_long("help"));
}

#[test]
fn _0010() {
  // Given: Long option with standalone flag.
  //  When: The option is given on input.
  //  Then: One option is resolved.
  let matches = Clar::new(APP)
    .options([ClarOption::long("help", "help").standalone()])
    .resolve(["--help"])
    .unwrap();
  assert!(matches.is_present("help"));
  assert!(matches.is_long("help"));
  assert!(!matches.is_short("help"));
}

#[test]
fn _0011() {
  // Given: Definition: Options with single option.
  //  When: Default value is set without setting takes value.
  //  Then: Report an error.
  eq_diag(
    "using default value for flag option '-c' is not allowed",
    r#"
      error: using default value for flag option '-c' is not allowed
    "#,
    Clar::new(APP)
      .options(vec![ClarOption::short("color", 'c').default_value("auto")])
      .resolve(EMPTY_INPUT)
      .unwrap_err(),
  );
  eq_diag(
    "using default value for flag option '--color' is not allowed",
    r#"
      error: using default value for flag option '--color' is not allowed
    "#,
    Clar::new(APP)
      .options(vec![ClarOption::long("color", "color").default_value("auto")])
      .resolve(EMPTY_INPUT)
      .unwrap_err(),
  );
  eq_diag(
    "using default value for flag option '--color' is not allowed",
    r#"
      error: using default value for flag option '--color' is not allowed
    "#,
    Clar::new(APP)
      .options(vec![ClarOption::new("color", 'c', "color").default_value("auto")])
      .resolve(EMPTY_INPUT)
      .unwrap_err(),
  );
}

#[test]
fn _0012() {
  // Given: Definition: Options with single option.
  //  When: Default missing value is set without setting takes value.
  //  Then: Report an error.
  eq_diag(
    "using default missing value for flag option '-c' is not allowed",
    r#"
      error: using default missing value for flag option '-c' is not allowed
    "#,
    Clar::new(APP)
      .options(vec![ClarOption::short("color", 'c').default_missing_value("always")])
      .resolve(EMPTY_INPUT)
      .unwrap_err(),
  );
  eq_diag(
    "using default missing value for flag option '--color' is not allowed",
    r#"
      error: using default missing value for flag option '--color' is not allowed
    "#,
    Clar::new(APP)
      .options(vec![ClarOption::long("color", "color").default_missing_value("always")])
      .resolve(EMPTY_INPUT)
      .unwrap_err(),
  );
  eq_diag(
    "using default missing value for flag option '--color' is not allowed",
    r#"
      error: using default missing value for flag option '--color' is not allowed
    "#,
    Clar::new(APP)
      .options(vec![ClarOption::new("color", 'c', "color").default_missing_value("always")])
      .resolve(EMPTY_INPUT)
      .unwrap_err(),
  );
}

#[test]
fn _0013() {
  // Given: Definition: Options with single option.
  //  When: Possible values are set without setting takes value.
  //  Then: Report an error.
  eq_diag(
    "using possible values for flag option '-c' is not allowed",
    r#"
      error: using possible values for flag option '-c' is not allowed
    "#,
    Clar::new(APP)
      .options(vec![ClarOption::short("color", 'c').possible_values(["auto", "always", "never"])])
      .resolve(EMPTY_INPUT)
      .unwrap_err(),
  );
  eq_diag(
    "using possible values for flag option '--color' is not allowed",
    r#"
      error: using possible values for flag option '--color' is not allowed
    "#,
    Clar::new(APP)
      .options(vec![ClarOption::long("color", "color").possible_values(["auto", "always", "never"])])
      .resolve(EMPTY_INPUT)
      .unwrap_err(),
  );
  eq_diag(
    "using possible values for flag option '--color' is not allowed",
    r#"
      error: using possible values for flag option '--color' is not allowed
    "#,
    Clar::new(APP)
      .options(vec![ClarOption::new("color", 'c', "color").possible_values(["auto", "always", "never"])])
      .resolve(EMPTY_INPUT)
      .unwrap_err(),
  );
}

#[test]
fn _0014() {
  // Given: Definition: Option that takes a value having possible values.
  //  When: Default value is invalid.
  //  Then: Report an error.
  eq_diag(
    "invalid default value 'brown' for '-c <WHEN>'\n  [possible values: auto, always, never]",
    r#"
      error: invalid default value 'brown' for '-c <WHEN>'
        [possible values: auto, always, never]
    "#,
    Clar::new(APP)
      .options(vec![
        ClarOption::short("color", 'c')
          .takes_value("WHEN")
          .possible_values(["auto", "always", "never"])
          .default_value("brown"),
      ])
      .resolve(EMPTY_INPUT)
      .unwrap_err(),
  );
}

#[test]
fn _0015() {
  // Given: Definition: Option that takes a value having possible values.
  //  When: Default missing value is invalid.
  //  Then: Report an error.
  eq_diag(
    "invalid default missing value 'brown' for '-c [<WHEN>]'\n  [possible values: auto, always, never]",
    r#"
      error: invalid default missing value 'brown' for '-c [<WHEN>]'
        [possible values: auto, always, never]
    "#,
    Clar::new(APP)
      .options(vec![
        ClarOption::short("color", 'c')
          .takes_value("WHEN")
          .possible_values(["auto", "always", "never"])
          .default_missing_value("brown"),
      ])
      .resolve(EMPTY_INPUT)
      .unwrap_err(),
  );
}

#[test]
fn _0016() {
  // Given: Definition: Option that takes a value having possible values.
  //  When: Provided value is invalid.
  //  Then: Report an error.
  eq_diag(
    "invalid value 'brown' for '-c <WHEN>'\n  [possible values: auto, always, never]",
    r#"
      error: invalid value 'brown' for '-c <WHEN>'
        [possible values: auto, always, never]

      Usage: clars [OPTIONS]
    "#,
    Clar::new(APP)
      .options(vec![
        ClarOption::short("color", 'c').takes_value("WHEN").possible_values(["auto", "always", "never"]),
      ])
      .resolve(["-c=brown"])
      .unwrap_err(),
  );
  eq_diag(
    "invalid value 'brown' for '--color <WHEN>'\n  [possible values: auto, always, never]",
    r#"
      error: invalid value 'brown' for '--color <WHEN>'
        [possible values: auto, always, never]

      Usage: clars [OPTIONS]
    "#,
    Clar::new(APP)
      .options(vec![
        ClarOption::long("color", "color")
          .takes_value("WHEN")
          .possible_values(["auto", "always", "never"]),
      ])
      .resolve(["--color=brown"])
      .unwrap_err(),
  );
}
