import copy

verbose = False
verboseprint = print if verbose else lambda *a, **k: None

def find_h_splits(pattern):
    ans = []
    for i in range(1, len(pattern)):
        # `i` is the first row reflected down.
        # It is also the amount of rows that we need to check.
        failed = False
        for j in range(min(i, len(pattern) - i)):
            if pattern[i - 1 - j] != pattern[i + j]:
                failed = True
                break

        if not failed:
            ans.append(i)

    return ans

def transpose(pattern):
    return list(map(list, zip(*pattern)))

def solve(pattern):
    sols = []
    for i in find_h_splits(pattern):
        sols.append(i * 100)

    for j in find_h_splits(transpose(pattern)):
        sols.append(j)
    
    return sols


def flip(c):
    if c == '.':
        return '#'
    else:
        assert(c == '#')
        return '.'

def mut_solve(pattern):
    old = solve(copy.deepcopy(pattern))
    assert(len(old) == 1)
    old = old[0]

    found_sols = set()
    for i in range(len(pattern)):
        for j in range(len(pattern[i])):
            pattern[i][j] = flip(pattern[i][j])
            solutions = list(filter(lambda s: s != old, solve(pattern)))
            if len(solutions) > 0:
                assert(len(solutions) == 1)
                return solutions[0]
            pattern[i][j] = flip(pattern[i][j])

    ljoin = lambda xs: ''.join(xs)
    print('pattern:', '\n'.join(map(ljoin, pattern)), '\ntpattern:', '\n'.join(map(ljoin, transpose(pattern))), sep='\n')
    print('old: ', old, 'found sols:', found_sols)
    assert(False)


ans_1 = 0
ans_2 = 0

with open('input', 'r') as file:
    pattern = []
    for line in file:
        line = line.rstrip()
        if line == '':
            ans_1 += solve(pattern)[0]
            ans_2 += mut_solve(pattern)
            pattern = []
        else:
            pattern.append(list(line))

    if len(pattern) > 0:
        ans_1 += solve(pattern)[0]
        ans_2 += mut_solve(pattern)
        pattern = []

print('ans (p1):', ans_1)
print('ans (p2):', ans_2)
# print('ans (p2):', ans_2)

