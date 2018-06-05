#[derive(Clone)]
pub enum Regex {
  Literal(char),
  Group(Vec<char>),
  Empty,
  Star(Box<Regex>),
  Alternation(Vec<Regex>),
  Seq(Vec<Regex>),
}

impl Regex {
  pub fn star(r: Regex) -> Regex {
    Regex::Star(Box::new(r))
  }

  pub fn empty() -> Regex {
    Regex::star(Regex::Empty)
  }

  pub fn add_or(&mut self, r: Regex) {
    match self {
      Regex::Alternation(ref mut v) => v.push(r),
      _ => *self = Regex::Alternation(vec![self.clone(), r])
    }
  }
}
