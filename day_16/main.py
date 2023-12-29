import copy
from enum import Enum

Dir = Enum('Dir', list('NESW'))

with open('../common/pos.py', 'r') as script:
    exec(script.read())

def go(p, d):
    match d:
        case Dir.N: return Pos(p.i - 1, p.j)
        case Dir.E: return Pos(p.i, p.j + 1)
        case Dir.S: return Pos(p.i + 1, p.j)
        case Dir.W: return Pos(p.i, p.j - 1)

class Light:
    def __init__(self, d, pos):
        self.d = d
        self.pos = pos

    def __repr__(self):
        return f"({self.pos.i}, {self.pos.j}, {self.d})"

    def __str__(self):
        return self.__repr__()

    def __hash__(self):
        return hash((self.d, self.pos))

    def __eq__(self, other):
        return self.d == other.d and self.pos == other.pos

    def __ne__(self, other):
        return not(self == other)

    def move(self, c):
        match c:
            case '.':
                return [Light(self.d, go(self.pos, self.d))]

            case '/':
                match self.d:
                    case Dir.N: new_d = Dir.E
                    case Dir.E: new_d = Dir.N
                    case Dir.S: new_d = Dir.W
                    case Dir.W: new_d = Dir.S
                return [Light(new_d, go(self.pos, new_d))]

            case '\\':
                match self.d:
                    case Dir.N: new_d = Dir.W
                    case Dir.E: new_d = Dir.S
                    case Dir.S: new_d = Dir.E
                    case Dir.W: new_d = Dir.N
                return [Light(new_d, go(self.pos, new_d))]

            case '-':
                if self.d == Dir.E or self.d == Dir.W:
                    return [Light(self.d, go(self.pos, self.d))]
                return [
                    Light(Dir.W, go(self.pos, Dir.W)),
                    Light(Dir.E, go(self.pos, Dir.E))
                ]

            case '|':
                if self.d == Dir.N or self.d == Dir.S:
                    return [Light(self.d, go(self.pos, self.d))]
                return [
                    Light(Dir.N, go(self.pos, Dir.N)),
                    Light(Dir.S, go(self.pos, Dir.S))
                ]

            case _:
                raise RuntimeError(f"Unexpected symbol '{c}'!")


with open('input', 'r') as file:
    fld = list(map(lambda line: list(line.rstrip()), file.readlines()))

n = len(fld)
m = len(fld[0])

def solve(start):
    global fld, n, m

    visited = set()
    visited.add(start)

    curs = [start]
    nxts = []

    while len(curs) > 0:
        for cur in curs:
            assert(cur.pos.i >= 0)
            assert(cur.pos.j >= 0)
            assert(cur.pos.i < n)
            assert(cur.pos.j < m)

            c = fld[cur.pos.i][cur.pos.j]
            for nxt in cur.move(c):
                if nxt.pos.i < 0 or nxt.pos.i >= n or nxt.pos.j < 0 or nxt.pos.j >= m:
                    continue
                if nxt not in visited:
                    visited.add(nxt)
                    nxts.append(nxt)

        curs = nxts
        nxts = []

    # Done!
    seen_positions = set()
    for light in visited:
        seen_positions.add(light.pos)

    return len(seen_positions)

print('ans (p1):', solve(Light(Dir.E, Pos(0, 0))))

best = 0

for i in range(n):
    best = max(best, solve(Light(Dir.E, Pos(i, 0))))
    best = max(best, solve(Light(Dir.W, Pos(i, m - 1))))

for j in range(m):
    best = max(best, solve(Light(Dir.S, Pos(0, j))))
    best = max(best, solve(Light(Dir.N, Pos(n - 1, j))))

print('ans (p2):', best)


