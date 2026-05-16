use super::*;

#[test]
fn _0001() {
  // Given: Option definition with invalid short label.
  //  When: Creating the resolver.
  //  Then: Validation error must be returned with proper message.
  eq_diag(
    "short option must be a letter or digit, but '-$' found",
    r#"
      error: short option must be a letter or digit, but '-$' found
    "#,
    Clar::new(APP).options([ClarOption::short("color", '$')]).resolve(EMPTY_INPUT).unwrap_err(),
  );
}

#[test]
fn _0002() {
  // Given: Option definition with invalid long label.
  //  When: Creating the resolver.
  //  Then: Validation error must be returned with proper message.
  eq_diag(
    "long option must contain letters, digits or hyphens but '--colo®' found",
    r#"
      error: long option must contain letters, digits or hyphens but '--colo®' found
    "#,
    Clar::new(APP).options([ClarOption::long("color", "colo®")]).resolve(EMPTY_INPUT).unwrap_err(),
  );
}

#[test]
fn _0003() {
  // Given: Definition: Commands.
  //  When: Resolved an empty input.
  //  Then: Getting help messages should work properly.
  let matches = Clar::new(APP)
    .commands([ClarCommand::new("a").help("A").help_long("ALPHA")])
    .resolve(EMPTY_INPUT)
    .unwrap();
  eq_text(
    r#"
    A

    Usage: clars a
  "#,
    matches.get_help_command("a"),
  );
  eq_text(
    r#"
    A

    Usage: clars a
  "#,
    matches.get_help_long_command("a"),
  );
  eq_text(
    r#"
    error: command 'b' is not defined"#,
    matches.get_help_command("b"),
  );
  eq_text(
    r#"
    error: command 'b' is not defined"#,
    matches.get_help_long_command("b"),
  );
}

#[test]
fn _0004() {
  // Given: Definition: Options.
  //  When: Resolved an empty input.
  //  Then: Getting help messages for commands should return errors.
  let matches = Clar::new(APP)
    .options([ClarOption::short("color", 'c').help("C").help_long("Coloring")])
    .resolve(EMPTY_INPUT)
    .unwrap();
  eq_text(r#"error: command 'a' is not defined"#, matches.get_help_command("a"));
  eq_text(r#"error: command 'a' is not defined"#, matches.get_help_long_command("a"));
}

#[test]
fn _0005() {
  // Given: Definition: Command -> Commands-Options.
  //  When: Short name of the option in subcommand is invalid.
  //  Then: Validation error must be returned with proper message.
  eq_diag(
    "short option must be a letter or digit, but '-$' found",
    r#"
      error: short option must be a letter or digit, but '-$' found
    "#,
    Clar::new(APP)
      .commands([ClarCommand::new("a").options([ClarOption::short("color", '$')])])
      .resolve(EMPTY_INPUT)
      .unwrap_err(),
  );
}

#[test]
fn _0006() {
  // Given: Definition: Options with one short option.
  //  When: Option appears more than once.
  //  Then: Error must be returned with a proper message.
  eq_diag(
    "option '-c' can occur at most 1 time, found 2",
    r#"
      error: option '-c' can occur at most 1 time, found 2

      Usage: clars [OPTIONS]
    "#,
    Clar::new(APP).options([ClarOption::short("color", 'c')]).resolve(["-c", "-c"]).unwrap_err(),
  );
}

#[test]
fn _0007() {
  // Given: Definition: Options with one long option.
  //  When: Option appears more than once.
  //  Then: Error must be returned with a proper message.
  eq_diag(
    "option '--color' can occur at most 1 time, found 2",
    r#"
      error: option '--color' can occur at most 1 time, found 2

      Usage: clars [OPTIONS]
    "#,
    Clar::new(APP)
      .options([ClarOption::long("color", "color")])
      .resolve(["--color", "--color"])
      .unwrap_err(),
  );
}

#[test]
fn _0008() {
  // Given: Definition: Options with one option.
  //  When: Option appears more than once (mixed labels).
  //  Then: Error must be returned with a proper message.
  eq_diag(
    "option '--color' can occur at most 1 time, found 2",
    r#"
      error: option '--color' can occur at most 1 time, found 2

      Usage: clars [OPTIONS]
    "#,
    Clar::new(APP)
      .options([ClarOption::new("color", 'c', "color")])
      .resolve(["-c", "--color"])
      .unwrap_err(),
  );
}

#[test]
fn _0009() {
  // Given: Definition: Options with one short option.
  //  When: Option appears more than once (short labels).
  //  Then: Error must be returned with a proper message.
  eq_diag(
    "option '-c' can occur at most 1 time, found 2",
    r#"
      error: option '-c' can occur at most 1 time, found 2

      Usage: clars [OPTIONS]
    "#,
    Clar::new(APP).options([ClarOption::short("color", 'c')]).resolve(["-c", "-c"]).unwrap_err(),
  );
}

#[test]
fn _0010() {
  // Given: Definition: Options with one long option.
  //  When: Option appears more than once (long labels).
  //  Then: Error must be returned with a proper message.
  eq_diag(
    "option '--color' can occur at most 1 time, found 2",
    r#"
      error: option '--color' can occur at most 1 time, found 2

      Usage: clars [OPTIONS]
    "#,
    Clar::new(APP)
      .options([ClarOption::long("color", "color")])
      .resolve(["--color", "--color"])
      .unwrap_err(),
  );
}

#[test]
fn _0011() {
  // Given: Definition: Options with one short option.
  //  When: Option appears more than max occurrences limit.
  //  Then: Error must be returned with a proper message.
  eq_diag(
    "option '-c' can occur at most 5 times, found 7",
    r#"
      error: option '-c' can occur at most 5 times, found 7

      Usage: clars [OPTIONS]
    "#,
    Clar::new(APP)
      .options([ClarOption::short("color", 'c').max_occurrences(5)])
      .resolve(["-c", "-c", "-c", "-c", "-c", "-c", "-c"])
      .unwrap_err(),
  );
}
