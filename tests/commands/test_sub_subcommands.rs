use super::*;

fn clar() -> Clar {
  Clar::new(APP).commands([
    ClarCommand::new("a").commands([ClarCommand::new("a"), ClarCommand::new("b")]),
    ClarCommand::new("b").commands([
      ClarCommand::new("a"),
      ClarCommand::new("b").commands([ClarCommand::new("a"), ClarCommand::new("b"), ClarCommand::new("c")]),
      ClarCommand::new("c"),
    ]),
    ClarCommand::new("c").commands([ClarCommand::new("a")]),
  ])
}

fn all_test_cases() -> [Vec<&'static str>; 12] {
  [
    vec!["a"],
    vec!["a", "a"],
    vec!["a", "b"],
    vec!["b"],
    vec!["b", "a"],
    vec!["b", "b"],
    vec!["b", "b", "a"],
    vec!["b", "b", "b"],
    vec!["b", "b", "c"],
    vec!["b", "c"],
    vec!["c"],
    vec!["c", "a"],
  ]
}

fn check(input: Vec<&str>) {
  for test_case in all_test_cases() {
    let matches = clar().resolve(input.as_slice()).unwrap();
    assert_eq!(test_case == input, matches.is_present(test_case.as_slice()));
  }
}

#[test]
fn _0001() {
  for test_case in all_test_cases() {
    check(test_case);
  }
}

#[test]
fn _0004() {
  assert_eq!("unexpected argument 'e' found", clar().resolve(["e"]).unwrap_err().to_string());
  assert_eq!("unexpected argument 'x' found", clar().resolve(["a", "x"]).unwrap_err().to_string());
  assert_eq!("unexpected argument 'y' found", clar().resolve(["a", "b", "y"]).unwrap_err().to_string());
  assert_eq!("unexpected argument 'z' found", clar().resolve(["c", "z"]).unwrap_err().to_string());
}
