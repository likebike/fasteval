#!/usr/bin/env python3

import time
import statistics

def calc():
    for i in range(100):
        x = 3.0 * (3.0 + 3.0) / 3.0

stats = []
for i in range(100):
    start = time.time()
    calc()
    stats.append((time.time()-start)*1_000_000_000)

avg = sum(stats)/len(stats)
stdev = statistics.stdev(stats, avg)

print("python_eval_only_100x:", int(avg), "ns  +/-", int(stdev))
print("It's very difficult to estimate the parse time in a fair way.  (Maybe 'time' the run of a PYC?  But that includes many proc startup costs...)")

