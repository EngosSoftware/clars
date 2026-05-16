use crate::APP;
use clars::{Clar, ClarCommand, ClarOption};

fn clar() -> Clar {
  Clar::new(APP).options_commands(
    [ClarOption::new("color", 'c', "color")],
    [
      ClarCommand::new("a").commands([ClarCommand::new("a"), ClarCommand::new("b")]),
      ClarCommand::new("b").commands([
        ClarCommand::new("a"),
        ClarCommand::new("b").commands([ClarCommand::new("a"), ClarCommand::new("b"), ClarCommand::new("c")]),
        ClarCommand::new("c"),
      ]),
      ClarCommand::new("c").commands([ClarCommand::new("a")]),
    ],
  )
}

#[test]
fn _0001() {
  let matches = clar().resolve(["-c", "a", "b"]).unwrap();
  assert!(matches.is_present("color"));
  assert!(matches.is_present(["a", "b"]));
}
