use rayon::prelude::*;
use std::fs::read_to_string;
use std::ops::RangeInclusive;

fn main() {
    let content = read_to_string("./day2/files/input").expect("Invalid file");

    let part1 = compute(&content, parse, is_valid_part_1);
    println!("Sum for part 1 is {}", part1);

    let part2 = compute(&content, parse, is_valid_part_2);
    println!("Sum for part 2 is {}", part2);
}

fn compute(
    content: &str,
    parser: fn(&str) -> RangeInclusive<i64>,
    validator: fn(&i64) -> bool,
) -> i64 {
    content
        .par_split(',')
        .flat_map(|range| parser(range).into_par_iter())
        .filter(|id| !validator(id))
        .sum()
}

fn parse(range: &str) -> RangeInclusive<i64> {
    let mut parts = range.split("-");
    let n1 = parts.next().expect("missing");
    let n2 = parts.next().expect("missing");
    RangeInclusive::new(
        n1.parse::<i64>().expect("invalid number"),
        n2.parse::<i64>().expect("invalid number"),
    )
}

fn is_valid_part_1(id: &i64) -> bool {
    let text = id.to_string();
    if !text.len().is_multiple_of(2) {
        return true;
    }

    let (start, end) = text.split_at(text.len() / 2);
    start != end
}

fn is_valid_part_2(id: &i64) -> bool {
    let text = id.to_string();

    let chars = text.chars();
    let mut part = String::new();

    for char in chars {
        part.push(char);

        if part == text {
            break;
        }

        let size = text.len() / part.len();

        if part.repeat(size) == text {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(parse("11-22"), RangeInclusive::new(11, 22));
        assert_eq!(parse("222220-222224"), RangeInclusive::new(222220, 222224));
    }

    #[test]
    fn test_valid_part_1() {
        assert_eq!(is_valid_part_1(&11), false);
        assert_eq!(is_valid_part_1(&12), true);
        assert_eq!(is_valid_part_1(&222221), true);
    }

    #[test]
    fn test_valid_part_2() {
        assert_eq!(is_valid_part_2(&11), false);
        assert_eq!(is_valid_part_2(&12), true);
        assert_eq!(is_valid_part_2(&111), false);
        assert_eq!(is_valid_part_2(&212121), false);
    }

    #[test]
    fn test_with_example_part_1() {
        let test_data = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

        let result: i64 = compute(test_data, parse, is_valid_part_1);

        assert_eq!(result, 1227775554);
    }

    #[test]
    fn test_with_example_part_2() {
        let test_data = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

        let result: i64 = compute(test_data, parse, is_valid_part_2);

        assert_eq!(result, 4174379265);
    }
}
