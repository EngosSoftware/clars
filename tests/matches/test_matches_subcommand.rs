use super::*;

fn subcommand(name: &str, value: Option<String>, default_value: Option<String>) -> CliSubcommand {
  CliSubcommand {
    name: name.to_string(),
    value,
    default_value,
    items: vec![],
  }
}

#[test]
fn _0001() {
  let mut clar = Clar::new("clars");
  clar.add_subcommands(vec![subcommand("print", None, None)]);

  // Subcommand appears once in command line.
  let matches = clar.clone().resolve(["print"]).unwrap();
  assert!(matches.is_present("print"));
  assert_eq!(1, matches.get_count("print"));
  assert_eq!(vec![VALUE_NONE], matches.get_values("print"));

  // Subcommand does not appear in command line.
  let matches = clar.clone().resolve(["send"]).unwrap();
  assert!(!matches.is_present("print"));
  assert_eq!(0, matches.get_count("print"));
  assert_eq!(EMPTY_VALUES, matches.get_values("print"));

  // Subcommand is not configured in command line.
  let matches = clar.resolve(["send"]).unwrap();
  assert!(!matches.is_present("sned"));
  assert_eq!(0, matches.get_count("send"));
  assert_eq!(EMPTY_VALUES, matches.get_values("send"));
}

#[test]
fn _0002() {
  let mut clar = Clar::new("clars");
  clar.add_subcommands(vec![subcommand("print", some!("document"), None)]);

  // Subcommand appears once in command line.
  let matches = clar.clone().resolve(["print"]).unwrap();
  assert!(matches.is_present("print"));
  assert_eq!(1, matches.get_count("print"));
  assert_eq!(vec![some!("document")], matches.get_values("print"));

  // Subcommand does not appear in command line.
  let matches = clar.resolve(["send"]).unwrap();
  assert!(!matches.is_present("print"));
  assert_eq!(0, matches.get_count("print"));
  assert_eq!(EMPTY_VALUES, matches.get_values("print"));
}

#[test]
fn _0003() {
  let mut clar = Clar::new("clars");
  clar.add_subcommands(vec![subcommand("print", some!("document"), some!("send"))]);

  // Subcommand appears once in command line.
  let matches = clar.clone().resolve(["print"]).unwrap();
  assert!(matches.is_present("print"));
  assert_eq!(1, matches.get_count("print"));
  assert_eq!(vec![some!("document")], matches.get_values("print"));

  // Subcommand does not appear in command line.
  let matches = clar.resolve(["send"]).unwrap();
  assert!(!matches.is_present("print"));
  assert_eq!(0, matches.get_count("print"));
  assert_eq!(vec![some!("send")], matches.get_values("print"));
}
