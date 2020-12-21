use regex::Regex;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct Recipe {
    ingredients: HashSet<String>,
    allergens: HashSet<String>,
}

#[aoc_generator(day21)]
pub fn load_input(input: &str) -> Vec<Recipe> {
    let mut output = vec![];
    let re = Regex::new(r"^([a-z\s]+)\(contains ([a-z,\s]+)\)$").unwrap();
    for line in input.lines() {
        for cap in re.captures_iter(line) {
            let ingredients: HashSet<String> =
                cap[1].split_whitespace().map(|x| x.to_string()).collect();
            let allergens: HashSet<String> =
                cap[2].split(',').map(|x| x.trim().to_string()).collect();

            output.push(Recipe {
                ingredients,
                allergens,
            });
        }
    }
    output
}

pub fn algorithm_step(
    input: &[Recipe],
    allergens: &HashSet<String>,
    ingredients: &mut HashMap<String, Option<String>>,
) {
    for all in allergens.iter() {
        // Get recipes with this allergen listed
        let mut recipe_list: Vec<Recipe> = vec![];
        for recipe in input {
            if recipe.allergens.contains(all) {
                recipe_list.push(recipe.clone());
            }
        }

        // Candidate ingredients
        let mut candidate_ingredients: HashSet<String> = HashSet::new();
        for recipe in &recipe_list {
            for ing in recipe.ingredients.iter() {
                candidate_ingredients.insert(ing.to_string());
            }
        }

        let mut ingredient_all_contain: Vec<String> = vec![];
        for cing in candidate_ingredients.iter() {
            let mut all_contain = true;
            if let Some(Some(_ingredient)) = ingredients.get(cing) {
                // Already has an allergen, not a candidate
                all_contain = false;
            }

            for recipe in recipe_list.iter().skip(1) {
                if recipe.ingredients.iter().find(|&x| x == cing).is_none() {
                    // Candidate ingredient not contained in this recipe, move along
                    all_contain = false;
                }
            }
            if all_contain {
                ingredient_all_contain.push(cing.to_string());
            }
        }

        if ingredient_all_contain.len() == 1 {
            // We have a unique match!
            if let Some(x) = ingredients.get_mut(&ingredient_all_contain[0]) {
                *x = Some((*all).clone());
            }
        }
    }
}

#[aoc(day21, part1)]
pub fn part1(input: &[Recipe]) -> u64 {
    let mut allergens: HashSet<String> = HashSet::new();
    let mut ingredients: HashMap<String, Option<String>> = HashMap::new();

    for recipe in input {
        for ing in &recipe.ingredients {
            if !ingredients.contains_key(ing) {
                // Add to ingredients map
                ingredients.insert(ing.to_string(), None);
            }
        }

        for all in &recipe.allergens {
            allergens.insert(all.to_string());
        }
    }

    for _ in 0..allergens.len() {
        algorithm_step(input, &allergens, &mut ingredients);
    }

    // Build the safe_list
    let mut safe_list: HashSet<String> = HashSet::new();
    for (k, v) in &ingredients {
        if v.is_none() {
            safe_list.insert(k.clone());
        }
    }

    // Count all occurances of ingredients in the safe_list
    let mut cntr = 0;
    for candidate in safe_list.iter() {
        for recipe in input {
            if recipe.ingredients.contains(candidate) {
                cntr += 1;
            }
        }
    }

    cntr
}

#[aoc(day21, part2)]
pub fn part2(input: &[Recipe]) -> String {
    let mut allergens: HashSet<String> = HashSet::new();
    let mut ingredients: HashMap<String, Option<String>> = HashMap::new();

    for recipe in input {
        for ing in &recipe.ingredients {
            if !ingredients.contains_key(ing) {
                // Add to ingredients map
                ingredients.insert(ing.to_string(), None);
            }
        }

        for all in &recipe.allergens {
            allergens.insert(all.to_string());
        }
    }

    for _ in 0..allergens.len() {
        algorithm_step(input, &allergens, &mut ingredients);
    }

    // Build the danger list
    let mut danger_list: Vec<(String, String)> = vec![];
    for (k, v) in &ingredients {
        if let Some(allergen) = v {
            danger_list.push((k.clone(), allergen.to_string()));
        }
    }

    // Sort them
    danger_list.sort_unstable_by_key(|x| x.1.clone());
    let first = danger_list[0].0.clone();

    danger_list.iter().skip(1).fold(first, |mut acc, x| {
        acc.push(',');
        acc.push_str(&x.0);
        acc
    })
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let input = read_to_string("input/21a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 5);
    }

    #[test]
    fn test_part2() {
        let input = read_to_string("input/21a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), "mxmxvkd,sqjhc,fvjkl".to_string());
    }
}
