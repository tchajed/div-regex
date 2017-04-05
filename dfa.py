class Dfa:
    """Representation of Deterministic finite automata (DFA)."""

    def __init__(self, delta, accept_states, init_state):
        """
        delta: a transition table. Should be an array of hashes, where
            delta[s][x] gives next state for state s on input x.
        accept_states: list of accept states
        """
        self._delta = delta
        self._accept_states = accept_states
        self._init_state = init_state

    def states(self):
        return list(range(len(self._delta)))

    @property
    def accept_states(self):
        return self._accept_states

    @property
    def init_state(self):
        return self._init_state

    def transition(self, s, x):
        """Next state upon receiving input x in state s."""
        return self._delta[s][x]

    def next_states(self, s):
        """Map from next states to inputs triggering the transition.

        Only possible next states appear as keys, and values give a list of
        inputs.
        """
        next_states = {}
        for x, next_s in self._delta[s].items():
            if next_s not in next_states:
                next_states[next_s] = []
            next_states[next_s].append(x)
        return next_states

    def run(self, s):
        """ Run DFA on string s, producing a terminating state. """
        state = self.init_state
        for c in s:
            state = self.transition(state, c)
        return state

    def _minimal_partition(self):
        """Partition the DFA states according to equivalence."""
        non_accept_states = [s for s in self.states()
                             if s not in self.accept_states]
        p = Partition([list(self.accept_states), non_accept_states])
        # will break when we stop making progress
        while True:
            # two states are equivalent if they have map each input to the same
            # partition (assuming q and other_q started in the same partition)
            def same_partition(q, other_q):
                for x in self._delta[q].keys():
                    delta_q = self.transition(q, x)
                    delta_other_q = self.transition(other_q, x)
                    if p.index(delta_q) != p.index(delta_other_q):
                        return False
                return True
            new_p = p.refine(same_partition)
            # reached a fixed point, no more refinement is posible
            if len(new_p.sets) == len(p.sets):
                break
            p = new_p
        return p

    def minimal(self):
        """Compute an equivalent DFA with the minimal number of states.

        Follows the partition-based algorithm in these lecture notes (from
        UIUC's CS 373 from Spring 2010):
        https://courses.engr.illinois.edu/cs373/sp2010/lectures/lect_11.pdf.

        Does not modify self.
        """
        # The heavy lifting of computing which DFA states to merge is handled
        # by _minimal_partition.
        p = self._minimal_partition()

        # Now we must re-organize the DFA to use the partition indices in p as
        # states, copying over the transition function with the new state names.
        state_renaming = p.indices()

        # Each new state transitions according to an arbitrary state from that
        # partition.
        delta = []
        for q in range(len(p.sets)):
            # arbitrary row from old delta corresponding to this partition
            old_q_delta = self._delta[p.sets[q][0]]
            new_q_delta = {}
            for x, old_next_q in old_q_delta.items():
                new_q_delta[x] = state_renaming[old_next_q]
            delta.append(new_q_delta)

        # The initial state may have been renamed.
        init = state_renaming[self._init_state]

        # The accept states are those partitions that contain original accept
        # states. Minimizations begins by keeping accept states together, so if
        # a partition has any accept state, all are accept states.
        accept_states = set([])
        for accept_q in self.accept_states:
            accept_states.add(state_renaming[accept_q])

        # Assemble the new DFA
        return Dfa(delta, accept_states, init)

class Partition:
    def __init__(self, sets):
        self.sets = [s for s in sets if s]

    def refine(self, same_partition):
        sets = []
        for p in self.sets:
            # We take each each element of p and remove it plus anything
            # equivalent to it, moving them into a new top-level partition set.
            # Continue until all the elements of p end up in the same
            # partition.
            while len(p) > 0:
                q = p[0]
                # partition into things similar to q...
                q_p = [q]
                # ...and others
                other_p = []
                for other_q in p[1:]:
                    if same_partition(q, other_q):
                        q_p.append(other_q)
                    else:
                        other_p.append(other_q)
                # this partition was determined by p
                sets.append(q_p)
                # continue into all other elements
                p = other_p
        return Partition(sets)

    def indices(self):
        new_names = {}
        for i, s in enumerate(self.sets):
            for q in s:
                new_names[q] = i
        return new_names

    def index(self, q):
        for i, s in enumerate(self.sets):
            if q in s:
                return i
        raise ValueError("unknown state {} for partition".format(q))
