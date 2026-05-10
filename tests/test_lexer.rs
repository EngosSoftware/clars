/*
use clars::{Lexer, Result, Token};

fn parse<I, S>(input: I) -> Result<Vec<Token>>
where
  I: IntoIterator<Item = S>,
  S: AsRef<str>,
{
  Lexer::default()
    .parse(input)
    .map(|tokens| tokens.iter().cloned().collect::<Vec<Token>>())
}

#[test]
fn _0001() {
  let input: Vec<String> = vec![];
  assert_eq!(0, parse(input).unwrap().len());
}

#[test]
fn _0002() {
  assert_eq!(vec![Token::OptionTerminator(vec![])], parse(["--"]).unwrap());
}

#[test]
fn _0003() {
  assert_eq!(
    vec![Token::Argument("signature".to_string())],
    parse(["signature"]).unwrap()
  );
}

#[test]
fn _0004() {
  let input = vec!["--help"];
  assert_eq!(vec![Token::LongOption("help".to_string(), None)], parse(input).unwrap());
}

#[test]
fn _0005() {
  let input = vec!["--h----"];
  assert_eq!(
    vec![Token::LongOption("h----".to_string(), None)],
    parse(input).unwrap()
  );
}

#[test]
fn _0006() {
  let input = vec!["--h1-h2-h3"];
  assert_eq!(
    vec![Token::LongOption("h1-h2-h3".to_string(), None)],
    parse(input).unwrap()
  );
}

#[test]
fn _0007() {
  let input = vec!["--color=always"];
  assert_eq!(
    vec![Token::LongOption("color".to_string(), Some("always".to_string()))],
    parse(input).unwrap()
  );
}

#[test]
fn _0008() {
  let input = vec!["--color", "always"];
  assert_eq!(
    vec![
      Token::LongOption("color".to_string(), None),
      Token::Argument("always".to_string())
    ],
    parse(input).unwrap()
  );
}

#[test]
#[ignore]
fn _0009() {
  let input = vec!["--color =   always"];
  assert_eq!("whitespace after", parse(input).unwrap_err().to_string());
}

#[test]
#[ignore]
fn _0010() {
  let input = vec!["-- color=always"];
  assert_eq!("whitespace before", parse(input).unwrap_err().to_string());
}

#[test]
#[ignore]
fn _0011() {
  let input = vec!["--color=   always"];
  assert_eq!("whitespace before", parse(input).unwrap_err().to_string());
}

#[test]
#[ignore]
fn _0012() {
  let input = vec!["--color=always "];
  assert_eq!("whitespace after", parse(input).unwrap_err().to_string());
}

#[test]
#[ignore]
fn _0013() {
  let input = vec!["--color=always=never"];
  assert_eq!("too many equal signs", parse(input).unwrap_err().to_string());
}

#[test]
fn _0014() {
  let input = vec!["-"];
  assert_eq!(vec![Token::Argument("-".to_string())], parse(input).unwrap());
}

#[test]
fn _0015() {
  let input = vec!["-V"];
  assert_eq!(vec![Token::ShortOption('V', None)], parse(input).unwrap());
}

#[test]
fn _0016() {
  let input = vec!["-0"];
  assert_eq!(vec![Token::ShortOption('0', None)], parse(input).unwrap());
}

#[test]
fn _0017() {
  let input = vec!["-c=never"];
  assert_eq!(
    vec![Token::ShortOption('c', Some("never".to_string()))],
    parse(input).unwrap()
  );
}

#[test]
#[ignore]
fn _0018() {
  let input = vec!["-c=never=always"];
  assert_eq!("too many equal signs", parse(input).unwrap_err().to_string());
}

#[test]
fn _0019() {
  let input = vec!["-c", "never"];
  assert_eq!(
    vec![Token::ShortOption('c', None), Token::Argument("never".to_string())],
    parse(input).unwrap()
  );
}

#[test]
fn _0020() {
  let input = vec!["-czf=file"];
  assert_eq!(
    vec![
      Token::ShortOption('c', None),
      Token::ShortOption('z', None),
      Token::ShortOption('f', Some("file".to_string())),
    ],
    parse(input).unwrap()
  );
}

#[test]
fn _0021() {
  let input = vec!["-czf", "file"];
  assert_eq!(
    vec![
      Token::ShortOption('c', None),
      Token::ShortOption('z', None),
      Token::ShortOption('f', None),
      Token::Argument("file".to_string())
    ],
    parse(input).unwrap()
  );
}

#[test]
#[ignore]
fn _0022() {
  let input = vec!["-c =never"];
  assert_eq!("whitespace after", parse(input).unwrap_err().to_string());
}

#[test]
#[ignore]
fn _0023() {
  let input = vec!["-c=never "];
  assert_eq!("whitespace after", parse(input).unwrap_err().to_string());
}

#[test]
#[ignore]
fn _0024() {
  let input = vec!["- c=never"];
  assert_eq!("whitespace before", parse(input).unwrap_err().to_string());
}

#[test]
#[ignore]
fn _0025() {
  let input = vec!["-c= never"];
  assert_eq!("whitespace before", parse(input).unwrap_err().to_string());
}

#[test]
fn _0026() {
  assert_eq!(
    "long option must start with a letter, but '--0help' found",
    parse(["--0help"]).unwrap_err().to_string()
  );
}

#[test]
fn _0027() {
  assert_eq!(
    "long option must contain letters, digits or hyphens but '--h$a' found",
    parse(["--h$a"]).unwrap_err().to_string()
  );
}

#[test]
fn _0028() {
  assert_eq!(
    "short option must be a letter or digit, but '-$' found",
    parse(["-$"]).unwrap_err().to_string()
  );
}

#[test]
fn _0029() {
  let input = vec!["--color", "always", "-v"];
  assert_eq!(
    vec![
      Token::LongOption("color".to_string(), None),
      Token::Argument("always".to_string()),
      Token::ShortOption('v', None)
    ],
    parse(input).unwrap()
  );
}

#[test]
fn _0030() {
  let input = vec!["--color", "--", "--always", "-v", "--"];
  assert_eq!(
    vec![
      Token::LongOption("color".to_string(), None),
      Token::OptionTerminator(vec!["--always".to_string(), "-v".to_string(), "--".to_string()])
    ],
    parse(input).unwrap()
  );
}
*/
