import re

class Pick:
    def __init__(self, r, g, b):
        self.r = r
        self.g = g
        self.b = b

    def lte(self, other):
        return (self.r <= other.r) and \
               (self.g <= other.g) and \
               (self.b <= other.b)

    def max(self, other):
        return Pick(
            max(self.r, other.r),
            max(self.g, other.g),
            max(self.b, other.b)
        )

    def power(self):
        return self.r * self.g * self.b

    def __str__(self):
        return f"({self.r}, {self.g}, {self.b})"

    def __repr__(self):
        return self.__str__()

class Game:
    def __init__(self, id, picks):
        self.id = id
        self.picks = picks

        self.max = Pick(0, 0, 0)
        for p in self.picks:
            self.max = p.max(self.max)

    def __str__(self):
        return f"Game(ID = {self.id}, {self.picks})"

PREFIX = re.compile(r'^Game (\d+): (.*)$')
PICK = re.compile(r'^(\d+) (red|blue|green)$')

def parse_game(s):
    m = PREFIX.match(s)
    assert m
    id = int(m.group(1))
    picks = []
    for pp in m.group(2).split(';'):
        cur = Pick(0, 0, 0)
        for p in pp.strip().split(','):
            p = p.strip()
            m = PICK.match(p)
            assert m
            value = int(m.group(1))
            color = m.group(2)
            if color == 'red':
                cur.r += value
            elif color == 'green':
                cur.g += value
            else:
                assert color == 'blue'
                cur.b += value
        picks.append(cur)

    return Game(id, picks)

games = []
with open('input', 'r') as file:
    for line in file:
        line = line.rstrip()
        game = parse_game(line)
        games.append(game)

BAG = Pick(12, 13, 14)

ans_1 = 0
ans_2 = 0
for game in games:
    if all(map(lambda p: p.lte(BAG), game.picks)):
        ans_1 += game.id
    ans_2 += game.max.power()

print('ans (p1):', ans_1)
print('ans (p2):', ans_2)
