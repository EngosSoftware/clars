use super::*;

fn option(takes_value: bool, default_value: Option<String>, default_missing_value: Option<String>) -> CliOption {
  CliOption {
    name: "help".to_string(),
    short_name: Some('h'),
    long_name: Some("help".to_string()),
    standalone: false,
    default_value,
    default_missing_value,
    takes_value,
  }
}

fn eq<I, S>(command_line: I, input: &Input) -> Result<(bool, usize, Vec<Option<String>>)>
where
  I: IntoIterator<Item = S>,
  S: AsRef<str>,
{
  let mut clar = Clar::new("clars");
  clar.add_options(vec![option(
    input.takes_value,
    input.default_value.clone(),
    input.default_missing_value.clone(),
  )]);
  let matches = clar.clone().resolve(command_line)?;
  Ok((
    matches.is_present("help"),
    matches.get_count("help"),
    matches.get_values("help"),
  ))
}

#[derive(Debug)]
struct Input {
  appearances: usize,
  value_provided: bool,
  takes_value: bool,
  default_value: Option<String>,
  default_missing_value: Option<String>,
}

fn command_line(input: &Input) -> Vec<&str> {
  match input.appearances {
    0 => vec![],
    1 => {
      if input.value_provided {
        vec!["-h=X"]
      } else {
        vec!["-h"]
      }
    }
    2 => {
      if input.value_provided {
        vec!["-h=X", "--help", "Y"]
      } else {
        vec!["-h", "--help"]
      }
    }
    _ => panic!("invalid number of appearances"),
  }
}

fn expected_error_message(input: &Input) -> Option<&str> {
  match (
    input.takes_value,
    input.value_provided,
    input.default_missing_value.is_some(),
  ) {
    (true, false, false) if input.appearances > 0 => Some("option must have a value"),
    (false, true, _) if input.appearances > 0 => Some("option must not have a value"),
    _ => None,
  }
}

fn expected_is_present(input: &Input) -> bool {
  input.appearances > 0
}

fn expected_count(input: &Input) -> usize {
  input.appearances
}

fn expected_values(input: &Input) -> Vec<Option<String>> {
  match (
    input.appearances,
    input.value_provided,
    input.takes_value,
    input.default_value.is_some(),
    input.default_missing_value.is_some(),
  ) {
    (0, _, true, true, _) => vec![input.default_value.clone()],
    (0, ..) => EMPTY_VALUES,
    (1, false, true, _, true) => vec![input.default_missing_value.clone()],
    (1, true, true, _, _) => vec![some!("X")],
    (1, ..) => vec![VALUE_NONE],
    (2, false, true, _, true) => vec![some!("B"), some!("B")],
    (2, true, true, _, _) => vec![some!("X"), some!("Y")],
    (2, ..) => vec![VALUE_NONE, VALUE_NONE],
    (other, ..) => panic!("invalid number of appearances: {}", other),
  }
}

#[test]
fn all_cases_should_work() {
  let boolean_values: [bool; 2] = [false, true];
  let default_values: Vec<Option<String>> = vec![None, some!("A")];
  let default_missing_values: Vec<Option<String>> = vec![None, some!("B")];
  // Prepare inputs for tests.
  let mut inputs = vec![];
  for appearances in 0..=2 {
    for value_provided in boolean_values {
      for takes_value in boolean_values {
        for default_value in &default_values {
          for default_missing_value in &default_missing_values {
            inputs.push(Input {
              appearances,
              value_provided,
              takes_value,
              default_value: default_value.clone(),
              default_missing_value: default_missing_value.clone(),
            });
          }
        }
      }
    }
  }
  // Execute tests.
  for input in &inputs {
    if let Some(expected_error_message) = expected_error_message(input) {
      assert_eq!(
        expected_error_message,
        eq(command_line(input), input).unwrap_err().to_string()
      );
    } else {
      let (is_present, count, values) = eq(command_line(input), input).unwrap();
      assert_eq!(expected_is_present(input), is_present);
      assert_eq!(expected_count(input), count);
      assert_eq!(expected_values(input), values);
    }
  }
}
