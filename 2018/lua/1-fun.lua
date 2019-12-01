if not each then require('fun')() end

a = {}
for line in io.lines('1.in') do
  a[#a + 1] = tonumber(line)
end

print('sum', sum(a))
