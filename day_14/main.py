import copy

with open('input', 'r') as file:
    fld = list(map(lambda line: list(line.rstrip()), file.readlines()))

n = len(fld)
m = len(fld[0])

def go_north():
    global fld, n, m
    for j in range(m):
        for i in range(n):
            if fld[i][j] != 'O':
                continue
            k = i - 1
            while k >= 0 and fld[k][j] == '.':
                fld[k][j] = 'O'
                fld[k + 1][j] = '.'
                k -= 1

def go_south():
    global fld, n, m
    for j in range(m):
        for i in range(n - 1, -1, -1):
            if fld[i][j] != 'O':
                continue
            k = i + 1
            while k < n and fld[k][j] == '.':
                fld[k][j] = 'O'
                fld[k - 1][j] = '.'
                k += 1

def go_west():
    global fld, n, m
    for i in range(n):
        for j in range(m):
            if fld[i][j] != 'O':
                continue
            k = j - 1
            while k >= 0 and fld[i][k] == '.':
                fld[i][k] = 'O'
                fld[i][k + 1] = '.'
                k -= 1

def go_east():
    global fld, n, m
    for i in range(n):
        for j in range(m - 1, -1, -1):
            if fld[i][j] != 'O':
                continue
            k = j + 1
            while k < m and fld[i][k] == '.':
                fld[i][k] = 'O'
                fld[i][k - 1] = '.'
                k += 1

def count_weight():
    global fld, n, m
    res = 0
    for i in range(n):
        for j in range(m):
            if fld[i][j] == 'O':
                res += n - i
    return res

def show():
    global fld
    return '\n'.join(map(lambda row: ''.join(row), fld))

seen = {}
def cycle(i):
    global fld, n, m
    go_north()
    go_west()
    go_south()
    go_east()
    s = show()
    # print(s, '\n', count_weight(), '\n')
    if s in seen:
        return seen[s] + 1
    seen[s] = i
    return None


saved_fld = copy.deepcopy(fld)
go_north()
print('ans (p1):', count_weight())

fld = saved_fld
seen[show()] = 0

i = 0
ws = [count_weight()]
cycle_start = None
while not cycle_start:
    i += 1
    cycle_start = cycle(i)
    ws.append(count_weight())

cycle_len = len(ws) - cycle_start
ans_2 = ws[cycle_start + (10 ** 9 - cycle_start) % cycle_len]
print('ans (p2):', ans_2)
