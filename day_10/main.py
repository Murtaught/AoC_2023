import io, sys, collections
from enum import Enum

#print(sys.getrecursionlimit())
sys.setrecursionlimit(100000)


class Dir(Enum):
    U = 1
    R = 2
    D = 3
    L = 4


def pretty_symbol(c, d):
    match c:
        case '-':
            match d:
                case Dir.L: return '←'
                case Dir.R: return '→'
                case None: return '─'
                case _: raise RuntimeError("Unexpected dir!")
        case '|':
            match d:
                case Dir.U: return '↑'
                case Dir.D: return '↓'
                case None: return '│'
                case _: raise RuntimeError("Unexpected dir!")
        case '7':
            return '┐'
        case 'J':
            return '┘'
        case 'L':
            return '└'
        case 'F':
            return '┌'
        case _:
            return c


def next_dir(d, c):
    match (d, c):
        case (Dir.U, '|'): return Dir.U
        case (Dir.U, '7'): return Dir.L
        case (Dir.U, 'F'): return Dir.R
        case (Dir.R, '-'): return Dir.R
        case (Dir.R, '7'): return Dir.D
        case (Dir.R, 'J'): return Dir.U
        case (Dir.D, '|'): return Dir.D
        case (Dir.D, 'J'): return Dir.L
        case (Dir.D, 'L'): return Dir.R
        case (Dir.L, '-'): return Dir.L
        case (Dir.L, 'L'): return Dir.U
        case (Dir.L, 'F'): return Dir.D
        case _:
            raise RuntimeError(f"Unexpected pair ({d}, {c})!")


def go(p, d):
    match d:
        case Dir.U: return Pos(p.i - 1, p.j)
        case Dir.R: return Pos(p.i, p.j + 1)
        case Dir.D: return Pos(p.i + 1, p.j)
        case Dir.L: return Pos(p.i, p.j - 1)


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


def next_positions(p, c):
    match c:
        case '-':
            return [Pos(p.i, p.j - 1), Pos(p.i, p.j + 1)]
        case '|':
            return [Pos(p.i - 1, p.j), Pos(p.i + 1, p.j)]
        case '7':
            return [Pos(p.i, p.j - 1), Pos(p.i + 1, p.j)]
        case 'J':
            return [Pos(p.i - 1, p.j), Pos(p.i, p.j - 1)]
        case 'L':
            return [Pos(p.i - 1, p.j), Pos(p.i, p.j + 1)]
        case 'F':
            return [Pos(p.i, p.j + 1), Pos(p.i + 1, p.j)]
        case 'S':
            return [
                Pos(p.i, p.j - 1),
                Pos(p.i, p.j + 1),
                Pos(p.i - 1, p.j),
                Pos(p.i + 1, p.j)
            ]
        case _:
            raise RuntimeError(f"Unexpected character '{c}'!")


