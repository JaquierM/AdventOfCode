use std::{fs::read_to_string, ops::RangeInclusive};

fn main() {
    let content = read_to_string("./day5/files/input").expect("Invalid file");
    let (fresh_ranges, ingredients) = parse(&content);
    let part1 = count_fresh_ingredients(&fresh_ranges, &ingredients);
    println!("Sum for part 1 is {}", part1);

    let part2 = get_fresh_ingredients(&fresh_ranges);
    println!("Sum for part 2 is {}", part2);
}

fn parse(input: &str) -> (Vec<RangeInclusive<i64>>, Vec<i64>) {
    let mut fresh_ranges = Vec::new();
    let mut ingredients = Vec::new();

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        if line.contains("-") {
            let (start, end) = line.split_at(line.find('-').expect("- not found"));
            let end = &end[1..];

            fresh_ranges.push(
                start.parse().expect("invalid number")..=end.parse().expect("invalid number"),
            );
        } else {
            ingredients.push(line.parse().expect("invalid number"));
        }
    }

    (merge_range(&mut fresh_ranges), ingredients)
}

fn merge_range(fresh_ranges: &mut [RangeInclusive<i64>]) -> Vec<RangeInclusive<i64>> {
    fresh_ranges.sort_by_key(|r| *r.start());

    let mut merged: Vec<RangeInclusive<i64>> = Vec::new();

    for range in fresh_ranges {
        let cloned = range.clone();

        let matched_range = merged
            .iter()
            .find(|r| r.contains(cloned.start()) || r.contains(cloned.end()));

        if let Some(matched_range) = matched_range.cloned() {
            merged.pop_if(|element| *element == matched_range);

            let new_start = *cloned.start().min(matched_range.start());
            let new_end = *cloned.end().max(matched_range.end());

            merged.push(new_start..=new_end);
        } else {
            merged.push(cloned);
        }
    }

    merged
}

fn count_fresh_ingredients(fresh_ranges: &[RangeInclusive<i64>], ingredients: &[i64]) -> usize {
    ingredients
        .iter()
        .filter(|ingredient| check_ingredient_fresh(fresh_ranges, ingredient))
        .count()
}

fn get_fresh_ingredients(fresh_ranges: &[RangeInclusive<i64>]) -> i64 {
    fresh_ranges.iter().map(|r| *r.end() - *r.start() + 1).sum()
}

fn check_ingredient_fresh(fresh_ranges: &[RangeInclusive<i64>], ingredient: &i64) -> bool {
    fresh_ranges.iter().any(|range| range.contains(ingredient))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range() {
        let range = 3..=5;
        assert_eq!(range.contains(&2), false);
        assert_eq!(range.contains(&3), true);
        assert_eq!(range.contains(&4), true);
        assert_eq!(range.contains(&5), true);
        assert_eq!(range.contains(&6), false);
    }

    #[test]
    fn test_parse() {
        let test_data = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

        let result = parse(&test_data);

        assert_eq!(result, (vec![3..=5, 10..=20], vec![1, 5, 8, 11, 17, 32]));
    }

    #[test]
    fn test_ingredient_fresh() {
        assert_eq!(check_ingredient_fresh(&vec![3..=5], &2), false);
        assert_eq!(check_ingredient_fresh(&vec![3..=5], &3), true);
        assert_eq!(check_ingredient_fresh(&vec![3..=5], &4), true);
        assert_eq!(check_ingredient_fresh(&vec![3..=5], &5), true);
        assert_eq!(check_ingredient_fresh(&vec![3..=5], &6), false);
    }

    #[test]
    fn test_range_are_merged() {
        let test_data = "3-5
    10-14
    16-20
    12-18

    1
    5
    8
    11
    17
    32";

        let (mut fresh_ranges, _) = parse(&test_data);

        let fresh_ranges = merge_range(&mut fresh_ranges);

        assert_eq!(fresh_ranges, vec![3..=5, 10..=20]);
    }

    #[test]
    fn test_with_example_part_1() {
        let test_data = "3-5
    10-14
    16-20
    12-18

    1
    5
    8
    11
    17
    32";

        let (fresh_ranges, ingredients) = parse(&test_data);
        let result = count_fresh_ingredients(&fresh_ranges, &ingredients);

        assert_eq!(result, 3);
    }

    #[test]
    fn test_with_example_part_2() {
        let test_data = "3-5
    10-14
    16-20
    12-18

    1
    5
    8
    11
    17
    32";

        let (fresh_ranges, _) = parse(&test_data);
        let result = get_fresh_ingredients(&fresh_ranges);

        println!("{:?}", result);

        assert_eq!(result, 14);
    }
}
