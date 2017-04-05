# Divisibility regex

This repo produces a regex to determine if a number is divisible by some specific number n.

For example, here's a regular expression that matches numbers divisible by 4:

```
^([048]*|([048]*[159]([37]|([26][048]*[159]))*[26][048]*)|((([048]*[26])|([048]*[159]([37]|([26][048]*[159]))*([048]|([26][048]*[26]))))([26]|([048][048]*[26])|(([159]|([048][048]*[159]))([37]|([26][048]*[159]))*([048]|([26][048]*[26]))))*(([048][048]*)|(([159]|([048][048]*[159]))([37]|([26][048]*[159]))*[26][048]*)))|((([048]*[37])|([048]*[159]([37]|([26][048]*[159]))*([159]|([26][048]*[37])))|((([048]*[26])|([048]*[159]([37]|([26][048]*[159]))*([048]|([26][048]*[26]))))([26]|([048][048]*[26])|(([159]|([048][048]*[159]))([37]|([26][048]*[159]))*([048]|([26][048]*[26]))))*([37]|([048][048]*[37])|(([159]|([048][048]*[159]))([37]|([26][048]*[159]))*([159]|([26][048]*[37]))))))([159]|([26][048]*[37])|(([37]|([26][048]*[159]))([37]|([26][048]*[159]))*([159]|([26][048]*[37])))|(([048]|([26][048]*[26])|(([37]|([26][048]*[159]))([37]|([26][048]*[159]))*([048]|([26][048]*[26]))))([26]|([048][048]*[26])|(([159]|([048][048]*[159]))([37]|([26][048]*[159]))*([048]|([26][048]*[26]))))*([37]|([048][048]*[37])|(([159]|([048][048]*[159]))([37]|([26][048]*[159]))*([159]|([26][048]*[37]))))))*(([26][048]*)|(([37]|([26][048]*[159]))([37]|([26][048]*[159]))*[26][048]*)|(([048]|([26][048]*[26])|(([37]|([26][048]*[159]))([37]|([26][048]*[159]))*([048]|([26][048]*[26]))))([26]|([048][048]*[26])|(([159]|([048][048]*[159]))([37]|([26][048]*[159]))*([048]|([26][048]*[26]))))*(([048][048]*)|(([159]|([048][048]*[159]))([37]|([26][048]*[159]))*[26][048]*))))))$
```

## Finite automaton

Constructing a finite automaton to solve this task is relatively simple. Let us use n to refer to the modulus, the number we're checking divisibility by, and m to refer to the number we're checking. The idea is to handle each digit inductively, from most significant to least significant, keeping track of the number so far mod n.

Suppose we know `m_i % n`, where `m_i` is intuitively the most significant i digits of m; mathematically, we're shifting all of m down to leave only i digits and then truncating the fractional part. We can get one digit because `m_{i+1} = m_i * 10 + d_{i+1}`, where `d_{i+1}` is the new digit. But it's easy to extend our knowledge (this is the inductive part):

```txt
m_{i+1} % n = (m_i * 10 + d_{i+1}) % n
            = ((m_i % n) * 10 + d_{i+1}) % n.
```

This suggest the finite automaton. Putting together the pieces, we'll use the state of the automaton to track `m_i % n` (so we'll need n states), and handle each digit $d_{i+1}$ from most significant to least significant. The above formula tells us how to fill out the entire transition table for the finite automaton.

## DFAs to regular expressions

Where do regular expressions come in? In the theory of computation we think of a regular expression as representing some _language_, which is just the set of things it matches. The term "regular" in "regular expression" is related to the fact that regular expressions represent the regular languages, which are those that could be matched by a DFA.

It's understood that regular expressions are "just" a syntax for NFAs. What's less often mentioned is that NFAs (and DFAs by extension) can be represented as regular expressions - NFAs are no more powerful. This isn't unexpected, but the construction is actually non-obvious. I looked it up in my theory of computation course (CS 373 at UIUC) and was surprised to discover that I had indeed learned a construction and forgotten it. You can read [Prof. Parthasarathy's lecture notes](https://courses.engr.illinois.edu/cs373/sp2010/lectures/lect_08.pdf) for the full construction, but the basic idea is the following:

Define a GNFA (generalized NFA) as an NFA where arbitrary regular expressions, rather than just input characters, appear on the edges. Clearly an NFA can be converted to a GNFA. For simplicity, use a normal form for the GNFA: a single initial state with no incoming edges, and a single accept state with no outgoing edges. The key to the GNFA representation is that we can remove an interior node and fix up the GNFA to work around the removal. You can see `rip_state` in [gnfa.py](gnfa.py) for an implementation (it's really rather simple!). After removing all the interior nodes, the GNFA will just be an initial state, an accept state, and a single edge with an equivalent regular expression!
