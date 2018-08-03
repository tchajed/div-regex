use dfa::Dfa;
use regex::Regex;
use simple_regex::Re;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::hash::Hash;
use std::iter;

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
pub enum State<S> {
  State(S),
  Init,
  Final,
}

pub struct Gnfa<S: Hash + Eq> {
  delta: HashMap<(State<S>, State<S>), Re>,
}

impl<S: Hash + Eq + Copy> Gnfa<S> {
  fn from_dfa(dfa: &Dfa<S, char>) -> Self {
    let init_transition =
      ((State::Init, State::State(dfa.init_state)), Re::eps());
    let final_transitions = dfa
      .accept_states
      .iter()
      .map(|&s| ((State::State(s), State::Final), Re::eps()));
    let delta: HashMap<_, _> = dfa
      .transition_map()
      .keys()
      .flat_map(|&s1| {
        dfa.next_states(s1).into_iter().map(move |(s2, xs)| {
          ((State::State(s1), State::State(s2)), Re::Group(xs))
        })
      }).chain(iter::once(init_transition))
      .chain(final_transitions)
      .collect();
    Gnfa { delta }
  }

  pub fn dfa_re(dfa: &Dfa<S, char>) -> Regex {
    Gnfa::from_dfa(dfa).to_re().regex()
  }

  pub fn dfa_re_str(dfa: &Dfa<S, char>) -> String {
    Gnfa::from_dfa(dfa).to_re().to_re_syntax()
  }

  fn transition(&self, s: State<S>, next: State<S>) -> Re {
    self.delta.get(&(s, next)).cloned().unwrap_or(Re::Empty)
  }

  fn incoming_edges(&self, next0: S) -> Vec<(State<S>, Re)> {
    self
      .delta
      .iter()
      .filter_map(|((s, next_s), r)| {
        if *next_s == State::State(next0) {
          Some((s.clone(), r.clone()))
        } else {
          None
        }
      }).collect()
  }

  fn outgoing_edges(&self, s0: S) -> Vec<(State<S>, Re)> {
    self
      .delta
      .iter()
      .filter_map(|((s, next_s), r)| {
        if *s == State::State(s0) {
          Some((next_s.clone(), r.clone()))
        } else {
          None
        }
      }).collect()
  }

  fn loop_regex(&self, s: State<S>) -> Re {
    self.transition(s, s)
  }

  // delete all edges referencing s
  fn delete_state(&mut self, s: S) {
    let edges: Vec<_> = self
      .delta
      .keys()
      .flat_map(|&e| {
        if e.0 == State::State(s) || e.1 == State::State(s) {
          Some(e)
        } else {
          None
        }
      }).collect();
    for e in edges {
      self.delta.remove(&e);
    }
  }

  fn rip_state(&mut self, q_rip: S) {
    let in_list = self.incoming_edges(q_rip);
    let out_list = self.outgoing_edges(q_rip);
    let r_rip = Re::star(self.loop_regex(State::State(q_rip)));
    for (q_in, r_in) in in_list {
      for &(q_out, ref r_out) in &out_list {
        let r_in_replacement =
          Re::Seq(vec![r_in.clone(), r_rip.clone(), r_out.clone()]);
        match self.delta.entry((q_in, q_out)) {
          Entry::Occupied(mut e) => e.get_mut().add_or(r_in_replacement),
          Entry::Vacant(e) => {
            e.insert(r_in_replacement);
          }
        }
      }
    }
    self.delete_state(q_rip);
  }

  fn arbitrary_state(&self) -> Option<S> {
    for &(s1, s2) in self.delta.keys() {
      match s1 {
        State::State(s1) => return Some(s1),
        _ => match s2 {
          State::State(s2) => return Some(s2),
          _ => {}
        },
      }
    }
    None
  }

  fn rip_all(&mut self) {
    loop {
      match self.arbitrary_state() {
        Some(q) => self.rip_state(q),
        None => return,
      }
    }
  }

  fn to_re(mut self) -> Re {
    self.rip_all();
    self
      .delta
      .get(&(State::Init, State::Final))
      .cloned()
      .unwrap_or(Re::Empty)
      .simplify()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn even() {
    let delta = vec![(0, vec![('a', 1)]), (1, vec![('a', 0)])];
    let dfa = Dfa::make(delta, 0, vec![0]);
    let re = Gnfa::dfa_re(&dfa);
    let tests = vec![("", true), ("a", false), ("aa", true), ("aaa", false)];
    for (test, expected) in tests {
      assert_eq!(
        re.is_match(test),
        expected,
        "text: \"{}\" regex: {}",
        test,
        re
      );
    }
  }

  #[test]
  fn three_state() {
    // ab(.ab)*
    let delta = vec![
      (0, vec![('a', 1), ('b', 0)]),
      (1, vec![('b', 2), ('a', 0)]),
      (2, vec![('a', 0), ('b', 0)]),
    ];
    let dfa = Dfa::make(delta, 0, vec![2]);
    let re = Gnfa::dfa_re(&dfa);
    let tests = vec![
      ("", false),
      ("ab", true),
      ("aba", false),
      ("abaab", true),
      ("abab", false),
    ];
    for (test, expected) in tests {
      assert_eq!(
        re.is_match(test),
        expected,
        "text: \"{}\" regex: {}",
        test,
        re
      );
    }
  }

  #[test]
  fn ab() {
    let delta = vec![
      (0, vec![('a', 1), ('b', 3)]),
      (1, vec![('b', 2), ('a', 3)]),
      (2, vec![('a', 3), ('b', 3)]),
      (3, vec![('a', 3), ('b', 3)]),
    ];
    let dfa = Dfa::make(delta, 0, vec![2]);
    let re = Gnfa::dfa_re(&dfa);
    let tests = vec![
      ("", false),
      ("a", false),
      ("ab", true),
      ("aba", false),
      ("abaab", false),
      ("abab", false),
    ];
    for (test, expected) in tests {
      assert_eq!(
        re.is_match(test),
        expected,
        "text: \"{}\" regex: {}",
        test,
        re
      );
    }
  }
}
