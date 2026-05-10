use super::*;

#[test]
fn valid_short_option_labels_should_work() {
  let labels = ['a'..='z', 'A'..='Z', '0'..='9'].into_iter().flatten();
  for label in labels {
    let matches = Clar::new(APP)
      .options(vec![ClarOption::new_short("option", label)])
      .resolve([format!("-{}", label)])
      .unwrap();
    assert!(matches.is_present("option"));
  }
}

#[test]
fn valid_long_option_labels_should_work() {
  let labels = ["a", "a0", "a0-b"];
  for label in labels {
    let matches = Clar::new(APP)
      .options(vec![ClarOption::new_long("option", label)])
      .resolve([format!("--{}", label)])
      .unwrap();
    assert!(matches.is_present("option"));
  }
}

#[test]
fn joined_short_options_should_work() {
  let matches = Clar::new(APP)
    .options(vec![
      ClarOption::new_short("A", 'a'),
      ClarOption::new_short("B", 'b'),
      ClarOption::new_short("C", 'c'),
      ClarOption::new_short("D", 'd'),
    ])
    .resolve(["-abcd"])
    .unwrap();
  assert!(matches.is_present("A"));
  assert!(matches.is_present("B"));
  assert!(matches.is_present("C"));
  assert!(matches.is_present("D"));
}
