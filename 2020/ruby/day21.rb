require 'set'

data = <<~DATA
mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)
DATA

data = File.read('../in/day21.in')

foods = data.lines.map do |line|
  m = line.match(/(.+) \(contains (.+)\)/)
  [m[1].split, m[2].split(', ')]
end

ALL_INGREDIENTS = foods.map(&:first).flatten
ALL_ALLERGENS = foods.map(&:last).flatten

# try to map allergen to ingredient
suspect_ingredients = Hash.new { |h, k| h[k] = Set.new(ALL_INGREDIENTS) }

foods.each do |ingredients, allergens|
  allergens.each do |a|
    suspect_ingredients[a] &= ingredients
  end
end

safe_ingredients = ALL_INGREDIENTS.to_set - suspect_ingredients.values.reduce(:|)

# Part 1
puts ALL_INGREDIENTS.count { |i| safe_ingredients.include?(i) }

##############

# Part 2
bad = {}

until bad.length == ALL_ALLERGENS.uniq.length
  # move those suspect_ingredients
  # which have a definite suspect (1 candidate)
  suspect_ingredients.each do |allergen, ingset|
    next if ingset.size > 1
    bad[ingset.first] = allergen
  end

  # keep only those uncertain (2+ candidates)
  suspect_ingredients.select! { |a, i| i.size > 1 }

  known_ingredients = bad.keys

  suspect_ingredients = suspect_ingredients
    .map { |a, iset| [a, iset - known_ingredients] }
    .to_h
end

puts bad.sort_by(&:last).map(&:first).join(',')
