def points(count):
    if count <= 0:
        return 0
    return 2 ** (count - 1)

ans_1 = 0
copies_cnt = {}

def increment(card, amount):
    # print(f'+> {card}: +{amount}')
    global copies_cnt
    if card not in copies_cnt:
        copies_cnt[card] = 0
    copies_cnt[card] += amount

with open('input', 'r') as file:
    for line in file:
        line = line.rstrip().split()
        assert line[0] == 'Card'
        assert line[1].endswith(':')

        card = line[1]
        card = int(card[0:len(card) - 1])

        increment(card, 1)

        winning = set()
        seen_pipe = False
        count = 0

        for number in line[2:]:
            if number == '|':
                seen_pipe = True
            else:
                number = int(number)
                if not seen_pipe:
                    winning.add(number)
                else:
                    if number in winning:
                        count += 1

        for i in range(card + 1, card + count + 1):
            increment(i, copies_cnt[card])

        ans_1 += points(count)

print('ans (p1):', ans_1)

ans_2 = 0
for count in copies_cnt.values():
    ans_2 += count

print('ans (p2):', ans_2)




