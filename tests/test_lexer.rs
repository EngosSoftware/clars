use clars::{Lexer, Result, Token};

fn parse<I, S>(input: I) -> Result<Vec<Token>>
where
  I: IntoIterator<Item = S>,
  S: AsRef<str>,
{
  Lexer::default().parse(input).map(|tokens| tokens.to_vec())
}

#[test]
fn _0001() {
  let input: Vec<String> = vec![];
  assert_eq!(0, parse(input).unwrap().len());
}

#[test]
fn _0002() {
  assert_eq!(vec![Token::OptionTerminator], parse(["--"]).unwrap());
}

#[test]
fn _0003() {
  assert_eq!(vec![Token::Value("signature".to_string())], parse(["signature"]).unwrap());
}

#[test]
fn _0004() {
  let input = vec!["--help"];
  assert_eq!(vec![Token::LongOption("help".to_string())], parse(input).unwrap());
}

#[test]
fn _0005() {
  let input = vec!["--h----"];
  assert_eq!(vec![Token::LongOption("h----".to_string())], parse(input).unwrap());
}

#[test]
fn _0006() {
  let input = vec!["--h1-h2-h3"];
  assert_eq!(vec![Token::LongOption("h1-h2-h3".to_string())], parse(input).unwrap());
}

#[test]
fn _0007() {
  let input = vec!["--color=always"];
  assert_eq!(vec![Token::LongOption("color".to_string()), Token::Value("always".to_string())], parse(input).unwrap());
}

#[test]
fn _0008() {
  let input = vec!["--color", "always"];
  assert_eq!(vec![Token::LongOption("color".to_string()), Token::Value("always".to_string())], parse(input).unwrap());
}

#[test]
fn _0009() {
  let input = vec!["--color =   always"];
  assert_eq!("whitespace after", parse(input).unwrap_err().to_string());
}

#[test]
fn _0010() {
  let input = vec!["-- color=always"];
  assert_eq!("whitespace before", parse(input).unwrap_err().to_string());
}

#[test]
fn _0011() {
  let input = vec!["--color=   always"];
  assert_eq!("whitespace before", parse(input).unwrap_err().to_string());
}

#[test]
fn _0012() {
  let input = vec!["--color=always "];
  assert_eq!("whitespace after", parse(input).unwrap_err().to_string());
}

#[test]
fn _0013() {
  let input = vec!["--color=always=never"];
  assert_eq!("too many equal signs", parse(input).unwrap_err().to_string());
}

#[test]
fn _0014() {
  let input = vec!["-"];
  assert_eq!(vec![Token::Value("-".to_string())], parse(input).unwrap());
}

#[test]
fn _0015() {
  let input = vec!["-V"];
  assert_eq!(vec![Token::ShortOption('V')], parse(input).unwrap());
}

#[test]
fn _0016() {
  let input = vec!["-0"];
  assert_eq!(vec![Token::ShortOption('0')], parse(input).unwrap());
}

#[test]
fn _0017() {
  let input = vec!["-c=never"];
  assert_eq!(vec![Token::ShortOption('c'), Token::Value("never".to_string())], parse(input).unwrap());
}

#[test]
fn _0018() {
  let input = vec!["-c=never=always"];
  assert_eq!("too many equal signs", parse(input).unwrap_err().to_string());
}

#[test]
fn _0019() {
  let input = vec!["-c", "never"];
  assert_eq!(vec![Token::ShortOption('c'), Token::Value("never".to_string())], parse(input).unwrap());
}

#[test]
fn _0020() {
  let input = vec!["-czf=file"];
  assert_eq!(
    vec![Token::ShortOption('c'), Token::ShortOption('z'), Token::ShortOption('f'), Token::Value("file".to_string())],
    parse(input).unwrap()
  );
}

#[test]
fn _0021() {
  let input = vec!["-czf", "file"];
  assert_eq!(
    vec![Token::ShortOption('c'), Token::ShortOption('z'), Token::ShortOption('f'), Token::Value("file".to_string())],
    parse(input).unwrap()
  );
}

#[test]
fn _0022() {
  let input = vec!["-c =never"];
  assert_eq!("whitespace after", parse(input).unwrap_err().to_string());
}

#[test]
fn _0023() {
  let input = vec!["-c=never "];
  assert_eq!("whitespace after", parse(input).unwrap_err().to_string());
}

#[test]
fn _0024() {
  let input = vec!["- c=never"];
  assert_eq!("whitespace before", parse(input).unwrap_err().to_string());
}

#[test]
fn _0025() {
  let input = vec!["-c= never"];
  assert_eq!("whitespace before", parse(input).unwrap_err().to_string());
}

#[test]
fn _0026() {
  assert_eq!("long option name must start with a letter", parse(["--0help"]).unwrap_err().to_string());
}

#[test]
fn _0027() {
  assert_eq!("long option name must contain letters, digits or hyphens", parse(["--h$a"]).unwrap_err().to_string());
}

#[test]
fn _0028() {
  assert_eq!("short option must be a letter or digit", parse(["-$"]).unwrap_err().to_string());
}

#[test]
fn _0029() {
  let input = vec!["--color", "always", "-v"];
  assert_eq!(
    vec![Token::LongOption("color".to_string()), Token::Value("always".to_string()), Token::ShortOption('v')],
    parse(input).unwrap()
  );
}

#[test]
fn _0030() {
  let input = vec!["--color", "--", "--always", "-v", "--"];
  assert_eq!(
    vec![
      Token::LongOption("color".to_string()),
      Token::OptionTerminator,
      Token::Value("--always".to_string()),
      Token::Value("-v".to_string()),
      Token::Value("--".to_string())
    ],
    parse(input).unwrap()
  );
}
