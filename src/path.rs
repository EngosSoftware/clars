/// Path to item in the definition tree.
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct ClarPath(Vec<String>);

impl ClarPath {
  pub fn join(&self, other: &str) -> Self {
    let mut segments = self.0.clone();
    segments.push(other.to_string());
    Self(segments)
  }

  pub fn names(&self) -> &[String] {
    &self.0
  }

  pub fn parent_names(&self) -> &[String] {
    &self.0[..self.0.len().saturating_sub(1)]
  }

  pub fn parent(&self) -> Option<Self> {
    let mut segments = self.0.clone();
    segments.pop();
    (!segments.is_empty()).then_some(Self(segments))
  }
}

impl std::fmt::Display for ClarPath {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0.join("->"))
  }
}

pub trait IntoClarPath {
  fn into_clar_path(self) -> ClarPath;
}

impl IntoClarPath for &str {
  fn into_clar_path(self) -> ClarPath {
    ClarPath(vec![self.to_string()])
  }
}

impl IntoClarPath for &[&str] {
  fn into_clar_path(self) -> ClarPath {
    ClarPath(self.iter().map(|s| s.to_string()).collect())
  }
}

impl<const N: usize> IntoClarPath for [&str; N] {
  fn into_clar_path(self) -> ClarPath {
    ClarPath(self.iter().map(|s| s.to_string()).collect())
  }
}
