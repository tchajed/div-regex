#!/usr/bin/env python3

# End-to-end test of divisibility regex.
# Run tests with
# python3 -m unittest discover

from __future__ import print_function

import re
import unittest

from div_re import div_re

class TestDivRe(unittest.TestCase):

    def _testModulus(self, n):
        r = re.compile(div_re(n))

        for m in range(1000):
            regex_div = True if r.match(str(m)) else False
            true_div = m % n == 0
            self.assertEqual(regex_div, true_div,
                             msg="wrong divisibility of {} by {}".format(m, n))

    def test_mod_1(self):
        self._testModulus(1)

    def test_mod_3(self):
        self._testModulus(3)

    def test_mod_4(self):
        self._testModulus(4)

    def test_mod_5(self):
        self._testModulus(5)

    def test_mod_7(self):
        self._testModulus(7)

if __name__ == "__main__":
    unittest.main()
