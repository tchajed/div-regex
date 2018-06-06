use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::iter;

// TODO: figure out how to get RLS to infer types

pub struct Dfa<S: Hash + Eq, C: Hash + Eq> {
  delta: HashMap<S, HashMap<C, S>>,
  // TODO: should these be exposed via functions or are fields fine?
  pub init_state: S,
  pub accept_states: HashSet<S>,
}

impl<S: Hash + Eq + Copy, C: Hash + Eq + Copy> Dfa<S, C> {
  // these violations should probably not be a panic but a bit of error information
  fn transition_invariant(&self) -> bool {
    let delta = &self.delta;
    let states: HashSet<_> = delta
      .keys()
      .chain(self.accept_states.iter())
      .chain(iter::once(&self.init_state))
      .collect();
    let chars: HashSet<_> =
      delta.values().flat_map(|next| next.keys()).collect();
    states.iter().all(|s| delta.contains_key(s)) && delta.values().all(|next| {
      // need a mapping for the same set of inputs
      chars.iter().all(|c| next.contains_key(c))
        && next.values().all(|s| states.contains(s))
    })
  }

  pub fn transition_map(&self) -> &HashMap<S, HashMap<C, S>> {
    &self.delta
  }

  pub fn make(
    delta: impl IntoIterator<Item = (S, impl IntoIterator<Item = (C, S)>)>,
    initial_state: S,
    accept_states: impl IntoIterator<Item = S>,
  ) -> Self {
    Dfa::new(
      delta
        .into_iter()
        .map(|(s, next)| (s, next.into_iter().collect()))
        .collect(),
      accept_states.into_iter().collect(),
      initial_state,
    )
  }

  pub fn new(
    delta: HashMap<S, HashMap<C, S>>,
    accept_states: HashSet<S>,
    init_state: S,
  ) -> Self {
    let dfa = Dfa {
      delta,
      init_state,
      accept_states,
    };
    assert!(
      dfa.transition_invariant(),
      "DFA completeness invariant violated"
    );
    dfa
  }

  pub fn transition(&self, s: S, x: C) -> S {
    self.delta[&s][&x]
  }

  pub fn next_states(&self, s: S) -> HashMap<S, Vec<C>> {
    self.delta[&s]
      .iter()
      .fold(HashMap::new(), |mut acc, (&x, &s)| {
        acc.entry(s).or_default().push(x);
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
    let dfa: Dfa<_, char> = Dfa::make(vec![(1, vec![])], 1, vec![1]);
    assert_eq!(dfa.accepts(vec![]), true);
  }

  #[test]
  fn accept_a() {
    let dfa = Dfa::make(
      vec![
        (1, vec![('a', 2), ('b', 3)]),
        (2, vec![('a', 3), ('b', 3)]),
        (3, vec![('a', 3), ('b', 3)]),
      ],
      1,
      vec![2],
    );
    for (input, expected) in vec![
      ("", false),
      ("a", true),
      ("b", false),
      ("aa", false),
      ("ab", false),
    ] {
      assert_eq!(
        dfa.accepts(input.chars()),
        expected,
        "final state {}",
        dfa.run(input.chars())
      );
    }
  }

  #[test]
  #[should_panic]
  fn invariant_all_states_mentioned() {
    Dfa::make(vec![(1, vec![('a', 2)])], 1, vec![1]);
  }

  #[test]
  #[should_panic]
  fn invariant_accept_states_mentioned() {
    Dfa::make(vec![(1, vec![('a', 1)])], 1, vec![2]);
  }

  #[test]
  #[should_panic]
  fn invariant_same_inputs() {
    Dfa::make(vec![(1, vec![('a', 2)]), (2, vec![])], 1, vec![1]);
  }
}
