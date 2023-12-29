import copy

verbose = False
verboseprint = print if verbose else lambda *a, **k: None


def split_to_xs(s):
    s = ''.join(s)
    return list(map(lambda t: len(t), filter(lambda t: t != '', s.split('.'))))

def solve_brute(s, ys):
    ans = 0
    s = list(s)
    indices = []
    for i, c in enumerate(s):
        if c == '?':
            indices.append(i)
    
    for mask in range(1 << len(indices)):
        for i in range(len(indices)):
            c = '#' if ((mask >> i) & 1) == 1 else '.'
            s[indices[i]] = c

        if split_to_xs(s) == ys:
            ans += 1

    return ans


# DP with memoization approach.
def solve(s, ys):
    if not s.endswith('.'):
        s = s + '.'

    verboseprint(f"solve('{s}', {ys})")

    # `i` - left index for `s`.
    # `j` - left index for `ys`.
    # `acc` - current running length of '#'s.
    def rec(i, j, acc, d):
        if i >= len(s):
            assert(acc == 0)
            return 1 if j >= len(ys) else 0

        c = s[i]
        verboseprint(f"  {'>' * d} rec(i = {i} ({c}), j = {j}, acc = {acc})")

        if c == '.':
            if acc > 0:
                if j >= len(ys) or acc != ys[j]:
                    return 0
                return cache_rec(i + 1, j + 1, 0, d + 1)
            return cache_rec(i + 1, j, 0, d + 1)

        if c == '#':
            return cache_rec(i + 1, j, acc + 1, d + 1)

        assert(c == '?')

        var_1 = 0
        var_2 = 0

        if j < len(ys):
            if acc < ys[j]:
                var_1 += cache_rec(i + 1, j, acc + 1, d + 1)
            elif acc == ys[j]:
                var_2 += cache_rec(i + 1, j + 1, 0, d + 1)

        if acc == 0:
            var_2 += cache_rec(i + 1, j, 0, d + 1)

        verboseprint(f"  {'>' * d} {var_1} + {var_2} = {var_1 + var_2}")

        return var_1 + var_2

    cache = {}
    def cache_rec(i, j, acc, d):
        ckey = (i, j, acc)
        if ckey in cache:
            verboseprint(f"  {'>' * d} cache hit!")
            return cache[ckey]

        ans = rec(i, j, acc, d)
        cache[ckey] = ans
        return ans

    return cache_rec(0, 0, 0, 1)


ans_1 = 0
ans_2 = 0

with open('input', 'r') as file:
    for line_no, line in enumerate(file):
        s, ys = line.rstrip().split(' ')
        ys = list(map(int, ys.split(',')))
        ans_1 += solve(s, ys)
        ans_2 += solve('?'.join([s] * 5), ys * 5)
        percent = 100.0 * (line_no + 1) / 1000.0
        verboseprint(f"solved line {line_no + 1} / 1000 ({percent:3.2f})", flush=True)

print('ans (p1):', ans_1)
print('ans (p2):', ans_2)

