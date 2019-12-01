-- input = {"abcdef", "bababc", "abbcde", "abcccd", "aabcdd", "abcdee", "ababab"}

-- part 1

num2 = 0
num3 = 0

for word in io.lines('2.in') do
  counts = {}
  for c = 1, #word do
    char = word:byte(c)
    if counts[char] then
      counts[char] = counts[char] + 1
    else
      counts[char] = 1
    end
  end

  has2 = false
  has3 = false

  for _, v in pairs(counts) do
    if v == 2 then has2 = true end
    if v == 3 then has3 = true end
  end

  if has2 then num2 = num2 + 1  end
  if has3 then num3 = num3 + 1  end
end

-- print(num2 * num3)

-- part 2

-- input = {"abcde", "fghij", "klmno", "pqrst", "fguij", "axcye", "wvxyz"}

input = {}
for line in io.lines('2.in') do
  input[#input + 1] = line
end

for _, w1 in ipairs(input) do
  for _, w2 in ipairs(input) do
    diff = 0
    offset = 0
    for i = 1, #w1 do
      if w1:byte(i) ~= w2:byte(i) then
        diff = diff + 1
        offset = i
      end
    end
    if diff == 1 then
      common = w1:sub(1,offset-1) .. w1:sub(offset+1,#w1)
      print(w1, w2, common)
      goto summary
    end
  end
end

::summary::
