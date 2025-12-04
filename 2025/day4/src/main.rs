use std::fs::read_to_string;

#[derive(PartialEq)]
struct Position {
    row: usize,
    column: usize,
}

fn main() {
    let content = read_to_string("./day4/files/input").expect("Invalid file");
    let mut input = parse(&content);
    let mut movable = get_movable(&input);

    println!("Sum for part 1 is {}", movable.len());

    let mut part2 = movable.len();

    while !movable.is_empty() {
        input = remove_moved(&input, &movable);
        movable = get_movable(&input);
        part2 += movable.len();
    }

    println!("Sum for part 2 is {}", part2);
}

fn parse(input: &str) -> Vec<Vec<bool>> {
    let mut shelf: Vec<Vec<bool>> = Vec::new();

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        shelf.push(
            line.trim()
                .chars()
                .map(|c| match c {
                    '.' => false,
                    '@' => true,
                    _ => false,
                })
                .collect(),
        );
    }

    shelf
}

fn get_movable(shelf: &Vec<Vec<bool>>) -> Vec<Position> {
    let mut movable: Vec<Position> = Vec::new();

    let row_len = shelf.len();
    let column_len = shelf.first().map(|row| row.len()).unwrap_or(0);

    for row in 0..row_len {
        for column in 0..column_len {
            if count_adjacent(shelf, Position { column, row }) < 4 {
                movable.push(Position { column, row });
            }
        }
    }

    movable
}

fn remove_moved(shelf: &Vec<Vec<bool>>, movable: &Vec<Position>) -> Vec<Vec<bool>> {
    let mut new_shelf: Vec<Vec<bool>> = Vec::new();

    for (row_index, column) in shelf.iter().enumerate() {
        let mut new_column: Vec<bool> = Vec::new();
        for (column_index, roll) in column.iter().enumerate() {
            if *roll
                && movable.contains(&Position {
                    row: row_index,
                    column: column_index,
                })
            {
                new_column.push(false);
            } else if *roll {
                new_column.push(true);
            } else {
                new_column.push(false);
            }
        }
        new_shelf.push(new_column);
    }

    new_shelf
}

fn count_adjacent(shelf: &Vec<Vec<bool>>, pos: Position) -> i32 {
    let is_roll = shelf
        .get(pos.row)
        .map(|current_row| current_row.get(pos.column).copied().unwrap_or(false))
        .unwrap_or(false);

    if !is_roll {
        return i32::MAX;
    }

    let mut count: i32 = 0;

    let row_start = if pos.row >= 1 { pos.row - 1 } else { pos.row };
    let row_end = pos.row + 2;

    let column_start = if pos.column >= 1 {
        pos.column - 1
    } else {
        pos.column
    };
    let column_end = pos.column + 2;

    for row_number in row_start..row_end {
        if let Some(current_row) = shelf.get(row_number) {
            for column_number in column_start..column_end {
                if row_number == pos.row && column_number == pos.column {
                    continue;
                }

                if let Some(roll) = current_row.get(column_number).copied()
                    && roll
                {
                    count += 1;
                };
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing_basic() {
        let test_data = "@.
                               @.";

        let expected = vec![vec![true, false], vec![true, false]];

        assert_eq!(parse(test_data), expected);
    }

    #[test]
    fn test_count_adjacent() {
        let test_data = vec![vec![true, false], vec![true, false]];

        assert_eq!(
            count_adjacent(&test_data, Position { row: 0, column: 0 }),
            1
        );
        assert_eq!(
            count_adjacent(&test_data, Position { row: 0, column: 1 }),
            i32::MAX
        );
        assert_eq!(
            count_adjacent(&test_data, Position { row: 1, column: 0 }),
            1
        );
        assert_eq!(
            count_adjacent(&test_data, Position { row: 1, column: 1 }),
            i32::MAX
        );
    }

    #[test]
    fn test_with_example_part_1() {
        let test_data = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

        let result = get_movable(&parse(&test_data)).len();

        assert_eq!(result, 13);
    }

    #[test]
    fn test_with_example_part_2() {
        let test_data = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

        let mut input = parse(&test_data);
        let mut movable = get_movable(&input);

        println!("Sum for part 1 is {}", movable.len());

        let mut result = movable.len();

        while !movable.is_empty() {
            input = remove_moved(&input, &movable);
            movable = get_movable(&input);
            result += movable.len();
        }

        assert_eq!(result, 43);
    }
}
