use std::cmp::PartialEq;
use std::collections::HashSet;
use std::fs::read_to_string;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, Clone)]
struct Button {
    index: Vec<usize>,
}

impl FromStr for Button {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.starts_with('(') || !s.ends_with(')') {
            return Err(String::from("Button must be between [()]"));
        }

        Ok(Button {
            index: s[1..s.len() - 1]
                .split(",")
                .map(|l| l.parse::<usize>().expect("Invalid number"))
                .collect(),
        })
    }
}

#[derive(Debug)]
struct Machine {
    wanted_state: Vec<bool>,
    buttons: Vec<Button>,
    wanted_joltage: Vec<i32>,
}

impl FromStr for Machine {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();

        let wanted_state = parts
            .get(0)
            .expect("Missing state")
            .chars()
            .filter(|c| *c == '.' || *c == '#')
            .map(|c| match c {
                '#' => true,
                _ => false,
            })
            .collect();

        let buttons: Vec<Button> = parts[1..parts.len() - 1]
            .iter()
            .map(|b| Button::from_str(*b).expect("Invalid button"))
            .collect();

        let joltage = parts.last().expect("Missing joltage");
        let joltage = joltage[1..joltage.len() - 1]
            .split(',')
            .map(|c| c.parse::<i32>().expect("Invalid number"))
            .collect();

        Ok(Machine {
            wanted_state,
            buttons,
            wanted_joltage: joltage,
        })
    }
}

type StateLight = (Vec<bool>, Option<Button>, usize);
type StateJolt = (Vec<i32>, Option<Button>, usize);

impl Machine {
    fn joltage_valid(&self, to_validate: &[i32]) -> bool {
        self.wanted_joltage
            .iter()
            .zip(to_validate)
            .all(|(j1, j2)| j2 <= j1)
    }

    fn push_buttons(&self) -> i32 {
        let mut states: Vec<StateLight> = vec![(vec![false; self.wanted_state.len()], None, 0)];
        let mut visited: HashSet<Vec<bool>> = HashSet::new();

        for _ in 0..2 << self.wanted_state.len() {
            let mut new_states: Vec<StateLight> = vec![];
            for state in states {
                for button in self.buttons.iter() {
                    if let Some(previous) = state.1.as_ref()
                        && previous == button
                    {
                        continue;
                    }

                    let new_state: StateLight = (
                        state
                            .0
                            .iter()
                            .enumerate()
                            .map(|(index, previous)| {
                                if button.index.contains(&index) {
                                    !*previous
                                } else {
                                    *previous
                                }
                            })
                            .collect(),
                        Some(button.clone()),
                        state.2 + 1,
                    );

                    if !visited.insert(new_state.0.clone()) {
                        continue;
                    }

                    if new_state.0 == self.wanted_state {
                        return new_state.2 as i32;
                    }

                    new_states.push(new_state);
                }
            }

            states = new_states;
        }
        0
    }

    fn push_buttons_and_lever(&self) -> i32 {
        let mut states: Vec<StateJolt> = vec![(vec![0; self.wanted_state.len()], None, 0)];
        let mut visited: HashSet<Vec<i32>> = HashSet::new();

        for _ in 0..self.wanted_joltage.iter().max().copied().unwrap_or(0) + 3 {
            let mut new_states: Vec<StateJolt> = vec![];
            for state in states {
                for button in self.buttons.iter() {
                    if let Some(previous) = state.1.as_ref()
                        && previous == button
                    {
                        continue;
                    }

                    let new_state: StateJolt = (
                        state
                            .0
                            .iter()
                            .enumerate()
                            .map(|(index, previous)| {
                                if button.index.contains(&index) {
                                    previous + 1
                                } else {
                                    *previous
                                }
                            })
                            .collect(),
                        Some(button.clone()),
                        state.2 + 1,
                    );

                    if !self.joltage_valid(&new_state.0) {
                        continue;
                    }

                    if !visited.insert(new_state.0.clone()) {
                        continue;
                    }

                    if new_state.0 == self.wanted_joltage {
                        println!("Took {}", new_state.2);
                        return new_state.2 as i32;
                    }

                    new_states.push(new_state);
                }
            }

            states = new_states;
        }
        0
    }
}

fn main() {
    let content = read_to_string("./day10/files/input").expect("Invalid file");
    let machines = parse(&content);

    let part1 = machines.iter().map(|m| m.push_buttons()).sum::<i32>();
    println!("Sum for part 1 is {part1}");

    let part2 = machines
        .iter()
        .map(|m| m.push_buttons_and_lever())
        .sum::<i32>();

    println!("Sum for part 2 is {part2}");
}

fn parse(input: &str) -> Vec<Machine> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| Machine::from_str(line).expect("Invalid machine"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_DATA: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

    #[test]
    fn test_example_part_1() {
        let machines = parse(TEST_DATA);

        let count: i32 = machines.iter().map(|m| m.push_buttons()).sum();

        assert_eq!(count, 7);
    }

    #[test]
    fn test_example_part_2() {
        let machines = parse(TEST_DATA);

        let count: i32 = machines.iter().map(|m| m.push_buttons_and_lever()).sum();

        assert_eq!(count, 33);
    }
}
