/*
use clars::{Clar, CliArgument, CliOption, Value};

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
  assert_eq!(expected, clar.resolve(input).unwrap_err().0.to_string());
}

#[test]
fn _0001() {
  let options = vec![
    CliOption::new("help", 'h', "help").standalone(),
    CliOption::new("version", 'V', "version").standalone(),
    CliOption::new("style", 's', "style"),
    CliOption::new("label", 'l', "label"),
  ];
  let arguments = vec![CliArgument::new("file")];
  let clar = Clar::new("clars").options_arguments(options, arguments);

  assert_ok(["-h"], clar.clone(), vec![Value::short("help", None)]);
  assert_ok(["--help"], clar.clone(), vec![Value::long("help", None)]);
  assert_ok(["-V"], clar.clone(), vec![Value::short("version", None)]);
  assert_ok(["--version"], clar.clone(), vec![Value::long("version", None)]);
  assert_ok(["-s"], clar.clone(), vec![Value::short("style", None)]);
  assert_ok(["--style"], clar.clone(), vec![Value::long("style", None)]);
  assert_ok(["-l"], clar.clone(), vec![Value::short("label", None)]);
  assert_ok(["--label"], clar.clone(), vec![Value::long("label", None)]);
  assert_ok(["file.json"], clar.clone(), vec![Value::argument("file", "file.json")]);
  assert_err(["-h", "-V"], clar.clone(), "option '-h' must be used alone");
}
*/
