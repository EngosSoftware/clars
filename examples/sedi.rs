use clars::{Clar, ClarMatches, CliArgument};

fn main() {
  let argument_pattern = CliArgument {
    name: "pattern".to_string(),
    default_value: None,
    required: true,
  };

  let argument_replacement = CliArgument {
    name: "replacement".to_string(),
    default_value: None,
    required: true,
  };

  let argument_file = CliArgument {
    name: "file".to_string(),
    default_value: None,
    required: true,
  };

  let (app, args) = clars::get_args();
  let mut clar = Clar::new(app);
  clar.add_arguments(vec![argument_pattern, argument_replacement, argument_file]);

  fn get(matches: &ClarMatches, name: &str) -> String {
    matches.get_values(name).first().cloned().flatten().unwrap()
  }

  if let Ok(matches) = clar.resolve(args) {
    println!("     pattern: {}", get(&matches, "pattern"));
    println!(" replacement: {}", get(&matches, "replacement"));
    println!("        file: {}", get(&matches, "file"))
  }
}
