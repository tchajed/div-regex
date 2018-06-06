use dfa::Dfa;
use std::char;
use std::iter;

const BASE: u32 = 10;

pub fn by(n: u32) -> Dfa<u32, char> {
  let delta = (0..n)
    .map(|s| {
      let digit_next = (0..BASE).map(|d| {
        let c: char = char::from_digit(d, BASE).unwrap(); // TODO: d -> char
        (c, (s * BASE + d) % n)
      });
      (s, digit_next.into_iter().collect())
    })
    .collect();
  Dfa::new(delta, 0, iter::once(0).collect())
}

#[cfg(test)]
mod tests {
  use super::*;

  fn test_modulus(n: u32) {
    let dfa = by(n);
    for m in 0..1000 {
      let s = m.to_string();
      assert_eq!(dfa.run(s.chars()), m % n)
    }
  }

  #[test]
  fn test_mod_4() {
    test_modulus(4)
  }

  #[test]
  fn test_mod_10() {
    test_modulus(10)
  }

  #[test]
  fn test_mod_14() {
    test_modulus(14)
  }
}
