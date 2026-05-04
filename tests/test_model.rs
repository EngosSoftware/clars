use clars::Clar;
use clars::model::{CliArgument, CliOption};
use clars::parser::Value;

fn assert_ok<I, S>(input: I, clar: Clar, expected: Vec<Value>)
where
  I: IntoIterator<Item = S>,
  S: AsRef<str>,
{
  let matches = clar.resolve(input).unwrap();
  let values = matches.raw_values();
  assert_eq!(expected, values);
}

fn assert_err<I, S>(input: I, clar: Clar, expected: &str)
where
  I: IntoIterator<Item = S>,
  S: AsRef<str>,
{
  assert_eq!(expected, clar.resolve(input).unwrap_err().to_string());
}

#[test]
fn _0001() {
  let option_help = CliOption {
    name: "help".to_string(),
    short_name: Some('h'),
    long_name: Some("help".to_string()),
    standalone: true,
    default_value: None,
    default_missing_value: None,
    takes_value: false,
  };

  let option_version = CliOption {
    name: "version".to_string(),
    short_name: Some('V'),
    long_name: Some("version".to_string()),
    standalone: true,
    default_value: None,
    default_missing_value: None,
    takes_value: false,
  };

  let option_style = CliOption {
    name: "style".to_string(),
    short_name: Some('s'),
    long_name: Some("style".to_string()),
    standalone: false,
    default_value: None,
    default_missing_value: None,
    takes_value: false,
  };

  let option_label = CliOption {
    name: "label".to_string(),
    short_name: Some('l'),
    long_name: Some("label".to_string()),
    standalone: false,
    default_value: None,
    default_missing_value: None,
    takes_value: false,
  };

  let argument_file = CliArgument {
    name: "FILE".to_string(),
    default_value: None,
    required: false,
  };

  let mut clar = Clar::new("clars");
  clar.add_options(vec![option_help, option_version, option_style, option_label]);
  clar.add_arguments(vec![argument_file]);

  assert_ok(["-h"], clar.clone(), vec![Value::short("help", None)]);
  assert_ok(["--help"], clar.clone(), vec![Value::long("help", None)]);
  assert_ok(["-V"], clar.clone(), vec![Value::short("version", None)]);
  assert_ok(["--version"], clar.clone(), vec![Value::long("version", None)]);
  assert_ok(["-s"], clar.clone(), vec![Value::short("style", None)]);
  assert_ok(["--style"], clar.clone(), vec![Value::long("style", None)]);
  assert_ok(["-l"], clar.clone(), vec![Value::short("label", None)]);
  assert_ok(["--label"], clar.clone(), vec![Value::long("label", None)]);
  assert_ok(["file.json"], clar.clone(), vec![Value::argument("FILE", "file.json")]);
  assert_err(["-h", "-V"], clar.clone(), "option must be used alone");
}
