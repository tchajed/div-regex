#!/usr/bin/env python3

from dfa import Dfa

def divisible_by(n):
    delta = []
    for s in range(n):
        s_delta = {}
        for d in range(10):
            x = str(d)
            s_delta[x] = (s * 10 + d)%n
        delta.append(s_delta)
    return Dfa(delta, set([0]), 0)
