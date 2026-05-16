use antex::{ColorMode, Text};
use clars::{Clar, ClarArgument, ClarOption, get_app_and_args, get_more_info_hint};
use std::process::ExitCode;

enum CliAction {
  ShowHelp(Text),
  ShowVersion,
  Replace(String, String, String),
  Error(Text),
}

fn get_cli(app: String) -> Clar {
  Clar::new(app).description("Inplace stream editor").options_arguments(
    [
      ClarOption::short("version", 'V')
        .long_label("version")
        .standalone()
        .help("Print version")
        .help_long("Print\nversion"),
      ClarOption::short("help", 'h')
        .long_label("help")
        .standalone()
        .help("Print help")
        .help_long("Print\nhelp"),
    ],
    [
      ClarArgument::new("pattern")
        .required()
        .help("Regular expression to search for")
        .help_long("Regular\nexpression\nto search for"),
      ClarArgument::new("replacement")
        .required()
        .help("String to replace each match with")
        .help_long("String\nto replace\neach\nmatch with"),
      ClarArgument::new("file")
        .required()
        .help("File to edit in place")
        .help_long("Input file to edit in place"),
    ],
  )
}

fn get_action(app: String, args: Vec<String>) -> CliAction {
  match get_cli(app).resolve(args) {
    Ok(matches) => {
      if matches.is_short("help") {
        return CliAction::ShowHelp(matches.get_help());
      }
      if matches.is_long("help") {
        return CliAction::ShowHelp(matches.get_help_long());
      }
      if matches.is_present("version") {
        return CliAction::ShowVersion;
      }
      if let (Some(pattern), Some(replacement), Some(file)) = (
        matches.get_first_value("pattern"),
        matches.get_first_value("replacement"),
        matches.get_first_value("file"),
      ) {
        return CliAction::Replace(pattern, replacement, file);
      }
      CliAction::ShowHelp(matches.get_help())
    }
    Err(diagnostic) => CliAction::Error(diagnostic.text().clone()),
  }
}

fn main() -> ExitCode {
  let (app, args) = get_app_and_args();
  match get_action(app, args) {
    CliAction::ShowHelp(text) => {
      println!("{}", text)
    }
    CliAction::ShowVersion => {
      println!("0.0.1")
    }
    CliAction::Replace(pattern, replacement, file) => {
      println!("Replaced '{}' with '{}' in file '{}'", pattern, replacement, file)
    }
    CliAction::Error(text) => {
      eprintln!("{}", text);
      eprintln!("{}", get_more_info_hint(ColorMode::default(), &["-h", "--help"]));
      return ExitCode::FAILURE;
    }
  }
  ExitCode::SUCCESS
}
