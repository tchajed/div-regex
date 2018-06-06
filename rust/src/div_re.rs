use div_dfa;
use gnfa::Gnfa;
use regex::Regex;

pub fn by(n: u32) -> Regex {
  Gnfa::dfa_re(&div_dfa::by(n))
}

#[cfg(test)]
mod tests {
  use super::*;

  fn test_modulus(m: u32) {
    let r = by(m);
    for n in 0..100 {
      assert_eq!(r.is_match(&format!("{}", n)), n % m == 0)
    }
  }

  #[test]
  fn test_mod_1() {
    test_modulus(1)
  }

  #[test]
  fn test_mod_2() {
    test_modulus(2)
  }

  #[test]
  fn test_mod_4() {
    test_modulus(4)
  }

  #[test]
  fn test_mod_5() {
    test_modulus(5)
  }

  // TODO: run after implementing minimization
  // #[test]
  // fn test_mod_10() {
  //   test_modulus(10)
  // }
}
