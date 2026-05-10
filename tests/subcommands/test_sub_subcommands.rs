use super::*;
use clars::ClarMatches;

fn clar() -> Clar {
  Clar::new(APP).subcommands([
    ClarCommand::new("a").commands([ClarCommand::new("a"), ClarCommand::new("b")]),
    ClarCommand::new("b").commands([
      ClarCommand::new("a"),
      ClarCommand::new("b").commands([ClarCommand::new("a"), ClarCommand::new("b"), ClarCommand::new("c")]),
      ClarCommand::new("c"),
    ]),
    ClarCommand::new("c").commands([ClarCommand::new("a")]),
  ])
}

fn check(present: &[&str], matches: ClarMatches) {
  assert_eq!(present, matches.get_paths());
}

#[test]
fn _0001() {
  check(&["a"], clar().resolve(["a"]).unwrap());
  check(&["a/a"], clar().resolve(["a", "a"]).unwrap());
  check(&["a/b"], clar().resolve(["a", "b"]).unwrap());
  check(&["b"], clar().resolve(["b"]).unwrap());
  check(&["b/a"], clar().resolve(["b", "a"]).unwrap());
  check(&["b/b"], clar().resolve(["b", "b"]).unwrap());
  check(&["b/b/a"], clar().resolve(["b", "b", "a"]).unwrap());
  check(&["b/b/b"], clar().resolve(["b", "b", "b"]).unwrap());
  check(&["b/b/c"], clar().resolve(["b", "b", "c"]).unwrap());
  check(&["b/c"], clar().resolve(["b", "c"]).unwrap());
  check(&["c"], clar().resolve(["c"]).unwrap());
  check(&["c/a"], clar().resolve(["c", "a"]).unwrap());
}

#[test]
fn _0002() {
  assert_eq!(
    "unexpected argument 'e' found",
    clar().resolve(["e"]).unwrap_err().to_string()
  );
  assert_eq!(
    "unexpected argument 'x' found",
    clar().resolve(["a", "x"]).unwrap_err().to_string()
  );
  assert_eq!(
    "unexpected argument 'y' found",
    clar().resolve(["a", "b", "y"]).unwrap_err().to_string()
  );
  assert_eq!(
    "unexpected argument 'z' found",
    clar().resolve(["c", "z"]).unwrap_err().to_string()
  );
}
