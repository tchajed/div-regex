#!/usr/bin/env python3

import re
import unittest

from div_dfa import divisible_by

class TestDivDfa(unittest.TestCase):

    def _testModulusComputation(self, n):
        dfa = divisible_by(n)

        for m in range(1000):
            dfa_mod = dfa.run(str(m))
            true_mod = m % n
            self.assertEqual(dfa_mod, true_mod,
                             msg="wrong {} % {}".format(m, n))

    def _testModulusMatch(self, n):
        dfa = divisible_by(n).minimal()

        for m in range(1000):
            dfa_div = dfa.accepts(str(m))
            true_div = m % n == 0
            self.assertEqual(dfa_div, true_div,
                             msg="wrong divisibility of {} by {}".format(m, n))

    def test_mod_4(self):
        self._testModulusComputation(4)

    def test_mod_10(self):
        self._testModulusComputation(10)

    def test_mod_14(self):
        self._testModulusComputation(14)

    def test_mimimized_4(self):
        self._testModulusMatch(4)

    def test_mimimized_10(self):
        self._testModulusMatch(10)

    def test_mimimized_14(self):
        self._testModulusMatch(14)

if __name__ == "__main__":
    unittest.main()
