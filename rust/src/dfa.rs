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
      initial_state,
      accept_states.into_iter().collect(),
    )
  }

  pub fn new(
    delta: HashMap<S, HashMap<C, S>>,
    init_state: S,
    accept_states: HashSet<S>,
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

  fn map<T, F>(&self, f: F) -> Dfa<T, C>
  where
    T: Eq + Hash + Copy,
    F: FnMut(&S) -> T,
  {
    // TODO: how is this different from a mutable reference in the binding? does
    // it hide something from the external signature?
    let mut f = f;
    let mut delta: HashMap<_, HashMap<C, T>> = HashMap::new();
    for (s, old_out) in self.delta.iter() {
      delta.entry(f(s)).or_insert_with(|| {
        old_out.iter().map(|(&c, next_s)| (c, f(&next_s))).collect()
      });
    }
    Dfa::new(
      delta,
      f(&self.init_state),
      self.accept_states.iter().map(f).collect(),
    )
  }

  fn chars(&self) -> Vec<C> {
    self.delta[&self.init_state].keys().map(|c| *c).collect()
  }

  fn minimal_partition(&self) -> Partition<S> {
    let non_accept_states = self
      .delta
      .keys()
      .filter(|&s| !self.accept_states.contains(s))
      .map(|s| *s)
      .collect();
    let accept_states = self.accept_states.iter().map(|s| *s).collect();
    let mut p = Partition::new(vec![non_accept_states, accept_states]);
    let chars = self.chars();
    loop {
      let new_p = p.refine(|q, other_q| {
        let delta_q = &self.delta[q];
        let delta_other_q = &self.delta[other_q];
        chars
          .iter()
          .all(|x| p.index(&delta_q[x]) == p.index(&delta_other_q[x]))
      });
      if new_p.len() == p.len() {
        return p;
      }
      p = new_p
    }
  }

  pub fn minimal(&self) -> Dfa<usize, C> {
    let p = self.minimal_partition();
    self.map(|s| p.index(s))
  }
}

struct Partition<S: Eq + Hash> {
  sets: Vec<Vec<S>>,
  set_index: HashMap<S, usize>,
}

impl<S: Eq + Hash + Clone> Partition<S> {
  pub fn new(sets: Vec<Vec<S>>) -> Self {
    let set_index = sets
      .iter()
      .enumerate()
      .flat_map(|(i, set)| set.into_iter().map(move |q| (q.clone(), i)))
      .collect();
    Partition { sets, set_index }
  }

  pub fn len(&self) -> usize {
    return self.sets.len();
  }

  pub fn refine<F>(&self, same_partition: F) -> Self
  where
    F: FnMut(&S, &S) -> bool,
  {
    let mut same_partition = same_partition;
    let sets: Vec<Vec<S>> = self
      .sets
      .iter()
      .flat_map(|p| {
        let mut sets = vec![];
        let mut p = p.clone();
        while !p.is_empty() {
          let q = p[0].clone();
          let q_p =
            p.drain_filter(|other_q| {
              q == *other_q || same_partition(&q, other_q)
            }).collect();
          sets.push(q_p);
        }
        sets
      })
      .collect();
    Partition::new(sets)
  }

  pub fn index(&self, q: &S) -> usize {
    self.set_index[q]
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
