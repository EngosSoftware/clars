use clars::Token;

#[test]
fn _0001() {
  let input: Vec<String> = vec![];
  assert_eq!(0, clars::parse(input).unwrap().len());
}

#[test]
fn _0002() {
  assert_eq!(vec![Token::OptionTerminator], clars::parse(["--"]).unwrap());
}

#[test]
fn _0003() {
  assert_eq!(vec![Token::Value("signature".to_string())], clars::parse(["signature"]).unwrap());
}

#[test]
fn _0004() {
  let input = vec!["--help"];
  assert_eq!(vec![Token::LongOption("help".to_string())], clars::parse(input).unwrap());
}

#[test]
fn _0005() {
  let input = vec!["--color=always"];
  assert_eq!(
    vec![Token::LongOption("color".to_string()), Token::Value("always".to_string())],
    clars::parse(input).unwrap()
  );
}

#[test]
fn _0006() {
  let input = vec!["--color", "always"];
  assert_eq!(
    vec![Token::LongOption("color".to_string()), Token::Value("always".to_string())],
    clars::parse(input).unwrap()
  );
}

#[test]
fn _0007() {
  let input = vec!["--color =   always"];
  assert_eq!("whitespace after", clars::parse(input).unwrap_err().to_string());
}

#[test]
fn _0008() {
  let input = vec!["-- color=always"];
  assert_eq!("whitespace before", clars::parse(input).unwrap_err().to_string());
}

#[test]
fn _0009() {
  let input = vec!["--color=   always"];
  assert_eq!("whitespace before", clars::parse(input).unwrap_err().to_string());
}

#[test]
fn _0010() {
  let input = vec!["--color=always "];
  assert_eq!("whitespace after", clars::parse(input).unwrap_err().to_string());
}

#[test]
fn _0011() {
  let input = vec!["--color=always=never"];
  assert_eq!("too many equal signs", clars::parse(input).unwrap_err().to_string());
}

#[test]
fn _0012() {
  let input = vec!["-"];
  assert_eq!(vec![Token::Value("-".to_string())], clars::parse(input).unwrap());
}

#[test]
fn _0013() {
  let input = vec!["-V"];
  assert_eq!(vec![Token::ShortOption('V')], clars::parse(input).unwrap());
}

#[test]
fn _0014() {
  let input = vec!["-c=never"];
  assert_eq!(vec![Token::ShortOption('c'), Token::Value("never".to_string())], clars::parse(input).unwrap());
}

#[test]
fn _0015() {
  let input = vec!["-c=never=always"];
  assert_eq!("too many equal signs", clars::parse(input).unwrap_err().to_string());
}

#[test]
fn _0016() {
  let input = vec!["-c", "never"];
  assert_eq!(vec![Token::ShortOption('c'), Token::Value("never".to_string())], clars::parse(input).unwrap());
}

#[test]
fn _0017() {
  let input = vec!["-czf=file"];
  assert_eq!(
    vec![Token::ShortOption('c'), Token::ShortOption('z'), Token::ShortOption('f'), Token::Value("file".to_string())],
    clars::parse(input).unwrap()
  );
}

#[test]
fn _0018() {
  let input = vec!["-czf", "file"];
  assert_eq!(
    vec![Token::ShortOption('c'), Token::ShortOption('z'), Token::ShortOption('f'), Token::Value("file".to_string())],
    clars::parse(input).unwrap()
  );
}

#[test]
fn _0019() {
  let input = vec!["-c =never"];
  assert_eq!("whitespace after", clars::parse(input).unwrap_err().to_string());
}

#[test]
fn _0020() {
  let input = vec!["-c=never "];
  assert_eq!("whitespace after", clars::parse(input).unwrap_err().to_string());
}

#[test]
fn _0021() {
  let input = vec!["- c=never"];
  assert_eq!("whitespace before", clars::parse(input).unwrap_err().to_string());
}

#[test]
fn _0022() {
  let input = vec!["-c= never"];
  assert_eq!("whitespace before", clars::parse(input).unwrap_err().to_string());
}
