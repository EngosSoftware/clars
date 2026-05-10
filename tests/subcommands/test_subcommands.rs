use super::*;

#[test]
fn _0001() {
  // Resolver that accepts only subcommands.
  // Subcommand is provided in the command line.
  let matches = Clar::new(APP)
    .subcommands(vec![
      ClarCommand::new("a"),
      ClarCommand::new("b"),
      ClarCommand::new("c"),
    ])
    .resolve(["b"])
    .unwrap();
  assert!(!matches.is_present("a"));
  assert!(matches.is_present("b"));
  assert!(!matches.is_present("c"));
}

#[test]
fn _0002() {
  // Resolver that accepts only subcommands.
  // Subcommand is not provided in the command line.
  let matches = Clar::new(APP)
    .subcommands(vec![
      ClarCommand::new("a"),
      ClarCommand::new("b"),
      ClarCommand::new("c"),
    ])
    .resolve(EMPTY_INPUT)
    .unwrap();
  assert!(!matches.is_present("a"));
  assert!(!matches.is_present("b"));
  assert!(!matches.is_present("c"));
}

#[test]
fn _0003() {
  // Resolver that accepts only subcommands.
  // Unknown subcommand is provided in the command line.
  assert_eq!(
    "unexpected argument 'd' found",
    Clar::new(APP)
      .subcommands(vec![
        ClarCommand::new("a"),
        ClarCommand::new("b"),
        ClarCommand::new("c"),
      ])
      .resolve(["d"])
      .unwrap_err()
      .to_string()
  );
}
