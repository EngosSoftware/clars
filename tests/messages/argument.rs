use super::*;

#[test]
fn _0001() {
  let expected = r#"
    error: command 'x' is not defined"#;
  let matches = Clar::new(APP)
    .commands([ClarCommand::new("a").help("A").commands([ClarCommand::new("b").help("B")])])
    .resolve(EMPTY_INPUT)
    .unwrap();
  eq_text(expected, matches.get_help_command("x"));
  eq_text(expected, matches.get_help_long_command("x"));
}

#[test]
fn _0002() {
  // Optional argument.
  let expected = r#"
    Usage: clars  [ARGS]

    Arguments:
      [COLOR]  Color
  "#;
  let matches = Clar::new(APP)
    .arguments([ClarArgument::new("color").help("Color")])
    .resolve(EMPTY_INPUT)
    .unwrap();
  eq_text(expected, matches.get_help());
  eq_text(expected, matches.get_help_long());
}

#[test]
fn _0003() {
  // Required argument.
  let matches = Clar::new(APP)
    .arguments([ClarArgument::new("color").help("Color").help_long("Coloring").caption("COLORING").required()])
    .resolve(["always"])
    .unwrap();
  eq_text(
    r#"
    Usage: clars <COLORING>

    Arguments:
      <COLORING>  Color
  "#,
    matches.get_help(),
  );
  eq_text(
    r#"
    Usage: clars <COLORING>

    Arguments:
      <COLORING>  Coloring
  "#,
    matches.get_help_long(),
  );
}
