use super::*;

#[test]
fn _0001() {
  // Resolver accepts only subcommands.
  // Subcommand is provided in the command line.
  let matches = Clar::new(APP)
    .commands([ClarCommand::new("a"), ClarCommand::new("b"), ClarCommand::new("c")])
    .resolve(["b"])
    .unwrap();
  assert!(!matches.is_present("a"));
  assert!(matches.is_present("b"));
  assert!(!matches.is_present("c"));
}

#[test]
fn _0002() {
  // Resolver accepts only subcommands.
  // Subcommand is not provided in the command line.
  let matches = Clar::new(APP)
    .commands([ClarCommand::new("a"), ClarCommand::new("b"), ClarCommand::new("c")])
    .resolve(EMPTY_INPUT)
    .unwrap();
  assert!(!matches.is_present("a"));
  assert!(!matches.is_present("b"));
  assert!(!matches.is_present("c"));
}

#[test]
fn _0003() {
  // Resolver accepts only subcommands.
  // Unknown subcommand is provided in the command line.
  assert_eq!(
    "unexpected argument 'd' found",
    Clar::new(APP)
      .commands([ClarCommand::new("a"), ClarCommand::new("b"), ClarCommand::new("c"),])
      .resolve(["d"])
      .unwrap_err()
      .to_string()
  );
}

#[test]
fn _0004() {
  // Resolver accepts only one subcommand that may be followed by an option.
  // The provided option name has an invalid name.
  assert_eq!(
    "long option must contain letters, digits or hyphens but '--he$p' found",
    Clar::new(APP)
      .commands([ClarCommand::new("a").options([ClarOption::long("help", "help")])])
      .resolve(["a", "--he$p"])
      .unwrap_err()
      .to_string()
  );
}

#[test]
fn _0005() {
  // Resolver accepts only one command.
  // The name of this command is invalid.
  eq_diag(
    "command name must contain letters, digits or hyphens but 'a$' found",
    r#"
      error: command name must contain letters, digits or hyphens but 'a$' found
    "#,
    Clar::new(APP).commands([ClarCommand::new("a$")]).resolve(["a$"]).unwrap_err(),
  );
  eq_diag(
    "command name must start with a letter, but '0a' found",
    r#"
      error: command name must start with a letter, but '0a' found
    "#,
    Clar::new(APP).commands([ClarCommand::new("0a")]).resolve(["0a"]).unwrap_err(),
  );
}
