# Divisibility regex

A program to compute a regex that matches numbers divisible by some specific n.

For example, here's a regular expression that matches numbers divisible by 4:

```txt
^(?:[048]*|(?:[048]*[13579](?:[13579]|(?:[26][048]*[13579]))*[26][048]*)|(?:(?:(?:[048]*[26])|(?:[048]*[13579](?:[13579]|(?:[26][048]*[13579]))*(?:[048]|(?:[26][048]*[26]))))(?:[26]|(?:[048][048]*[26])|(?:(?:[13579]|(?:[048][048]*[13579]))(?:[13579]|(?:[26][048]*[13579]))*(?:[048]|(?:[26][048]*[26]))))*(?:(?:[048][048]*)|(?:(?:[13579]|(?:[048][048]*[13579]))(?:[13579]|(?:[26][048]*[13579]))*[26][048]*))))$
```

Divisibility by 2 is simpler:

```txt
^(?:[02468]*|(?:[02468]*[13579](?:[13579]|(?:[02468][02468]*[13579]))*[02468][02468]*))$
```

and divisibility by 10 is even simpler:

```txt
^(?:0*|(?:0*[123456789](?:[123456789]|(?:00*[123456789]))*00*))$
```

Note that the program uses `(?:regexp)` to parenthesize `regexp`: this is a non-capturing group; otherwise on Python 2 we run into a limit of 100 capturing groups pretty quickly and even on Python 3 it's much faster.

## Background

Here's some quick background on the theory of computation that you need to understand what we're doing here. I'm going to assume you already know what a regular expression is (sorry!).

In the theory of computation, we define several classes of _machines_. Machines are reduced to the essence of computation: they take an input "string", a sequence of characters, and either accept or reject the whole string. Think of them as programs that match strings. Regular expressions are one example of a class of machines; here are some other examples:

**Deterministic finite automata (DFAs)**: These are programs that have only finitely many states. You run a DFA by following a transition table for each input character: the transition table tells you the next state based on the current state and the next input character. Some states are designated as "accept" states: if at the end of the string the state is an accept state, the program matches the string, and otherwise it doesn't.

**Non-deterministic finite automata (NFAs)**: These are like DFAs but instead of being in one state at any given time, they can be in many. The NFA accepts a string if at the end of the input it's in any accept state. We can run an NFA by tracking a set of states and advancing all of them with each character of input, potentially increasing or decreasing the set of current states.

We often think of DFAs and NFAs as graphs, where the states are nodes and edges give transitions. Edges are labelled with the input that triggers the transition. We can think of executing these graphs by starting in some initial state and following the edges for each input character. In general these graphs define an NFA: we might be in multiple states at the same time. It turns out DFAs have graphs that look like NFAs, but these graphs have a restriction: each state needs exactly one outgoing edge for each input. This condition ensures that when traversing the graph there's always exactly one current state.

## Finite automaton

Constructing a DFA to check divisibility is relatively simple. If we're checking m for divisibility by n, then the DFA will have n states and the state k means the modulus of the number so far is k. The input comes digit by digit, starting from the most significant digit. Let `d_i` be the ith digit and $m_i$ be the number formed by the first i digits; for example if m is 7361 then after the first two digits `m_2 = 73` and the DFA should be in the state 73 % n.

When the DFA receives digit `d_{i+1}`, the number so far goes from `m_i` to `m_{i+1} = m_i * 10 + d_{i+1}`. It's easy enough to compute the new state (`m_{i+1} % n`) knowing only the previous state (`m_i % n`):

```txt
m_{i+1} % n = (m_i * 10 + d_{i+1}) % n
            = ((m_i % n) * 10 + d_{i+1}) % n.
```

This formula gives the complete transition matrix for the DFA.

The initial state of the DFA is zero, since the empty number `m_0 = 0`, and for divisibility checking the only accepting state is 0.

## DFAs to regular expressions

