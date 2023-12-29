import copy, re, sys
from enum import Enum

Dir = Enum('Dir', list('URDL'))

class Pos:
    def __init__(self, i, j):
        self.i = i
        self.j = j

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


def go(p, d):
    match d:
        case Dir.U: return Pos(p.i - 1, p.j)
        case Dir.R: return Pos(p.i, p.j + 1)
        case Dir.D: return Pos(p.i + 1, p.j)
        case Dir.L: return Pos(p.i, p.j - 1)

RE = re.compile(r'^([URDL]) (\d+) \((#\w{6,6})\)$')

moves = []
with open('../input', 'r') as file:
    for line in file:
        m = RE.match(line.rstrip())
        assert(m)
        d = Dir[m.group(1)]
        steps = int(m.group(2))
        color = m.group(3)
        moves.append((d, steps, color))


contour = set()

pos = Pos(0, 0)
contour.add(pos)

for (d, steps, color) in moves:
    for _ in range(steps):
        pos = go(pos, d)
        contour.add(pos)

INF = 10**12
min_i = INF
min_j = INF
max_i = -INF
max_j = -INF
for pos in contour:
    min_i = min(pos.i, min_i)
    min_j = min(pos.j, min_j)
    max_i = max(pos.i, max_i)
    max_j = max(pos.j, max_j)

print('Contour length:', len(contour))

# That's cool, but let's actually solve the problem now.
n = max_i - min_i + 1
m = max_j - min_j + 1
fld = [None] * n
for i in range(n):
    fld[i] = [0] * m
    for j in range(m):
        p = Pos(min_i + i, min_j + j)
        fld[i][j] = 1 if p in contour else 0

def flood(pos):
    v = fld[pos.i][pos.j]
    if v > 0:
        return

    fld[pos.i][pos.j] = 1

    flood(Pos(pos.i - 1, pos.j))
    flood(Pos(pos.i + 1, pos.j))
    flood(Pos(pos.i, pos.j - 1))
    flood(Pos(pos.i, pos.j + 1))

entry_point = None
for i in range(1, n):
    for j in range(1, m):
        if (not fld[i][j]) and fld[i][j - 1]:
            if (not entry_point) or (i < entry_point.i) or (i == entry_point.i and j < entry_point.j):
                entry_point = Pos(i, j)

with open('pretty', 'wt') as out:
    for i in range(n):
        for j in range(m):
            print('#' if fld[i][j] else '.', end='', file=out)
        print(file=out)

sys.setrecursionlimit(1000000)
flood(entry_point)

ans_1 = 0
for i in range(n):
    for j in range(m):
        if fld[i][j]:
            ans_1 += 1

print('ans (p1):', ans_1)





