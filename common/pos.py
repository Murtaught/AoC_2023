class Pos:
    def __init__(self, i, j):
        self.i = i
        self.j = j

    def set_i(self, v):
            self.i = v

    def set_j(self, v):
            self.j = v

    def manhattan_dist(self, other):
        return abs(self.i - other.i) + abs(self.j - other.j)

    def __repr__(self):
        return f"({self.i}, {self.j})"

    def __str__(self):
        return self.__repr__()

    def __hash__(self):
        return hash((self.i, self.j))

    def __eq__(self, other):
        return self.i == other.i and self.j == other.j

    def __ne__(self, other):
        return not(self == other)
