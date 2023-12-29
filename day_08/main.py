import re, math
from functools import reduce

instrs = None
nodes = {}

NODES_REGEX = re.compile(r'^(\w+) = \((\w+), (\w+)\)$')

with open('input', 'r') as file:
    instrs = list(map(lambda c: 0 if c == 'L' else 1, file.readline().rstrip()))
    empty_line = file.readline().rstrip()
    assert empty_line == ''
    for line in file:
        line = line.rstrip()
        m = NODES_REGEX.match(line)
        assert(m)
        nodes[m.group(1)] = (m.group(2), m.group(3))


if 'AAA' in nodes:
    i = 0
    cur = 'AAA'
    steps = 0

    while cur != 'ZZZ':
        cur = nodes[cur][instrs[i]]
        # print('->', cur)
        steps += 1
        i = (i + 1) % len(instrs)

    print('ans (p1):', steps)


# Part 2:
# Brute force approach takes too long.
# We are probably expected to compute LCM instead.
def brute_force():
    i = 0
    cur = set(filter(lambda key: key.endswith('A'), nodes.keys()))
    steps = 0

    def finished():
        for key in cur:
            if not key.endswith('Z'):
                return False
        return True

    while not finished():
        cur = set(map(lambda key: nodes[key][instrs[i]], cur))
        # print('->', cur)
        steps += 1
        i = (i + 1) % len(instrs)

    print('ans (p2):', steps)


cycle_lens = []

def analyze(start):
    # print(f"# Analyzing '{start}':", flush=True)

    i = 0
    cur = start
    steps = 0
    cache = {}

    end_at = None

    while True:
        if cur.endswith('Z'):
            end_at = steps
            # print(f"  Found {cur} at {end_at}")

        ckey = (cur, i)
        if ckey in cache:
            cval = cache[ckey]
            cycle_len = steps - cval
            # It just so happens to be true.
            # It makes the problem so much easier!
            assert cycle_len == end_at
            # print(f"  Found cycle! {start} -{cval}-> {cur} -{cycle_len}-> {cur}")
            return cycle_len
        
        cache[ckey] = steps

        cur = nodes[cur][instrs[i]]
        steps += 1
        i = (i + 1) % len(instrs)


for key in nodes:
    if key.endswith('A'):
        cycle_lens.append(analyze(key))

def lcm(a, b):
    return (a * b) // math.gcd(a, b)

print('ans (p2):', reduce(lcm, cycle_lens))
