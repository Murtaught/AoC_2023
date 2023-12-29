
import re

spelled = {
    'one': 1,
    'two': 2,
    'three': 3,
    'four': 4,
    'five': 5,
    'six': 6,
    'seven': 7,
    'eight': 8,
    'nine': 9
}

RE_1 = re.compile(r'(\d)')
RE_2 = re.compile(r'(?=(\d|one|two|three|four|five|six|seven|eight|nine))')

def process(e):
    if e.isdigit():
        return int(e)
    return spelled[e]

def find_all(s, regex):
    return [process(m.group(1)) for m in re.finditer(regex, s)]

def get_addendum(numbers):
    return numbers[0] * 10 + numbers[-1]

ans_1 = 0
ans_2 = 0

with open('input', 'r') as file:
    for line in file:
        line = line.rstrip()
        ans_1 += get_addendum(find_all(line, RE_1))
        ans_2 += get_addendum(find_all(line, RE_2))

print('ans (p1):', ans_1)
print('ans (p2):', ans_2)

