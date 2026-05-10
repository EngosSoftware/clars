use super::*;

#[test]
fn _0001() {
  // Shortest possible message contains only usage with application name.
  let expected = r#"
    Usage: clars
  "#;
  let matches = Clar::new(APP).resolve(EMPTY_INPUT).unwrap();
  eq_msg(expected, matches.get_help());
  eq_msg(expected, matches.get_help_long());
}

#[test]
fn _0002() {
  // Single short option without help content.
  let expected = r#"
    Usage: clars [OPTIONS]

    Options:
      -c
  "#;
  let matches = Clar::new(APP)
    .options([ClarOption::new_short("color", 'c')])
    .resolve(EMPTY_INPUT)
    .unwrap();
  eq_msg(expected, matches.get_help());
  eq_msg(expected, matches.get_help_long());
}

#[test]
fn _0003() {
  // Single short option with only short help content.
  let matches = Clar::new(APP)
    .options([ClarOption::new_short("color", 'c')
      .help("Coloring")
      .help_long("Beautiful coloring")])
    .resolve(EMPTY_INPUT)
    .unwrap();
  eq_msg(
    r#"
    Usage: clars [OPTIONS]

    Options:
      -c  Coloring
  "#,
    matches.get_help(),
  );
  eq_msg(
    r#"
    Usage: clars [OPTIONS]

    Options:
      -c  Beautiful coloring
  "#,
    matches.get_help_long(),
  );
}

#[test]
fn _0004() {
  // Single short option with only long help content.
  let matches = Clar::new(APP)
    .options([ClarOption::new_short("color", 'c').help_long("Coloring")])
    .resolve(EMPTY_INPUT)
    .unwrap();
  eq_msg(
    r#"
    Usage: clars [OPTIONS]

    Options:
      -c
  "#,
    matches.get_help(),
  );
  eq_msg(
    r#"
    Usage: clars [OPTIONS]

    Options:
      -c  Coloring
  "#,
    matches.get_help_long(),
  );
}

#[test]
fn _0005() {
  // Single short option with default value.
  let expected = r#"
    Usage: clars [OPTIONS]

    Options:
      -c  [default: always]
  "#;
  let matches = Clar::new(APP)
    .options([ClarOption::new_short("color", 'c').default_value("always")])
    .resolve(EMPTY_INPUT)
    .unwrap();
  eq_msg(expected, matches.get_help());
  eq_msg(expected, matches.get_help_long());
}

#[test]
fn _0006() {
  // Single short option with default missing value.
  let expected = r#"
    Usage: clars [OPTIONS]

    Options:
      -c  [implicit: never]
  "#;
  let matches = Clar::new(APP)
    .options([ClarOption::new_short("color", 'c').default_missing_value("never")])
    .resolve(EMPTY_INPUT)
    .unwrap();
  eq_msg(expected, matches.get_help());
  eq_msg(expected, matches.get_help_long());
}

#[test]
fn _0007() {
  // Single short option with possible values.
  let expected = r#"
    Usage: clars [OPTIONS]

    Options:
      -c  (values: auto, always, never)
  "#;
  let matches = Clar::new(APP)
    .options([ClarOption::new_short("color", 'c').possible_values(["auto", "always", "never"])])
    .resolve(EMPTY_INPUT)
    .unwrap();
  eq_msg(expected, matches.get_help());
  eq_msg(expected, matches.get_help_long());
}

#[test]
fn _0008() {
  // Single short option with default value and default missing value.
  let expected = r#"
    Usage: clars [OPTIONS]

    Options:
      -c  [default: auto] [implicit: always]
  "#;
  let matches = Clar::new(APP)
    .options([ClarOption::new_short("color", 'c')
      .default_value("auto")
      .default_missing_value("always")])
    .resolve(EMPTY_INPUT)
    .unwrap();
  eq_msg(expected, matches.get_help());
  eq_msg(expected, matches.get_help_long());
}

#[test]
fn _0009() {
  // Single short option with default value, default missing value and possible values.
  let expected = r#"
    Usage: clars [OPTIONS]

    Options:
      -c  [default: auto] [implicit: always] (values: auto, always, never)
  "#;
  let matches = Clar::new(APP)
    .options([ClarOption::new_short("color", 'c')
      .default_value("auto")
      .default_missing_value("always")
      .possible_values(["auto", "always", "never"])])
    .resolve(EMPTY_INPUT)
    .unwrap();
  eq_msg(expected, matches.get_help());
  eq_msg(expected, matches.get_help_long());
}

#[test]
fn _0010() {
  // Single short option with help, default value, default missing value and possible values.
  let matches = Clar::new(APP)
    .options([ClarOption::new_short("color", 'c')
      .default_value("auto")
      .default_missing_value("always")
      .possible_values(["auto", "always", "never"])
      .help("Coloring")
      .help_long("Beautiful coloring")])
    .resolve(EMPTY_INPUT)
    .unwrap();
  eq_msg(
    r#"
    Usage: clars [OPTIONS]

    Options:
      -c  Coloring [default: auto] [implicit: always] (values: auto, always, never)
  "#,
    matches.get_help(),
  );
  eq_msg(
    r#"
    Usage: clars [OPTIONS]

    Options:
      -c  Beautiful coloring
            [default: auto] [implicit: always] (values: auto, always, never)
  "#,
    matches.get_help_long(),
  );
}
