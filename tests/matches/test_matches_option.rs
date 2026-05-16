use super::*;

fn option(takes_value: bool, default_value: Option<String>, default_missing_value: Option<String>) -> ClarOption {
  let mut option = ClarOption::new("color", 'c', "color").max_occurrences(2);
  if takes_value {
    option = option.takes_value("WHEN")
  }
  if let Some(value) = default_value {
    option = option.default_value(value);
  }
  if let Some(value) = default_missing_value {
    option = option.default_missing_value(value);
  }
  option
}

fn eq<I, S>(command_line: I, input: &Input) -> ClarResult<(bool, usize, Vec<Option<String>>)>
where
  I: IntoIterator<Item = S>,
  S: AsRef<str>,
{
  let clar = Clar::new(APP).options(vec![option(
    input.takes_value,
    input.default_value.clone(),
    input.default_missing_value.clone(),
  )]);
  let matches = clar.resolve(command_line).map_err(|e| e.to_owned())?;
  Ok((matches.is_present("color"), matches.get_count("color"), matches.get_values("color")))
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
        vec!["-c=never"]
      } else {
        vec!["-c"]
      }
    }
    2 => {
      if input.value_provided {
        vec!["-c=never", "--color", "Y"]
      } else {
        vec!["-c", "--color"]
      }
    }
    _ => panic!("invalid number of appearances"),
  }
}

fn expected_error_message(input: &Input) -> Option<&str> {
  match (input.takes_value, input.value_provided, input.default_missing_value.is_some()) {
    (true, false, false) if input.appearances > 0 => Some("a value is required for '-c <WHEN>' but none was supplied"),
    (false, true, _) if input.appearances > 0 => Some("option '-c' does not accept a value"),
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
    (1, true, true, _, _) => vec![some!("never")],
    (1, ..) => vec![VALUE_NONE],
    (2, false, true, _, true) => vec![some!("B"), some!("B")],
    (2, true, true, _, _) => vec![some!("never"), some!("Y")],
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
      for takes_value in [true, true] {
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
      assert_eq!(expected_error_message, eq(command_line(input), input).unwrap_err().to_string());
    } else {
      let (is_present, count, values) = eq(command_line(input), input).unwrap();
      assert_eq!(expected_is_present(input), is_present);
      assert_eq!(expected_count(input), count);
      assert_eq!(expected_values(input), values);
    }
  }
}
