use std::fmt::{Display, Formatter};
use std::fs::read_to_string;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
struct State {
    code: i32,
    position: i32,
}

#[derive(Debug)]
struct Movement {
    step: i32,
}

impl Default for State {
    fn default() -> Self {
        State {
            code: 0,
            position: 50,
        }
    }
}

impl Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[code: {}, position: {}]", self.code, self.position)
    }
}

impl FromStr for Movement {
    type Err = ParseIntError;

    fn from_str(raw: &str) -> Result<Self, ParseIntError> {
        let m = String::from(raw);
        let (direction, number) = m.split_at(1);
        let step = number.parse::<i32>()?;

        let step = match direction {
            "L" => -step,
            "R" => step,
            _ => step,
        };

        Ok(Movement { step })
    }
}

fn main() {
    let content = read_to_string("./files/input").expect("Invalid file");
    let lines: Vec<&str> = content.lines().collect();
    let state = entrance_code_v2(State::default(), &lines);

    println!("State is {}", state);
}

fn entrance_code_v2(initial: State, movements: &Vec<&str>) -> State {
    movements
        .iter()
        .map(move |data| Movement::from_str(data).expect("Invalid movement"))
        .fold(initial, move |state, movement| {
            let mut new_position = state.position + (movement.step % 100);
            let mut new_code = state.code + (movement.step.abs() / 100);

            if new_position == 0 && state.position != 0 {
                new_code += 1;
            }

            if new_position > 99 {
                new_position -= 100;
                if state.position != 0 {
                    new_code += 1;
                }
            } else if new_position < 0 {
                new_position += 100;
                if state.position != 0 {
                    new_code += 1;
                }
            }

            State {
                code: new_code,
                position: new_position,
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_example_v2() {
        let test_data = vec![
            "L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82",
        ];
        assert_eq!(entrance_code_v2(State::default(), &test_data).code, 6);
    }
    #[test]
    fn test_multiple_turn_v2() {
        let test_data = vec!["L50", "R1000"];
        assert_eq!(entrance_code_v2(State::default(), &test_data).code, 11);

        let test_data = vec!["R50", "L1000"];
        assert_eq!(entrance_code_v2(State::default(), &test_data).code, 11);
    }
}
