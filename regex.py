class Regex:
    def __eq__(self, other):
        if isinstance(other, self.__class__):
            return self.__dict__ == other.__dict__
        else:
            return False

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
        if self.is_eps():
            return "(?:)"
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
        sub_res = []
        for r in self.rs:
            if r.is_eps():
                sub_res.append("")
            else:
                sub_res.append(r.to_re())
        return "(?:{})".format("|".join(sub_res))

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
        return "(?:{})".format("".join([r.to_re() for r in self.rs]))

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

def _alt_common_prefix(rs):
    """Find a common prefix among two elements of rs."""
    for r in rs:
        if isinstance(r, Seq):
            new_rs = _alt_common_prefix_with(r.rs[0], rs)
            if new_rs is not None:
                return new_rs, True
        new_rs = _alt_common_prefix_with(r, rs)
        if new_rs is not None:
            return new_rs, True
    return rs, False

def _alt_common_prefix_with(prefix, rs):
    """Extract elements of rs that begin with prefix.

    Returns an rs with prefix factored out if more than one element matches,
    and None otherwise.
    """
    other_rs = []
    suffixes = []
    for r in rs:
        if r == prefix:
            suffixes.append(Eps())
            continue
        if isinstance(r, Seq):
            if r.rs[0] == prefix:
                suffixes.append(Seq(r.rs[1:]))
                continue
        other_rs.append(r)
    if len(suffixes) > 1:
        new_rs = other_rs
        new_rs.append(Seq([prefix, Alternation(suffixes)]))
        return new_rs
    return None

def _simplify(r):
    """Simplify a regular expression.

    Returns a tuple (r_new, simpler) where r_new is equivalent to r but
    possibly with some simplifications. simpler is a boolean that reports if
    any simplifications were made.

    This is a low-level function that implements one step of simplification.
    """
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
        # common prefix elimination is disabled because it's quite slow
        #rs, simpler = _alt_common_prefix(rs)
        #alt_simpler = alt_simpler or simpler
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
    """Simplify a regular expression.

    Simplifications include:
    - removing Empty as much as possible
    - removing the epsilon language (Empty*, which only matches the zero-length string) as much as possible
    - flattening ORs to a single level (since Alternation takes a list)
    - flattening sequences to a single level (since Seq takes a list)
    - using LiteralGroups ([abc] in normal regex syntax) instead of an OR of literals
    - unwrapping sequences and ORs of single regexes
    """
    r, simpler = _simplify(r)
    while simpler:
        r, simpler = _simplify(r)
    return r
