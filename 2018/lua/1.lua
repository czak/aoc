-- file = io.open('1.in')

-- current = 0
-- seen = {[0] = true}

lines = {}
for n in io.lines("1.in", "n") do
  lines[#lines + 1] = n
end

current = 0
seen = {[0] = true}

while true do
  for i, n in ipairs(lines) do
    current = current + n
    if seen[current] then
      print(current)
      return
    end
    seen[current] = true
  end
end

-- print(current)
