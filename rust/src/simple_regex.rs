use regex::Regex;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Re {
  Group(Vec<char>),
  Empty,
  Star(Box<Re>),
  Optional(Box<Re>),
  Alternation(Vec<Re>),
  Seq(Vec<Re>),
}

fn paren(s: impl fmt::Display) -> String {
  return format!("(?:{})", s);
}

impl Re {
  pub fn star(r: Re) -> Re {
    Re::Star(box r)
  }

  pub fn eps() -> Re {
    Re::star(Re::Empty)
  }

  pub fn add_or(&mut self, r: Re) {
    match self {
      Re::Alternation(ref mut v) => v.push(r),
      _ => *self = Re::Alternation(vec![self.clone(), r]),
    }
  }

  fn literal_group(rs: &Vec<Re>) -> Option<Re> {
    rs.iter()
      .try_fold(Vec::new(), |mut new_cs, r| match r {
        Re::Group(cs) => {
          new_cs.extend(cs);
          Some(new_cs)
        }
        _ => None,
      })
      .map(|cs| Re::Group(cs))
  }

  pub fn simplify(&self) -> Re {
    match self {
      Re::Group(_) => self.clone(),
      Re::Empty => self.clone(),
      Re::Star(box r) => match r.simplify() {
        Re::Star(r) => Re::Star(r),
        r => Re::Star(box r),
      },
      Re::Optional(box r) => match r.simplify() {
        Re::Optional(r) => Re::Optional(r),
        r => Re::Optional(box r),
      },
      Re::Alternation(ref rs) => {
        let mut new_rs = Vec::new();
        let mut has_empty = false;
        for r in rs {
          match r.simplify() {
            Re::Alternation(rs) => new_rs.extend(rs),
            Re::Star(box Re::Empty) => {
              has_empty = true;
            }
            Re::Empty => {}
            r => new_rs.push(r),
          }
        }
        if new_rs.len() == 0 {
          return Re::Empty;
        };
        let r = if new_rs.len() == 1 {
          new_rs[0].clone()
        } else {
          Re::literal_group(&new_rs).unwrap_or(Re::Alternation(new_rs))
        };
        if has_empty {
          Re::Optional(box r)
        } else {
          r
        }
      }
      Re::Seq(ref rs) => {
        let mut new_rs = Vec::new();
        for r in rs {
          match r.simplify() {
            Re::Seq(rs) => new_rs.extend(rs),
            Re::Star(box Re::Empty) => {}
            Re::Empty => return Re::Empty,
            r => new_rs.push(r),
          }
        }
        if new_rs.len() == 0 {
          return Re::eps();
        };
        if new_rs.len() == 1 {
          new_rs[0].clone()
        } else {
          Re::Seq(new_rs)
        }
      }
    }
  }

  fn to_re_syntax(&self) -> String {
    match self {
      Re::Group(cs) => {
        if cs.len() == 1 {
          cs[0].to_string()
        } else {
          format!("[{}]", cs.iter().collect::<String>())
        }
      }
      Re::Empty => panic!("cannot format empty regex"),
      Re::Star(box r) => match r {
        Re::Empty => "".to_string(),
        _ => format!("{}*", r.to_re_syntax()),
      },
      Re::Alternation(ref rs) => paren(format!(
        "{}",
        rs.iter()
          .map(|r| r.to_re_syntax())
          .collect::<Vec<_>>()
          .join("|")
      )),
      Re::Seq(ref rs) => paren(
        rs.iter()
          .map(|r| r.to_re_syntax())
          .collect::<Vec<_>>()
          .concat(),
      ),
      Re::Optional(box r) => format!("{}?", r.to_re_syntax()),
    }
  }

  pub fn regex(&self) -> Regex {
    Regex::new(&format!("^{}$", self.to_re_syntax())).unwrap()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn basic_printing() {
    let abc = Re::Group(vec!['a', 'b', 'c']);
    let tests = vec![
      (Re::Group(vec!['a']), "a"),
      (abc.clone(), "[abc]"),
      (Re::Star(box abc.clone()), "[abc]*"),
      (
        Re::Alternation(vec![abc.clone(), abc.clone()]),
        "(?:[abc]|[abc])",
      ),
      (Re::Seq(vec![abc.clone(), abc.clone()]), "(?:[abc][abc])"),
      (Re::eps(), ""),
    ];
    for (r, expected) in tests {
      assert_eq!(r.regex().as_str(), format!("^{}$", expected))
    }
  }

  #[test]
  fn simplification() {
    let a = Re::Group(vec!['a']);
    let b = Re::Group(vec!['b']);
    let tests = vec![
      (Re::Alternation(vec![]), Re::Empty),
      (Re::Seq(vec![]), Re::eps()),
      (Re::Alternation(vec![a.clone()]), a.clone()),
      (Re::Seq(vec![a.clone()]), a.clone()),
      (Re::Alternation(vec![Re::Alternation(vec![])]), Re::Empty),
      (Re::Seq(vec![Re::Seq(vec![])]), Re::eps()),
      (Re::Alternation(vec![Re::Empty, a.clone()]), a.clone()),
      (Re::Seq(vec![Re::Empty, a.clone()]), Re::Empty),
      (
        Re::Alternation(vec![Re::Empty, a.clone(), b.clone()]),
        Re::Group(vec!['a', 'b']),
      ),
      (
        Re::Alternation(vec![Re::eps(), a.clone()]),
        Re::Optional(box a.clone()),
      ),
      (Re::Star(box a.clone()), Re::Star(box a.clone())),
      (
        Re::Star(box Re::Star(box a.clone())),
        Re::Star(box a.clone()),
      ),
      (
        Re::Star(box Re::Star(box Re::Star(box a.clone()))),
        Re::Star(box a.clone()),
      ),
      (
        Re::Optional(box Re::Optional(box a.clone())),
        Re::Optional(box a.clone()),
      ),
    ];
    for (r, expected) in tests {
      assert_eq!(r.simplify(), expected)
    }
  }
}
