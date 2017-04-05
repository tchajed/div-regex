#!/usr/bin/env python3

from __future__ import print_function

import re
import sys
from div_re import div_re

def test_modulus(n):
    passed = True
    r = re.compile(div_re(n))

    for m in range(1000):
        regex_div = True if r.match(str(m)) else False
        true_div = m % n == 0
        passed = passed and regex_div == true_div
        if regex_div == True and true_div == False:
            print("matched non-divisible {}".format(m), file=sys.stderr)
        if regex_div == False and true_div == True:
            print("failed to match {}".format(m), file=sys.stderr)

    return passed

# TODO: convert to a nose test
if __name__ == "__main__":
    import argparse

    parser = argparse.ArgumentParser()
    parser.add_argument("n", type=int,
                        help="modulus to test divisibility against")

    args = parser.parse_args()

    if not test_modulus(args.n):
        sys.exit(1)
