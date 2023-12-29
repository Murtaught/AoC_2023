import re


class Value:
    def __init__(self, cat, val):
        self.cat = cat
        self.val = val

    def __repr__(self):
        return f"{self.cat}[{self.val}]"

    def __str__(self):
        return self.__repr__()


class Range:
    def __init__(self, d, l, r):
        self.d = d
        self.l = l
        self.r = r

    def __repr__(self):
        return f"{self.d} <- ({self.l}..{self.r})"

    def __str__(self):
        return self.__repr__()

    def __contains__(self, val):
        return self.l <= val and val <= self.r

    def map(self, val):
        assert val in self
        return self.d + val - self.l


class Mapping:
    def __init__(self, cat_1, cat_2):
        self.cat_1 = cat_1
        self.cat_2 = cat_2
        self.ranges = []

    def __repr__(self):
        return f"Mapping from {self.cat_1} to {self.cat_2}: {{{self.ranges}}}"

    def __str__(self):
        return self.__repr__()

    def add_range(self, d, l, r):
        self.ranges.append(Range(d, l, r))

    def map_value(self, value):
        assert value.cat == self.cat_1
        for rng in self.ranges:
            if value.val in rng:
                return Value(self.cat_2, rng.map(value.val))
        return Value(self.cat_2, value.val)


values = []
mappings = []

SEED_RE = re.compile(r'^seeds: ([\d\s]+)$')
MAP_RE = re.compile(r'^(\w+)-to-(\w+) map:$')

with open('input', 'r') as file:
    for line in file:
        line = line.rstrip()

        if line == '':
            continue

        m = SEED_RE.match(line)
        if m:
            for val in map(int, m.group(1).split()):
                values.append(Value('seed', val))
            continue

        m = MAP_RE.match(line)
        if m:
            cat_1 = m.group(1)
            cat_2 = m.group(2)
            # print(f"Found mapping from {cat_1} to {cat_2}...")
            mappings.append(Mapping(cat_1, cat_2))
            continue

        d, l, r = list(map(int, line.split()))
        mappings[-1].add_range(d, l, l + r)



# Let's solve!
# We assume that the order is right.
# print(values)
for mapping in mappings:
    # print('Using', mapping, '...')
    values = list(map(lambda value: mapping.map_value(value), values))
    # print(values)

print('ans (p1):', min(map(lambda value: value.val, values)))