class Field:
    def __init__(self):
        self.n = 0
        self.m = 0
        self.fld = []
        self.start_pos = None
        self.main_loop = set()
        self.dirs = {}
        self.internal = set()

    def __repr__(self):
        with io.StringIO() as out:
            print(f"Field[{self.n}x{self.m}]:", file=out)
            for i in range(self.n):
                for j in range(self.m):
                    if Pos(i, j) == self.start_pos:
                        print('S', end='', file=out)
                    elif Pos(i, j) in self.internal:
                        print('I', end='', file=out)
                    else:
                        print(pretty_symbol(self.fld[i][j], self.dirs.get(Pos(i, j))), end='', file=out)
                print(file=out)
            return out.getvalue()

    def __str__(self):
        return self.__repr__()

    def find_start_pos(self):
        for i in range(self.n):
            for j in range(self.m):
                if self.fld[i][j] == 'S':
                    return Pos(i, j)
        return None

    def __contains__(self, pos):
        return pos.i >= 0 and \
               pos.j >= 0 and \
               pos.i < self.n and \
               pos.j < self.m

    def compute_distances_from(self, start):
        queue = collections.deque()
        queue.append(start)

        dist = {}
        dist[start] = 0

        while len(queue) > 0:
            pos = queue.popleft()
            # print('pop', pos)

            if pos not in self:
                continue

            # print('c =', c)
            c = self.fld[pos.i][pos.j]
            if c == '.':
                continue

            d = dist[pos]
            for nxt in next_positions(pos, c):
                if nxt not in dist:
                    dist[nxt] = d + 1
                    queue.append(nxt)

        return dist

    
    def cleanup(self):
        for i in range(self.n):
            for j in range(self.m):
                if Pos(i, j) not in self.main_loop:
                    self.fld[i][j] = '.'


    def go_around(self, start_pos, start_dir):
        dirs = {}

        dirs[start_pos] = start_dir
        pos = go(start_pos, start_dir)
        d = next_dir(start_dir, self.fld[pos.i][pos.j])

        while pos != start_pos:
            dirs[pos] = d
            pos = go(pos, d)
            d = next_dir(d, self.fld[pos.i][pos.j])

        return dirs


    def check_internal(self, p):
        if (go(p, Dir.U) not in self.dirs) and \
           (go(p, Dir.R) not in self.dirs) and \
           (go(p, Dir.D) not in self.dirs) and \
           (go(p, Dir.L) not in self.dirs):
               return False

        return (self.dirs.get(go(p, Dir.U)) in [Dir.L, None]) and \
               (self.dirs.get(go(p, Dir.R)) in [Dir.U, None]) and \
               (self.dirs.get(go(p, Dir.D)) in [Dir.R, None]) and \
               (self.dirs.get(go(p, Dir.L)) in [Dir.D, None])


    def fill_internal(self, p):
        if (p not in self) or (p in self.internal):
            return

        c = self.fld[p.i][p.j]
        if c != '.':
            return

        self.internal.add(p)

        self.fill_internal(go(p, Dir.U))
        self.fill_internal(go(p, Dir.R))
        self.fill_internal(go(p, Dir.D))
        self.fill_internal(go(p, Dir.L))


    def find_internal(self):
        for i in range(self.n):
            for j in range(self.m):
                if self.fld[i][j] == '.':
                    if self.check_internal(Pos(i, j)):
                        self.fill_internal(Pos(i, j))


    def count_external(self):
        count = 0
        for i in range(self.n):
            for j in range(self.m):
                if (self.fld[i][j] == '.') and (Pos(i, j) not in self.internal):
                    count += 1
        return count

    # State machine.
    def find_internal_sm(self):
        self.internal = set()
        State = Enum('State', ['OUT', 'E1', 'IN', 'E2'])
        for i in range(self.n):
            state = State.OUT
            enter = None
            for j in range(self.m):
                c = self.fld[i][j]
                match state:
                    case State.OUT:
                        if c == '|':
                            state = State.IN
                        elif c == 'L' or c == 'F':
                            state = State.E1
                            enter = c
                        else:
                            assert(c == '.')

                    case State.E1:
                        if c == '7':
                            state = State.IN if enter == 'L' else State.OUT
                            enter = None
                        elif c == 'J':
                            state = State.IN if enter == 'F' else State.OUT
                            enter = None
                        else:
                            assert(c == '-')

                    case State.IN:
                        if c == '|':
                            state = State.OUT
                        elif c == 'L' or c == 'F':
                            state = State.E2
                            enter = c
                        else:
                            assert(c == '.')
                            self.internal.add(Pos(i, j))

                    case State.E2:
                        if c == '7':
                            state = State.OUT if enter == 'L' else State.IN
                            enter = None
                        elif c == 'J':
                            state = State.OUT if enter == 'F' else State.IN
                            enter = None
                        else:
                            assert(c == '-')


    @staticmethod
    def read_from_file(filename):
        field = Field()
        with open(filename, 'r') as file:
            for line in file:
                field.fld.append(list(line.rstrip()))
        field.n = len(field.fld)
        field.m = len(field.fld[0])
        field.start_pos = field.find_start_pos()

        # Hack. Works only for my input.
        field.fld[field.start_pos.i][field.start_pos.j] = '-'

        return field
            


field = Field.read_from_file('input')

dist = field.compute_distances_from(field.start_pos)
field.main_loop = set(dist.keys())
field.cleanup()

print('ans (p1):', max(dist.values()))

# field.dirs = field.go_around(field.start_pos, Dir.R)
# field.find_internal()

field.find_internal_sm()

print(field)

print('ans (p2):', len(field.internal))
print('(external: ', field.count_external(), ')', sep='')
