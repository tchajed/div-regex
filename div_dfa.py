#!/usr/bin/env python3

from __future__ import print_function

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

if __name__ == "__main__":
    import argparse

    parser = argparse.ArgumentParser()
    parser.add_argument("n", type=int,
                        help="modulus to test divisibility against")

    args = parser.parse_args()

    dfa = divisible_by(args.n)

    print(" " * 4, end="")
    for d in range(10):
        print("{:3}".format(d), end="")
    print("")
    for s in range(args.n):
        print("{:3} ".format(s), end="")
        for d in range(10):
            x = str(d)
            print("{:3}".format(dfa.transition(s, x)), end="")
        print("")
