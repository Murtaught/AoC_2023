class Symbol:
    def __init__(self, sym, i, j):
        self.sym = sym
        self.i = i
        self.j = j

    def __repr__(self):
        return f"({self.sym}, {self.i}, {self.j})"


class Number:
    def __init__(self, num, i, l, r):
        self.num = num
        self.i = i
        self.l = l
        self.r = r

    def __repr__(self):
        return f"({self.num}, {self.i}, {self.l}...{self.r})"

    def is_next_to(self, i, j):
        return (self.i - 1 <= i and i <= self.i + 1) and \
               (self.l - 1 <= j and j <= self.r)

    def is_next_to_list(self, symbols):
        for s in symbols:
            if self.is_next_to(s.i, s.j):
                return True
        return False


symbols = []
numbers = []

l = None
cur = ''
def add_number(i, j):
    global numbers, l, cur
    if cur:
        assert(l)
        numbers.append(Number(int(cur), i, l, j))
        cur = ''
        l = None


with open('input', 'r') as file:
    for i, line in enumerate(file):
        line = line.rstrip()

        for j, c in enumerate(line):
            if c.isdigit():
                if not l:
                    l = j
                cur += c
            else:
                add_number(i, j)
                if c == '.':
                    continue
                else:
                    symbols.append(Symbol(c, i, j))

        add_number(i, len(line))

# print("symbols:", symbols)
# print("numbers:", numbers)


ans_1 = 0

for number in numbers:
    if number.is_next_to_list(symbols):
        ans_1 += number.num

print('ans (p1):', ans_1)


ans_2 = 0

for symbol in symbols:
    if symbol.sym != '*':
        continue

    adj = []
    for number in numbers:
        if number.is_next_to(symbol.i, symbol.j):
            adj.append(number.num)

    if len(adj) == 2:
        ans_2 += adj[0] * adj[1]

print('ans (p2):', ans_2)




