use rayon::prelude::*;
use std::fs::read_to_string;

fn main() {
    let content = read_to_string("./day3/files/input").expect("Invalid file");

    let part1: i64 = content
        .par_lines()
        .filter(|line| !line.is_empty())
        .map(|batteries| extract_pair(batteries, 2))
        .sum();

    println!("Sum for part 1 is {}", part1);

    let part2: i64 = content
        .par_lines()
        .filter(|line| !line.is_empty())
        .map(|batteries| extract_pair(batteries, 12))
        .sum();
    println!("Sum for part 2 is {}", part2);
}

fn extract_pair(batteries: &str, size: usize) -> i64 {
    let mut len = batteries.len() - size + 1;
    let mut start: usize = 0;

    let mut res: Vec<char> = Vec::new();
    while res.len() != size {
        if start > len {
            break;
        }

        let max = &batteries[start..len].char_indices().fold(
            Option::None,
            |opt: Option<(usize, char)>, (i, c)| match opt {
                None => Some((i, c)),
                Some((_, letter)) => {
                    if c > letter {
                        Some((i, c))
                    } else {
                        opt
                    }
                }
            },
        );

        if let Some((index, letter)) = max {
            res.push(*letter);
            start += *index + 1;
            len += 1;
        } else {
            break;
        }
    }

    res.iter().collect::<String>().parse::<i64>().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_with_size_2() {
        assert_eq!(extract_pair("987654321111111", 2), 98);
        assert_eq!(extract_pair("811111111111119", 2), 89);
        assert_eq!(extract_pair("234234234234278", 2), 78);
        assert_eq!(extract_pair("818181911112111", 2), 92);
        assert_eq!(
            extract_pair(
                "2344323254238324344443324333412234342243363246314375326354514244431354834344246137562233387223242853",
                2
            ),
            88
        );
    }

    #[test]
    fn test_extract_with_size_12() {
        assert_eq!(extract_pair("987654321111111", 12), 987654321111);
        assert_eq!(extract_pair("811111111111119", 12), 811111111119);
        assert_eq!(extract_pair("234234234234278", 12), 434234234278);
        assert_eq!(extract_pair("818181911112111", 12), 888911112111);
        assert_eq!(
            extract_pair(
                "2344323254238324344443324333412234342243363246314375326354514244431354834344246137562233387223242853",
                12
            ),
            888723242853
        );
    }

    #[test]
    fn test_with_example_part_1() {
        let test_data = "987654321111111
811111111111119
234234234234278
818181911112111";

        let result: i64 = test_data
            .par_lines()
            .filter(|line| !line.is_empty())
            .map(|batteries| extract_pair(batteries, 2))
            .sum();

        assert_eq!(result, 357);
    }

    #[test]
    fn test_with_example_part_2() {
        let test_data = "987654321111111
811111111111119
234234234234278
818181911112111";

        let result: i64 = test_data
            .par_lines()
            .filter(|line| !line.is_empty())
            .map(|batteries| extract_pair(batteries, 12))
            .sum();

        assert_eq!(result, 3121910778619);
    }
}
