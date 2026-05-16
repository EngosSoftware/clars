use super::*;

#[test]
fn _0001() {
  let expected = r#"
    Usage: clars <COMMAND>

    Commands:
      a  A
  "#;
  let matches = Clar::new(APP).commands([ClarCommand::new("a").help("A")]).resolve(EMPTY_INPUT).unwrap();
  eq_text(expected, matches.get_help());
  eq_text(expected, matches.get_help_long());
}

#[test]
fn _0002() {
  let expected = r#"
    A

    Usage: clars a <COMMAND>

    Commands:
      b  B
  "#;
  let matches = Clar::new(APP)
    .commands([ClarCommand::new("a").help("A").commands([ClarCommand::new("b").help("B")])])
    .resolve(EMPTY_INPUT)
    .unwrap();
  eq_text(expected, matches.get_help_command("a"));
  eq_text(expected, matches.get_help_long_command("a"));
}

#[test]
fn _0003() {
  let expected = r#"
    B

    Usage: clars a b
  "#;
  let matches = Clar::new(APP)
    .commands([ClarCommand::new("a").help("A").commands([ClarCommand::new("b").help("B")])])
    .resolve(EMPTY_INPUT)
    .unwrap();
  eq_text(expected, matches.get_help_command(["a", "b"]));
  eq_text(expected, matches.get_help_long_command(["a", "b"]));
}

#[test]
fn _0004() {
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
fn _0005() {
  let matches = Clar::new(APP)
    .commands([ClarCommand::new("a").help("A").commands([ClarCommand::new("b").help("B").help_long("BETA")])])
    .resolve(EMPTY_INPUT)
    .unwrap();
  eq_text(
    r#"
    A

    Usage: clars a <COMMAND>

    Commands:
      b  B
  "#,
    matches.get_help_command("a"),
  );
  eq_text(
    r#"
    A

    Usage: clars a <COMMAND>

    Commands:
      b  BETA
  "#,
    matches.get_help_long_command("a"),
  );
}

#[test]
fn _0006() {
  eq_diag(
    "unexpected option '-q' found",
    r#"
      error: unexpected option '-q' found

      Usage: clars a <COMMAND>
    "#,
    clar()
      .description("Application")
      .commands([ClarCommand::new("a").help("A").commands([ClarCommand::new("b").help("B")])])
      .resolve(["a", "b", "-q"])
      .unwrap_err(),
  );
}
