#!/usr/bin/env python3

from __future__ import print_function

import regex

class Gnfa:
    """A GNFA (generalize NFA) is an NFA with regexes on the edges.

    These automata are a useful intermediate step in converting an NFA to a
    regular expression, as they trivially express NFAs and can be reduced to a
    single regular expression. This approach follows the one used in UIUC's
    Theory of Computation course (CS 373). See
    https://courses.engr.illinois.edu/cs373/sp2010/lectures/lect_08.pdf.
    """
    def __init__(self, delta, init, terminal):
        """
        delta: a map from state -> next state -> regex,
            with missing (s, s') pairs treated as the empty regex
        init: initial state
        terminal: final accepting state
        """
        self.delta = delta
        self._init = init
        self._terminal = terminal

    @classmethod
    def from_dfa(cls, dfa):
        delta = {}
        for s in range(len(dfa._delta)):
            s_delta = {}
            for next_s, xs in dfa.next_states(s).items():
                literals = [regex.Literal(x) for x in xs]
                s_delta[next_s] = regex.Alternation(literals)
            delta[s] = s_delta
        init_delta = {}
        init_delta[dfa.init_state] = regex.Eps()
        delta['init'] = init_delta
        for accept_state in dfa.accept_states:
            delta[accept_state]['final'] = regex.Eps()
        return Gnfa(delta, 'init', 'final')

    def transition(self, s, next_s):
        """Regex governing transitions from s to next_s."""
        return self.delta[s].get(next_s, regex.Empty())

    def incoming_edges(self, next_s):
        """Return a list of (s, R_in) pairs where s -> next_s on R_in."""
        edges = []
        for s, s_delta in self.delta.items():
            # filter out useless incoming empty edges
            if next_s in s_delta:
                edges.append((s, s_delta[next_s]))
        return edges

    def outgoing_edges(self, s):
        """Return a list of (next_s, R_out) pairs where s -> next_s on R_out."""
        edges = []
        for next_s, r_out in self.delta[s].items():
            edges.append((next_s, r_out))
        return edges

    def loop_regex(self, s):
        """Return a regex giving the self-loop at s."""
        return self.transition(s, s)

    def _delete_state(self, s):
        """Delete a state, removing transitions in and out.

        Modifies the regex, unless the state is logically unused.
        """
        del self.delta[s]
        for s_delta in self.delta.values():
            if s in s_delta:
                del s_delta[s]

    def rip_state(self, q_rip):
        """Rip out q_rip and patch up the GNFA to be equivalent."""
        in_list = self.incoming_edges(q_rip)
        out_list = self.outgoing_edges(q_rip)
        R_rip = regex.Star(self.loop_regex(q_rip))
        # We now have in_list, a list of incoming edges to q_rip, and out_list,
        # a list of outgoing edges. There might be cases where the states in
        # in_list and out_list are the same, which is fine. We will iterate
        # over all pairs of elements in order to consider all paths that go
        # through q_rip.
        for q_in, r_in in in_list:
            for q_out, r_out in out_list:
                # r_rip_replacement is a way to go from q_in directly to q_out
                # wherever the original GNFA went through q_rip.
                r_rip_replacement = regex.Seq([r_in, R_rip, r_out])
                # We want to install r_rip_replacement, but there might already
                # be an existing path: the GNFA should be able to take either,
                # so we construct an OR of the old path and the new one.
                old_in_out = self.transition(q_in, q_out)
                self.delta[q_in][q_out] = regex.Alternation([old_in_out, r_rip_replacement])
        # Now that every path through q_rip is redundant, we delete it.
        self._delete_state(q_rip)

    def _arbitrary_state(self):
        """Returns some state that isn't the initial or final state."""
        for s in self.delta.keys():
            if s != self._init:
                return s
        return None

    def rip_all(self):
        """Reduce the GNFA by removing all but the initial and final states."""
        q_rip = self._arbitrary_state()
        while q_rip is not None:
            self.rip_state(q_rip)
            q_rip = self._arbitrary_state()

    @classmethod
    def dfa_re(cls, dfa):
        """Convert a DFA to a regular expression via a GNFA.

        Performs regular expression simplification on the computed regular expression.
        """
        m = cls.from_dfa(dfa)
        m.rip_all()
        if list(m.delta.keys()) != [m._init]:
            raise ValueError('GNFA must have only init state')
        if list(m.delta['init'].keys()) != [m._terminal]:
            raise ValueError('GNFA must transition only to final state')
        r = m.transition(m._init, m._terminal)
        return regex.simplify(r)
