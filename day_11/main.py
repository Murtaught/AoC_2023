import copy

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


points = []

with open('input', 'r') as file:
    for i, line in enumerate(file):
        line = line.rstrip()
        for j, c in enumerate(line):
            if c == '#':
                points.append(Pos(i, j))

def expand(points, getter, setter, times):
    points.sort(key=getter)

    prev = None
    acc = 0

    for p in points:
        cur = getter(p)
        if not isinstance(prev, int):
            prev = cur
            continue
        
        d = max(0, cur - prev - 1)
        acc += d * (times - 1)
        prev = cur
        setter(p, cur + acc)

def solve(points, times):
    expand(points, lambda p: p.i, Pos.set_i, times)
    expand(points, lambda p: p.j, Pos.set_j, times)

    ans = 0
    for i in range(len(points)):
        for j in range(i):
            ans += points[i].manhattan_dist(points[j])

    return ans

print('ans (p1):', solve(copy.deepcopy(points), 2))
print('ans (p2):', solve(copy.deepcopy(points), 1000000))
