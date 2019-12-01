def part1(changes):
    print(sum(changes))

def part2(changes):
    seen = {0}
    current = 0
    while True:
        for n in changes:
            current += n
            if current in seen:
                print(current)
                return
            seen.add(current)

if __name__ == "__main__":
    changes = [int(line) for line in open("1.in")]
    part1(changes)
    part2(changes)
