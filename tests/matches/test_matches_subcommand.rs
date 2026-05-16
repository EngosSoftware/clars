use super::*;

fn subcommand(name: &str) -> ClarCommand {
  ClarCommand::new(name)
}

#[test]
fn _0001() {
  let clar = Clar::new(APP).commands(vec![subcommand("print")]);

  // Subcommand appears once in command line.
  let matches = clar.clone().resolve(["print"]).unwrap();
  assert!(matches.is_present("print"));
  assert_eq!(1, matches.get_count("print"));
  assert_eq!(EMPTY_VALUES, matches.get_values("print"));

  // Subcommand does not appear in command line.
  assert_eq!("unexpected argument 'send' found", clar.resolve(["send"]).unwrap_err().to_string());
}
