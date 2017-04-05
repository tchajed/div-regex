class Regex:
    pass

class Literal(Regex):
    def __init__(self, c):
        self.c = c

    def __repr__(self):
        return "Lit({})".format(self.c)

    def to_re(self):
        return self.c

    def is_empty(self):
        return False

    def is_eps(self):
        return False

class LiteralGroup(Regex):
    def __init__(self, cs):
        self.cs = cs

    def __repr__(self):
        return "LitGroup({})".format(self.cs)

    def to_re(self):
        assert len(self.cs) > 0, "empty literal groups are unrepresentable"
        return "[{}]".format("".join(self.cs))

    def is_empty(self):
        return len(self.cs) == 0

    def is_eps(self):
        return False

class Empty(Regex):
    """The empty language."""
    def __repr__(self):
        return "Empty()"

    def to_re(self):
        raise ValueError("empty regex cannot be represented as standard re")

    def is_empty(self):
        return True

    def is_eps(self):
        return False

class Star(Regex):
    """Kleene star."""
    def __init__(self, r):
        self.r = r

    def __repr__(self):
        return "Star({})".format(self.r)

    def to_re(self):
        return "{}*".format(self.r.to_re())

    def is_empty(self):
        return False

    def is_eps(self):
        return self.r.is_empty()

class Alternation(Regex):
    """Disjunction of regexes."""
    def __init__(self, rs):
        self.rs = rs

    def __repr__(self):
        return "Alternation({})".format(self.rs)

    def to_re(self):
        return "({})".format("|".join([r.to_re() for r in self.rs]))

    def is_empty(self):
        # every possibility must be empty
        for r in self.rs:
            if not r.is_empty():
                return False
        return True

    def is_eps(self):
        # every possibility must be uniquely the eps language
        for r in self.rs:
            if not r.is_eps():
                return False
        return True

class Seq(Regex):
    """Concatenation of regexes."""
    def __init__(self, rs):
        self.rs = rs

    def __repr__(self):
        return "Seq({})".format(self.rs)

    def to_re(self):
        return "({})".format("".join([r.to_re() for r in self.rs]))

    def is_empty(self):
        for r in self.rs:
            if r.is_empty():
                return True
        return False

    def is_eps(self):
        for r in self.rs:
            if not r.is_eps():
                return False
        return True

def Eps():
    """The language of just the empty string."""
    return Star(Empty())

def _alt_to_lit_group(rs):
    """Simplify an alternation to a literal group if possible.

    Returns a regular expression equivalent to rs and a simpler flag.
    """
    cs = []
    for r in rs:
        if isinstance(r, Literal):
            cs.append(r.c)
        else:
            return Alternation(rs), False
    return LiteralGroup(cs), True

def _simplify(r):
    if isinstance(r, Literal):
        return r, False
    if isinstance(r, LiteralGroup):
        if len(r.cs) == 0:
            return Empty(), True
        if len(r.cs) == 1:
            return Literal(r.cs[0]), True
        return r, False
    if isinstance(r, Empty):
        return r, False
    if isinstance(r, Star):
        r, simpler = _simplify(r.r)
        return Star(r), simpler
    if isinstance(r, Alternation):
        rs = []
        alt_simpler = False
        for r in r.rs:
            if r.is_empty():
                alt_simpler = True
                continue
            r, simpler = _simplify(r)
            alt_simpler = alt_simpler or simpler
            if isinstance(r, Alternation):
                rs.extend(r.rs)
            else:
                rs.append(r)
        if len(rs) == 0:
            return Empty(), True
        if len(rs) == 1:
            return rs[0], True
        r, simpler = _alt_to_lit_group(rs)
        return r, alt_simpler or simpler
    if isinstance(r, Seq):
        rs = []
        seq_simpler = False
        for r in r.rs:
            if r.is_empty():
                return Empty(), True
            if r.is_eps():
                seq_simpler = True
                continue
            r, simpler = _simplify(r)
            seq_simpler = seq_simpler or simpler
            if isinstance(r, Seq):
                rs.extend(r.rs)
            else:
                rs.append(r)
        if len(rs) == 0:
            return Eps(), True
        if len(rs) == 1:
            return rs[0], True
        return Seq(rs), seq_simpler
    raise ValueError("unexpected regex {}".format(r))

def simplify(r):
    r, simpler = _simplify(r)
    while simpler:
        r, simpler = _simplify(r)
    return r
