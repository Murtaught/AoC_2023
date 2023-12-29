def predict(xs):
    if all(map(lambda x: x == 0, xs)):
        return (0, 0)

    ds = list(map(lambda p: p[1] - p[0], zip(xs, xs[1:])))
    (p, n) = predict(ds)

    return (xs[0] - p, xs[-1] + n)


ans_1 = 0
ans_2 = 0

with open('input', 'r') as file:
    for line in file:
        xs = list(map(int, line.rstrip().split()))
        (p, n) = predict(xs)
        ans_1 += n
        ans_2 += p

print('ans (p1):', ans_1)
print('ans (p2):', ans_2)


