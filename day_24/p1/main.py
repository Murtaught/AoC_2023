class Vec3D:
    def __init__(self, x, y, z):
        self.x = x
        self.y = y
        self.z = z

    def __repr__(self):
        return f"({self.x}, {self.y}, {self.z})"

    def __str__(self):
        return self.__repr__()

    @staticmethod
    def parse(s):
        x, y, z = map(int, s.split(','))
        return Vec3D(x, y, z)

class Hail:
    def __init__(self, s, d):
        self.s = s
        self.d = d

    def __repr__(self):
        return f"H[{self.s} | {self.d}]"

    def __str__(self):
        return self.__repr__()

class AdHocRange:
    def __init__(self, a, b):
        self.a = a
        self.b = b

    def __contains__(self, x):
        return self.a <= x and x <= self.b

INPUT_FILE = 'input'
TEST_AREA = AdHocRange(200000000000000, 400000000000000) # (7, 27)

hails = []
with open(INPUT_FILE, 'r') as file:
    for line in file:
        s, d = line.rstrip().split(' @ ')
        s = Vec3D.parse(s)
        d = Vec3D.parse(d)
        hails.append(Hail(s, d))

def intersect_2d(a, b):
    dx = b.s.x - a.s.x
    dy = b.s.y - a.s.y
    det = b.d.x * a.d.y - b.d.y * a.d.x
    if det == 0:
        return None

    u = float(dy * b.d.x - dx * b.d.y) / det
    v = float(dy * a.d.x - dx * a.d.y) / det
    if u < 0 or v < 0:
        return None

    return (a.s.x + a.d.x * u, a.s.y + a.d.y * u)

n = len(hails)
ans_1 = 0

for i in range(n):
    for j in range(i):
        a = hails[j]
        b = hails[i]
        p = intersect_2d(a, b)
        # print(f"Intersecting {a} and {b} => {p}")
        if p and (p[0] in TEST_AREA) and (p[1] in TEST_AREA):
            ans_1 += 1

print("ans (p1):", ans_1)