Now we need to convert the DFA to a regular expression. It's not obvious how to do this, but there's a nice algorithm. I followed Prof. Parthasarathy's [CS 373 lecture notes](https://courses.engr.illinois.edu/cs373/sp2010/lectures/lect_08.pdf) from UIUC - I had taken this course but forgotten this particular construction. You can refer to the lecture notes for more details, but here's the rough idea.

First, define a GNFA (generalized NFA) to be an NFA where arbitrary regular expressions may appear on the edges. Running a GNFA is similar to an NFA, except that we can travel along any edge whose regular expression matches a prefix of the input. Unlike a DFA especially, we will _consume_ different amounts of input for different edges, and even a given edge might have multiple matches. Rather than just having current states we'll also need to track what part of the input remains for each state, and a given state might have multiple possibilities for how much input remains (for example, the regex `a*` can match `a` and either leave the whole input or consume it entirely).

For simplicity, during this construction we'll ensure a normal form for the GNFA: it will have a single initial state with no incoming edges and a single accept state with no outgoing edges. It's fairly easy to convert an NFA to a GNFA in this normal form, and a DFA is just a special case of an NFA. The key to the GNFA is that we can "rip out" a state and fix up the GNFA so it matches the same strings. You can see `Gnfa.rip_state` in [gnfa.py](python/gnfa.py) for an implementation (it's really rather simple!). After removing all the interior nodes, the GNFA will just be an initial state, an accept state, and a single edge with an equivalent regular expression!

## Simplifying regular expressions

We have to do one more thing to make the GNFA construction work out: it turns out that it's convenient when working with general regular expression algorithms like this one to use a regular expression that matches nothing. We represent regular expressions as an Abstract Syntax Tree (AST) in [regex.py](python/regex.py), a form that's much easier to manipulate than strings, and include a regular expression for this empty case (the class `Empty`). However, in the normal regular expression syntax that programming languages use there's no equivalent form (every regex in languages like Python and JavaScript matches _something_). To get around this, we implement some regular expression simplifications, which it turns out will eliminate all uses of the empty regex.

While we're at it, we make a bunch of other simplifications. These make a difference but regardless this approach produces regular expressions that are too complicated to really understand (see the regex above for divisibility by 4 - higher numbers are only worse).

## Simplifying the DFA

Rather than just taking the DFA defined above, we actually minimize it first. This makes the regexes slightly shorter in some cases, and is downright necessary for larger DFAs (the regex for divisibility by 10 is otherwise megabytes in size, for a regex that should be equivalent to `^[123456789]*0$`!). The minimization follows the efficient partition-based minimization algorithm in section 4 of the [lecture 11 notes](https://courses.engr.illinois.edu/cs373/sp2010/lectures/lect_11.pdf) from the same CS 373 course.

The idea of DFA minimization is that some DFA states are equivalent: transitions from those states are all symmetric. When DFA states are equivalent, they can be merged to produce an equivalent DFA (one that matches the same strings). The _partition refinement_ algorithm partitions DFA states into equivalent sets that are as large as possible, minimizing the size of the DFA; a _partition_ of the states is a collection of sets of states that are disjoint and whose union is all the states. You can think of a partition as dividing the states into several groups, where the groups will ultimately be chosen to each contain states that can be safely merged.

The algorithm begins by considering all DFA states to fall into only two partition sets: the accept states and the non-accept states. It then identifies cases where two states in the same partition should be separated, because they have different behaviors. We do this until no more partitions can be broken up. When two states are still in the same partition after this process, we know we can merge those states into a single one and not affect the states the DFA matches. The core of the minimization is the partition computed by `Dfa._minimal_partition` in [dfa.py](python/dfa.py). The implementation repeatedly refines with `Partition.refine` while considering states equivalent if on every input they map to states in the same partition. Then `Dfa.minimal` does the grunt work (which is more complicated than the refinement) of renaming states to implement the merging.
