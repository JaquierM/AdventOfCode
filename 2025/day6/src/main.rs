use std::fs::read_to_string;
#[derive(PartialEq, Debug)]
struct Problem {
    numbers: Vec<i64>,
    operation: Operation,
}

#[derive(PartialEq, Debug)]
enum Operation {
    Add,
    Multiply,
}

impl Problem {
    fn execute(&self) -> i64 {
        match self.operation {
            Operation::Add => self.numbers.iter().sum(),
            Operation::Multiply => self.numbers.iter().product(),
        }
    }

    fn parse_human(operation: &str, numbers: &[&str]) -> Problem {
        Problem {
            numbers: numbers
                .iter()
                .map(|n| {
                    n.trim()
                        .parse()
                        .unwrap_or_else(|_| panic!("Not a number [{n}]"))
                })
                .collect(),
            operation: Operation::from(operation),
        }
    }

    fn parse_cephalopods(operation: &str, numbers: &[&str]) -> Problem {
        let max_size = numbers.iter().map(|n| n.len()).max().unwrap_or(0);

        let numbers = (0..max_size)
            .map(|index| {
                numbers
                    .iter()
                    .map(|n| n.get(index..index + 1).unwrap_or(""))
                    .collect::<String>()
            })
            .map(|number| number.trim().parse::<i64>().expect("Invalid number"))
            .collect();

        Problem {
            numbers,
            operation: Operation::from(operation),
        }
    }
}

impl From<&str> for Operation {
    fn from(value: &str) -> Self {
        match value.trim() {
            "+" => Operation::Add,
            "*" => Operation::Multiply,
            _ => Operation::Add,
        }
    }
}

fn main() {
    let content = read_to_string("./day6/files/input").expect("Invalid file");
    let problems_human = parse(&content, Problem::parse_human);
    let problems_cephalopods = parse(&content, Problem::parse_cephalopods);

    let part1: i64 = problems_human.iter().map(Problem::execute).sum();
    println!("Sum for part 1 is {}", part1);

    let part2: i64 = problems_cephalopods.iter().map(Problem::execute).sum();
    println!("Sum for part 2 is {}", part2);
}

fn parse(input: &str, parser: fn(operation: &str, numbers: &[&str]) -> Problem) -> Vec<Problem> {
    let columns = {
        let lines: Vec<&str> = input.lines().filter(|x| !x.is_empty()).collect();
        let min_size = lines.iter().map(|line| line.len()).min().unwrap_or(0);

        let mut columns: Vec<Vec<&str>> = Vec::new();
        let mut start_index = 0;
        for index in 0..min_size {
            if lines.iter().all(|line| &line[index..index + 1] == " ") {
                columns.push(
                    lines
                        .iter()
                        .map(|line| line.get(start_index..index).unwrap_or(""))
                        .collect(),
                );
                start_index = index + 1;
            }
        }

        columns.push(
            lines
                .iter()
                .map(|line| line.get(start_index..).unwrap_or(""))
                .collect(),
        );
        columns
    };

    columns
        .iter()
        .map(|column| {
            let (operation, numbers) = column.split_last().expect("Invalid column");
            parser(operation, numbers)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_DATA: &str = "
123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  ";

    #[test]
    fn test_parse_human() {
        let result = parse(&TEST_DATA, Problem::parse_human);

        assert_eq!(
            result,
            vec![
                Problem {
                    numbers: vec![123, 45, 6],
                    operation: Operation::Multiply
                },
                Problem {
                    numbers: vec![328, 64, 98],
                    operation: Operation::Add
                },
                Problem {
                    numbers: vec![51, 387, 215],
                    operation: Operation::Multiply
                },
                Problem {
                    numbers: vec![64, 23, 314],
                    operation: Operation::Add
                }
            ]
        );
    }

    #[test]
    fn test_parse_cephalopods() {
        let result = parse(&TEST_DATA, Problem::parse_cephalopods);

        assert_eq!(
            result,
            vec![
                Problem {
                    numbers: vec![1, 24, 356],
                    operation: Operation::Multiply
                },
                Problem {
                    numbers: vec![369, 248, 8],
                    operation: Operation::Add
                },
                Problem {
                    numbers: vec![32, 581, 175],
                    operation: Operation::Multiply
                },
                Problem {
                    numbers: vec![623, 431, 4],
                    operation: Operation::Add
                }
            ]
        );
    }

    #[test]
    fn test_execute() {
        assert_eq!(
            Problem {
                numbers: vec![123, 45, 6],
                operation: Operation::Multiply
            }
            .execute(),
            33210
        );
        assert_eq!(
            Problem {
                numbers: vec![328, 64, 98],
                operation: Operation::Add
            }
            .execute(),
            490
        );
        assert_eq!(
            Problem {
                numbers: vec![51, 387, 215],
                operation: Operation::Multiply
            }
            .execute(),
            4243455
        );
        assert_eq!(
            Problem {
                numbers: vec![64, 23, 314],
                operation: Operation::Add
            }
            .execute(),
            401
        );
    }

    #[test]
    fn test_example_part_1() {
        let result: i64 = parse(&TEST_DATA, Problem::parse_human)
            .iter()
            .map(Problem::execute)
            .sum();
        assert_eq!(result, 4277556)
    }

    #[test]
    fn test_example_part_2() {
        let result: i64 = parse(&TEST_DATA, Problem::parse_cephalopods)
            .iter()
            .map(Problem::execute)
            .sum();
        assert_eq!(result, 3263827)
    }
}
