use lazy_static::lazy_static;
use maplit::hashmap;
use regex::Regex;
use std::collections::HashMap;

type RecipeMap = HashMap<&'static str, (u32, Vec<(u32, &'static str)>)>;
type LeftoverMap = HashMap<&'static str, u32>;

lazy_static! {
    static ref EX1: RecipeMap = hashmap! {
        "FUEL" => (1, vec![(7, "A"), (1, "E")]),
        "E" => (1, vec![(7, "A"), (1, "D")]),
        "D" => (1, vec![(7, "A"), (1, "C")]),
        "C" => (1, vec![(7, "A"), (1, "B")]),
        "B" => (1, vec![(1, "ORE")]),
        "A" => (10, vec![(10, "ORE")]),
    };
    static ref EX2: RecipeMap = hashmap! {
        "NZVS" => (5, vec![(157, "ORE")]),
        "DCFZ" => (6, vec![(165, "ORE")]),
        "FUEL" => (1, vec![(44, "XJWVT"), (5, "KHKGT"), (1, "QDVJ"), (29, "NZVS"), (9, "GPVTF"), (48, "HKGWZ")]),
        "QDVJ" => (9, vec![(12, "HKGWZ"), (1, "GPVTF"), (8, "PSHF")]),
        "PSHF" => (7, vec![(179, "ORE")]),
        "HKGWZ" => (5, vec![(177, "ORE")]),
        "XJWVT" => (2, vec![(7, "DCFZ"), (7, "PSHF")]),
        "GPVTF" => (2, vec![(165, "ORE")]),
        "KHKGT" => (8, vec![(3, "DCFZ"), (7, "NZVS"), (5, "HKGWZ"), (10, "PSHF")]),
    };
}

fn main() {
    assert_eq!(10, obtain("ORE", 10, &EX1, &mut hashmap! {}));
    assert_eq!(10, obtain("A", 10, &EX1, &mut hashmap! {}));
    assert_eq!(20, obtain("A", 20, &EX1, &mut hashmap! {}));
    assert_eq!(20, obtain("A", 12, &EX1, &mut hashmap! {}));
    assert_eq!(10, obtain("A", 12, &EX1, &mut hashmap! { "A" => 3 }));
    assert_eq!(11, obtain("C", 1, &EX1, &mut hashmap! {}));
    assert_eq!(21, obtain("D", 1, &EX1, &mut hashmap! {}));
    assert_eq!(31, obtain("FUEL", 1, &EX1, &mut hashmap! {}));
    assert_eq!(13312, obtain("FUEL", 1, &EX2, &mut hashmap! {}));

    let recipes = parse(include_str!("../../../in/day14.in"));
    println!("Part 1: {}", obtain("FUEL", 1, &recipes, &mut hashmap! {}));
}

fn split(s: &str) -> (u32, &str) {
    let mut split = s.split(" ");
    (
        split.next().unwrap().parse().unwrap(),
        split.next().unwrap(),
    )
}

fn parse(s: &'static str) -> RecipeMap {
    assert_eq!((7, "TEST"), split("7 TEST"));

    let mut recipes = RecipeMap::new();
    let re = Regex::new(r"\d+ [A-Z]+").unwrap();
    for line in s.lines() {
        let mut chemicals = re
            .find_iter(line)
            .map(|m| split(m.as_str()))
            .collect::<Vec<_>>();
        let (amount, target) = chemicals.pop().unwrap();
        recipes.insert(target, (amount, chemicals));
    }

    recipes
}

fn obtain(
    chemical: &'static str,
    mut required_amount: u32,
    recipes: &RecipeMap,
    leftovers: &mut LeftoverMap,
) -> u32 {
    if chemical == "ORE" {
        return required_amount;
    }

    // use leftovers first
    if let Some(left) = leftovers.get_mut(chemical) {
        if *left > required_amount {
            *left -= required_amount;
            return 0;
        } else {
            required_amount -= *left;
            *left = 0;
        }
    }

    let mut total = 0;

    // find substrates
    let (yield_amount, ingredients) = recipes.get(chemical).unwrap();

    let mut remaining_amount: i32 = required_amount as i32;
    while remaining_amount > 0 {
        for (ingredient_amount, ingredient_name) in ingredients {
            total += obtain(ingredient_name, *ingredient_amount, recipes, leftovers);
        }
        remaining_amount -= *yield_amount as i32;
    }

    if remaining_amount < 0 {
        *leftovers.entry(chemical).or_insert(0) += (-remaining_amount) as u32;
    }

    total
}
