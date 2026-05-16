use super::*;

#[test]
fn _0001() {
  // Given: Option that takes a value without default missing value.
  // When: Displaying help.
  // Then: Proper value label in sharp brackets should be displayed.
  let expected = r#"
    Usage: clars [OPTIONS]

    Options:
      -c <WHEN>  Coloring
  "#;
  let matches = Clar::new(APP)
    .options([ClarOption::short("color", 'c').takes_value("WHEN").help("Coloring")])
    .resolve(EMPTY_INPUT)
    .unwrap();
  eq_text(expected, matches.get_help());
  eq_text(expected, matches.get_help_long());
}

#[test]
fn _0002() {
  // Given: Option that takes a value with default missing value.
  // When: Displaying help.
  // Then: Proper value label in square brackets should be displayed.
  let matches = Clar::new(APP)
    .options([ClarOption::short("color", 'c')
      .takes_value("WHEN")
      .default_missing_value("always")
      .help("Coloring")])
    .resolve(EMPTY_INPUT)
    .unwrap();
  eq_text(
    r#"
    Usage: clars [OPTIONS]

    Options:
      -c [<WHEN>]  Coloring [implicit: always]
  "#,
    matches.get_help(),
  );
  eq_text(
    r#"
    Usage: clars [OPTIONS]

    Options:
      -c [<WHEN>]  Coloring
                     [implicit: always]
  "#,
    matches.get_help_long(),
  );
}
