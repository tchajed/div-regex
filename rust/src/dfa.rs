use std::collections::{HashMap, HashSet};
use std::hash::Hash;

// TODO: figure out how to get RLS to infer types
// TODO: figure out why warnings don't show up in flycheck
// TODO: switch to nightly

pub struct Dfa<S: Hash + Eq, C: Hash + Eq> {
  delta: HashMap<S, HashMap<C, S>>,
  accept_states: HashSet<S>,
  init_state: S,
}

impl<S: Hash + Eq + Copy, C: Hash + Eq + Copy> Dfa<S, C> {
  pub fn new(delta: HashMap<S, HashMap<C, S>>, accept_states: HashSet<S>, init_state: S) -> Self {
    Dfa {
      delta,
      accept_states,
      init_state,
    }
  }

  pub fn transition(&self, s: S, x: C) -> S {
    self.delta[&s][&x]
  }

  pub fn next_states(&self, s: S) -> HashMap<S, Vec<C>> {
    self.delta[&s]
      .iter()
      .fold(HashMap::new(), |mut acc, (&x, &s)| {
        acc.entry(s).or_insert(Vec::new()).push(x);
        acc
      })
  }

  pub fn run(&self, input: impl IntoIterator<Item = C>) -> S {
    input
      .into_iter()
      .fold(self.init_state, |s, x| self.delta[&s][&x])
  }

  pub fn accepts(&self, input: impl IntoIterator<Item = C>) -> bool {
    self.accept_states.contains(&self.run(input))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn trivial_accept() {
    let dfa: Dfa<_, char> = Dfa::new(HashMap::new(), vec![1].into_iter().collect(), 1);
    assert_eq!(dfa.accepts(vec![]), true);
  }
}
