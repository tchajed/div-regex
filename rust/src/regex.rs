use std::fmt;

#[derive(Clone, Debug)]
pub enum Regex {
  Literal(char),
  Group(Vec<char>),
  Empty,
  Star(Box<Regex>),
  Alternation(Vec<Regex>),
  Seq(Vec<Regex>),
}

fn paren(s: impl fmt::Display) -> String {
  return format!("(?:{})", s);
}

impl Regex {
  pub fn star(r: Regex) -> Regex {
    Regex::Star(box r)
  }

  pub fn eps() -> Regex {
    Regex::star(Regex::Empty)
  }

  pub fn add_or(&mut self, r: Regex) {
    match self {
      Regex::Alternation(ref mut v) => v.push(r),
      _ => *self = Regex::Alternation(vec![self.clone(), r]),
    }
  }

  fn literal_group(rs: &Vec<Regex>) -> Option<Regex> {
    rs.iter()
      .try_fold(Vec::new(), |mut cs, r| match r {
        Regex::Literal(c) => {
          cs.push(*c);
          Some(cs)
        }
        _ => None,
      })
      .map(|cs| Regex::Group(cs))
  }

  pub fn simplify(&self) -> Regex {
    match self {
      Regex::Literal(_) => self.clone(),
      Regex::Group(_) => self.clone(),
      Regex::Empty => self.clone(),
      Regex::Star(box r) => Regex::Star(box r.simplify()),
      Regex::Alternation(ref rs) => {
        if rs.len() == 0 {
          Regex::Empty
        } else if rs.len() == 1 {
          rs[0].clone()
        } else {
          let rs = rs
            .iter()
            .flat_map(|r| match r.simplify() {
              Regex::Alternation(rs) => rs.into_iter(),
              r => vec![r].into_iter(),
            })
            .collect();
          Regex::literal_group(&rs).unwrap_or(Regex::Alternation(rs))
        }
      }
      Regex::Seq(ref rs) => {
        if rs.len() == 0 {
          Regex::eps()
        } else if rs.len() == 1 {
          rs[0].clone()
        } else {
          let rs = rs
            .iter()
            .flat_map(|r| match r.simplify() {
              Regex::Seq(rs) => rs.into_iter(),
              r => vec![r].into_iter(),
            })
            .collect();
          Regex::Seq(rs)
        }
      }
    }
  }

  pub fn print(&self) -> String {
    match self {
      Regex::Literal(c) => c.to_string(),
      Regex::Group(cs) => format!("[{}]", cs.iter().collect::<String>()),
      Regex::Empty => panic!("cannot format empty regex"),
      Regex::Star(box r) => match r {
        Regex::Empty => "".to_string(),
        _ => format!("{}*", r.print()),
      },
      Regex::Alternation(ref rs) => paren(format!(
        "{}",
        rs.iter().map(|r| r.print()).collect::<Vec<_>>().join("|")
      )),
      Regex::Seq(ref rs) => {
        paren(rs.iter().map(|r| r.print()).collect::<Vec<_>>().concat())
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn basic_printing() {
    assert_eq!(Regex::Literal('a').print(), "a");
    assert_eq!(Regex::Group(vec!['a']).print(), "[a]");
    let abc = Regex::Group(vec!['a', 'b', 'c']);
    assert_eq!(abc.print(), "[abc]");
    assert_eq!(Regex::Star(box abc.clone()).print(), "[abc]*");
    assert_eq!(
      Regex::Alternation(vec![abc.clone(), abc.clone()]).print(),
      "(?:[abc]|[abc])"
    );
    assert_eq!(
      Regex::Seq(vec![abc.clone(), abc.clone()]).print(),
      "(?:[abc][abc])"
    );
    assert_eq!(Regex::eps().print(), "");
  }
}
