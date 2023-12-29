import sympy as sp

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

hails = []
with open('input', 'r') as file:
    for line in file:
        s, d = line.rstrip().split(' @ ')
        s = Vec3D.parse(s)
        d = Vec3D.parse(d)
        hails.append(Hail(s, d))

n = len(hails)

# Initial position and vector for our throw.
# These all are unknown variables.
x, y, z, dx, dy, dz = sp.symbols("x y z dx dy dz", integer=True)
symbols = [x, y, z, dx, dy, dz]
equations = []

# It seems to be important to minimize the number of equations and
# unknown variables.
for i, hail in enumerate(hails):
    s = hail.s
    d = hail.d
    equations.append(sp.Eq( (dy - d.y) * (s.x - x), (dx - d.x) * (s.y - y) ))
    equations.append(sp.Eq( (dz - d.z) * (s.x - x), (dx - d.x) * (s.z - z) ))

solutions = sp.solve(equations, *symbols, dict=True)
for i, sol in enumerate(solutions):
    print(f"Solution #{i + 1} / {len(solutions)}:")
    print(f"    x  = {sol[x]}, y  = {sol[y]}, z  = {sol[z]}")
    print(f"    dx = {sol[dx]}, dy = {sol[dy]}, dz = {sol[dz]}")
    print("ans (p2):", sol[x] + sol[y] + sol[z])


