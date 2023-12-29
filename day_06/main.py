from functools import reduce
from operator import mul

class Race:
    def __init__(self, time, dist):
        self.time = int(time)
        self.dist = int(dist)

    def __repr__(self):
        return f"Race(t: {self.time}, d: {self.dist})"

    def simulate(self, push_time):
        return max(0, self.time - push_time) * push_time

    def ways_to_win(self):
        count = 0
        for push_time in range(1, self.time):
            if self.simulate(push_time) > self.dist:
                count += 1
        return count


races = None
master_race = None

with open('input', 'r') as file:
    times = file.readline().rstrip().split()[1:]
    distances = file.readline().rstrip().split()[1:]
    races = [Race(t, d) for (t, d) in zip(times, distances)]
    master_race = Race(''.join(times), ''.join(distances))

ans_1 = reduce(mul, map(lambda r: r.ways_to_win(), races), 1)
print('ans_p1:', ans_1)

ans_2 = master_race.ways_to_win()
print('ans_p2:', ans_2)

