# Divisibility regex

A program to compute a regex that matches numbers divisible by some specific n.

For example, here's a regular expression that matches numbers divisible by 4:

```txt
^(?:[048]*|(?:[048]*[13579](?:[13579]|(?:[26][048]*[13579]))*[26][048]*)|(?:(?:(?:[048]*[26])|(?:[048]*[13579](?:[13579]|(?:[26][048]*[13579]))*(?:[048]|(?:[26][048]*[26]))))(?:[26]|(?:[048][048]*[26])|(?:(?:[13579]|(?:[048][048]*[13579]))(?:[13579]|(?:[26][048]*[13579]))*(?:[048]|(?:[26][048]*[26]))))*(?:(?:[048][048]*)|(?:(?:[13579]|(?:[048][048]*[13579]))(?:[13579]|(?:[26][048]*[13579]))*[26][048]*))))$
```

## Background

Here's some quick background on the theory of computation that you need to understand what we're doing here. I'm going to assume you already know what a regular expression is (sorry!).

In the theory of computation, we define several classes of _machines_. Machines are reduced to the essence of computation: they take an input "string", a sequence of characters, and either accept or reject the whole string. Think of them as programs that match strings. Regular expressions are one example of a class of machines; here are some other examples:

**Deterministic finite automata (DFAs)**: These are programs that have only finitely many states. You run a DFA by following a transition table for each input character: the transition table tells you the next state based on the current state and the next input character. Some states are designated as "accept" states: if at the end of the string the state is an accept state, the program matches the string, and otherwise it doesn't.

**Non-deterministic finite automata (NFAs)**: These are like DFAs but instead of being in one state at any given time, they can be in many. The NFA accepts a string if at the end of the input it's in any accept state. We can run an NFA by tracking a set of states and advancing all of them with each character of input, potentially increasing or decreasing the set of current states.

We often think of DFAs and NFAs as graphs, where the states are nodes and edges give transitions. Edges are labelled with the input that triggers the transition. We can think of executing these graphs by starting in some initial state and following the edges for each input character. In general these graphs define an NFA: we might be in multiple states at the same time. It turns out DFAs have graphs that look like NFAs, but these graphs have a restriction: each state needs exactly one outgoing edge for each input. This condition ensures that when traversing the graph there's always exactly one current state.

## Finite automaton

Constructing a DFA to check divisibility is relatively simple. Let us use n to refer to the modulus, the number we're checking divisibility by, and m to refer to the number we're checking. The idea is to handle each digit inductively, from most significant to least significant, keeping track of the number so far mod n.

Suppose we know `m_i % n`, where `m_i` is intuitively the most significant i digits of m; mathematically, we're shifting all of m down to leave only i digits and then truncating the fractional part. We can get one digit because `m_{i+1} = m_i * 10 + d_{i+1}`, where `d_{i+1}` is the new digit. But it's easy to extend our knowledge (this is the inductive part):

```txt
m_{i+1} % n = (m_i * 10 + d_{i+1}) % n
            = ((m_i % n) * 10 + d_{i+1}) % n.
```

This suggests the finite automaton. Putting together the pieces, we'll use the state of the automaton to track `m_i % n` (so we'll need n states), and handle each digit `d_{i+1}` from most significant to least significant. The above formula tells us how to fill out the entire transition table for the finite automaton.

## DFAs to regular expressions

Now we need to convert the DFA to a regular expression. It's not really obvious how to do this, but there's a nice algorithm. I followed Prof. Parthasarathy's [CS 373 lecture notes](https://courses.engr.illinois.edu/cs373/sp2010/lectures/lect_08.pdf) from UIUC - I had taken this course but forgotten this particular construction. You can refer to the lecture notes for more details, but here's the rough idea.

First, define a GNFA (generalized NFA) to be an NFA where arbitrary regular expressions may appear on the edges. Running a GNFA is similar to an NFA, except that we can travel along any edge whose regular expression matches a prefix of the input. Unlike a DFA especially, we will _consume_ different amounts of input for different edges, and even a given edge might have multiple matches. Rather than just having current states we'll also need to track what part of the input remains for each state, and a given state might have multiple possibilities for how much input remains (for example, the regex `a*` can match `a` and either leave the whole input or consume it entirely).

For simplicity, during this construction we'll ensure a normal form for the GNFA: it will have a single initial state with no incoming edges and a single accept state with no outgoing edges. It's fairly easy to convert an NFA to a GNFA in this normal form, and a DFA is just a special case of an NFA. The key to the GNFA is that we can "rip out" a state and fix up the GNFA so it matches the same strings. You can see `rip_state` in [gnfa.py](gnfa.py) for an implementation (it's really rather simple!). After removing all the interior nodes, the GNFA will just be an initial state, an accept state, and a single edge with an equivalent regular expression!

## Simplifying regular expressions

We have to do one more thing to make the GNFA construction work out: it turns out that it's convenient when working with general regular expression algorithms like this one to use a regular expression that matches nothing. We represent regular expressions as an Abstract Syntax Tree (AST) in [regex.py](regex.py), a form that's much easier to manipulate than strings, and include a regular expression for this empty case. However, in the normal regular expression syntax that programming languages use there's no equivalent form (every regex in languages like Python and Javascript matches _something_). To get around this, we implement some regular expression simplifications, which it turns out will eliminate all uses of the empty regex.

While we're at it, we make a bunch of other simplifications. These make a difference but regardless this approach produces regular expressions that are too complicated to really understand (see the regex above for divisibility by 4 - higher numbers are only worse).
