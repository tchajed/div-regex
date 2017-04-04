class Dfa:
    def __init__(self, delta, accept_states, init_state):
        """
        delta: a transition table. Should be an array of hashes, where
        delta[s][x] gives next state for state s on input x.
        accept_states: list of accept states
        """
        self._delta = delta
        self._accept_states = accept_states
        self._init_state = init_state

    @property
    def accept_states(self):
        return self._accept_states

    def transition(self, state, x):
        return self._delta[state][x]

    def run(self, s):
        """ Run DFA on string s, producing a terminating state. """
        state = self.init_state
        for c in s:
            state = self.transition(state, c)
        return state
