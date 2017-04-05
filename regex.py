class Regex:
    pass

class Literal(Regex):
    def __init__(self, c):
        self.c = c

    def __repr__(self):
        return "Lit({})".format(self.c)

class Empty(Regex):
    """The empty language."""
    def __repr__(self):
        return "Empty()"

class Alternation(Regex):
    """Disjunction of regexes."""
    def __init__(self, rs):
        self.rs = rs

    def __repr__(self):
        return "Alternation({})".format(self.rs)

class Star(Regex):
    """Kleene star."""
    def __init__(self, r):
        self.r = r

    def __repr__(self):
        return "Star({})".format(self.r)

class Seq(Regex):
    """Concatenation of regexes."""
    def __init__(self, rs):
        self.rs = rs

    def __repr__(self):
        return "Seq({})".format(self.rs)

def Eps():
    """The language of just the empty string."""
    return Star(Empty())
