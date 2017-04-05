#!/usr/bin/env python3

from __future__ import print_function

from div_dfa import divisible_by
import gnfa
import regex

def div_re(n):
    m = divisible_by(n).minimal()
    r = gnfa.Gnfa.dfa_re(m)
    return "^" + r.to_re() + "$"

if __name__ == "__main__":
    import argparse

    parser = argparse.ArgumentParser()
    parser.add_argument("n", type=int,
                        help="modulus to test divisibility against")

    args = parser.parse_args()

    r = div_re(args.n)
    print(r)
