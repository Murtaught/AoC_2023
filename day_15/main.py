import copy

with open('input', 'r') as file:
    ss = file.read().strip().split(',')

def hash(s):
    h = 0
    for c in s:
        h = (h + ord(c)) * 17 % 256
    return h

ans_1 = 0
for s in ss:
    ans_1 += hash(s)
print('ans (p1):', ans_1)

class Lens:
    def __init__(self, label, fl):
        self.label = label
        self.fl = int(fl)

    def __repr__(self):
        return f"{self.label} {self.fl}"

boxes = []
for _ in range(256):
    boxes.append([])

def box_replace_or_add(box, new_lens):
    for i, lens in enumerate(box):
        if lens.label == new_lens.label:
            box[i] = new_lens
            return
    box.append(new_lens)

def box_remove(box, label):
    for i, lens in enumerate(box):
        if lens.label == label:
            box.pop(i)
            return

for s in ss:
    if '=' in s:
        label, fl = s.split('=')
        lens = Lens(label, fl)
        box = boxes[hash(label)]
        box_replace_or_add(box, lens)
    else:
        assert(s[-1] == '-')
        label = s[:-1]
        box = boxes[hash(label)]
        box_remove(box, label)

    # print(f"After \"{s}\":")
    # for i, box in enumerate(boxes):
        # if len(box) == 0:
            # continue
        # print(f"Box {i}: {box}")

ans_2 = 0
for i, box in enumerate(boxes):
    for j, lens in enumerate(box):
        ans_2 += (1 + i) * (1 + j) * lens.fl
print('ans (p2):', ans_2)

