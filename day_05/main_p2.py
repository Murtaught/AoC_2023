import re
from ranges import Range, RangeSet

class TargetRange:
    def __init__(self, target, rng):
        self.target = target
        self.rng = rng

    def __repr__(self):
        return f"{self.target} <- {self.rng}"

    def __str__(self):
        return self.__repr__()


class Mapping:
    def __init__(self):
        self.ranges = []

    def add_range(self, d, l, r):
        self.ranges.append(TargetRange(d, Range(l, r)))

    def map_range_set(self, s):
        # print(f"Mapping {s} over {self.ranges} ...")
        nxt = RangeSet()
        for trng in self.ranges:
            inter = s.intersection(trng.rng)
            # print(f"  Intersection of {s} and {trng.rng} is {inter}")
            if inter.isempty():
                continue

            for irng in inter:
                start = irng.start - trng.rng.start + trng.target
                len = irng.end - irng.start
                nxt.add(Range(start, start + len))

            s = s.difference(inter)

        nxt.add(s)
        # print('nxt:', nxt)
        return nxt


seeds = []
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
            xs = list(map(int, m.group(1).split()))
            for i in range(1, len(xs), 2):
                seeds.append(Range(xs[i - 1], xs[i - 1] + xs[i]))
            continue

        m = MAP_RE.match(line)
        if m:
            cat_1 = m.group(1)
            cat_2 = m.group(2)
            # print(f"Found mapping from {cat_1} to {cat_2}...")
            mappings.append(Mapping())
            continue

        d, l, r = list(map(int, line.split()))
        mappings[-1].add_range(d, l, l + r)


seeds = RangeSet(seeds)
for mapping in mappings:
    seeds = mapping.map_range_set(seeds)

ans_2 = None
for rng in seeds:
    if (not ans_2) or (rng.start < ans_2):
        ans_2 = rng.start

print('ans (p2):', ans_2)
