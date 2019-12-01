from parse import *
from pprint import pprint

def parse_minute(entry):
    return parse('[{}:{minute:d}]{}', entry).named['minute']

# ------

with open('input4.txt') as f:
    input = sorted(map(str.strip, f.readlines()))

buckets = {}

start = None

for line in input:
    res = parse('{} Guard #{number:d} begins shift', line)
    if res:
        number = res.named['number']
        if number not in buckets:
            bucket = []
            buckets[number] = bucket
        else:
            bucket = buckets[number]
    else:
        if start is None:
            start = parse_minute(line)
        else:
            bucket.append(range(start, parse_minute(line)))
            start = None

def most_popular_minute_asleep(bucket):
    counts = {}
    for period in bucket:
        for minute in period:
            if minute in counts:
                counts[minute] += 1
            else:
                counts[minute] = 1
    if len(counts) == 0:
        return (0, 0)
    return sorted(counts.items(), key=lambda entry: entry[1], reverse=True)[0]

results = [(number, most_popular_minute_asleep(bucket)) for number, bucket in buckets.items()]

pprint(sorted(results, key=lambda entry: entry[1][1], reverse=True))
