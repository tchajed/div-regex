use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::iter;

// TODO: figure out how to get RLS to infer types

pub struct Dfa<S: Hash + Eq, C: Hash + Eq> {
  delta: HashMap<S, HashMap<C, S>>,
  pub accept_states: HashSet<S>,
  pub init_state: S,
}

impl<S: Hash + Eq + Copy, C: Hash + Eq + Copy> Dfa<S, C> {
  fn transition_invariant(
    delta: &HashMap<S, HashMap<C, S>>,
    accept_states: &HashSet<S>,
    init_state: &S,
  ) -> bool {
    let states: HashSet<_> = delta
      .keys()
      .chain(accept_states.iter())
      .chain(iter::once(init_state))
      .collect();
    let chars: HashSet<_> = delta.values().flat_map(|next| next.keys()).collect();
    states.iter().all(|s| delta.contains_key(s)) && delta.values().all(|next| {
      // need a mapping for the same set of inputs
      chars.iter().all(|c| next.contains_key(c)) && next.values().all(|s| states.contains(s))
    })
  }

  pub fn transition_map(&self) -> &HashMap<S, HashMap<C, S>> {
    &self.delta
  }

  pub fn new(delta: HashMap<S, HashMap<C, S>>, accept_states: HashSet<S>, init_state: S) -> Self {
    assert!(Dfa::transition_invariant(
      &delta,
      &accept_states,
      &init_state
    ));
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

  fn make<S, C>(
    delta: impl IntoIterator<Item = (S, impl IntoIterator<Item = (C, S)>)>,
    accept_states: impl IntoIterator<Item = S>,
    initial_state: S,
  ) -> Dfa<S, C>
  where
    S: Hash + Eq + Copy,
    C: Hash + Eq + Copy,
  {
    Dfa::new(
      delta
        .into_iter()
        .map(|(s, next)| (s, next.into_iter().collect()))
        .collect(),
      accept_states.into_iter().collect(),
      initial_state,
    )
  }

  #[test]
  fn trivial_accept() {
    let dfa: Dfa<_, char> = make(vec![(1, vec![])], vec![1], 1);
    assert_eq!(dfa.accepts(vec![]), true);
  }
}
