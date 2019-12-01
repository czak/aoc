import strutils, sequtils, intsets, math

func part1(changes: seq[int]): int =
  changes.sum

func part2(changes: seq[int]): int =
  var
    seen = initIntSet()
    current = 0

  while true:
    for n in changes:
      current += n
      if current in seen:
        return current
      seen.incl(current)

when isMainModule:
  let changes = toSeq("inputs/1.in".lines).map(parseInt)

  echo part1(changes)
  echo part2(changes)
