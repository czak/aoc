s = {3, 7}
count = 2

c1 = 1
c2 = 2

pattern = {0, 7, 4, 5, 0, 1}

-- function compare()
--   return s[count] == pattern[6] and
--     s[count - 1] == pattern[5] and
--     s[count - 2] == pattern[4] and
--     s[count - 3] == pattern[3] and
--     s[count - 4] == pattern[2] and
--     s[count - 5] == pattern[1]
-- end

a = 0

while true do
  sum = s[c1] + s[c2]

  if sum >= 10 then
    s[count + 1] = 1
    count = count + 1
    if s[count] == 1 and
      s[count - 1] == 0 and
      s[count - 2] == 5 and
      s[count - 3] == 4 and
      s[count - 4] == 7 and
      s[count - 5] == 0 then break end
  end
  s[count + 1] = sum % 10
  count = count + 1
  if s[count] == 1 and
    s[count - 1] == 0 and
    s[count - 2] == 5 and
    s[count - 3] == 4 and
    s[count - 4] == 7 and
    s[count - 5] == 0 then break end

  c1 = (c1 + s[c1]) % count + 1
  c2 = (c2 + s[c2]) % count + 1
end

print(count - #pattern)
