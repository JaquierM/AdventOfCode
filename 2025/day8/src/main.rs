use std::cmp::PartialEq;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::fs::read_to_string;
use std::ops::{Add, Sub};
use std::str::FromStr;

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
struct Position {
    x: i64,
    y: i64,
    z: i64,
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}

impl FromStr for Position {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let coordinates: Vec<_> = input
            .split(",")
            .map(|part| part.parse::<i64>().map_err(|_| String::from("Invalid int")))
            .collect();

        if coordinates.len() != 3 {
            Err(String::from("Invalid coordinates"))
        } else {
            Ok(Position {
                x: coordinates
                    .first()
                    .ok_or(String::from("Missing coordinates"))?
                    .clone()?,
                y: coordinates
                    .get(1)
                    .ok_or(String::from("Missing coordinates"))?
                    .clone()?,
                z: coordinates
                    .get(2)
                    .ok_or(String::from("Missing coordinates"))?
                    .clone()?,
            })
        }
    }
}

impl Add for &Position {
    type Output = Position;

    fn add(self, other: Self) -> Position {
        Position {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for &Position {
    type Output = Position;

    fn sub(self, other: Self) -> Self::Output {
        Position {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Position {
    fn distance(&self, other: &Position) -> f64 {
        (self - other).len()
    }

    fn len(&self) -> f64 {
        let i = self.x.pow(2);
        let i1 = self.y.pow(2);
        let i2 = self.z.pow(2);
        ((i + i1 + i2) as f64).sqrt()
    }
}

fn main() {
    let content = read_to_string("./day8/files/input").expect("Invalid file");
    let boxes = parse(&content);
    //let manifold = Manifold::from_str(&content).expect("Invalid manifold");
    //let (part1, part2) = manifold.launch_beam();

    let (part1, part2) = connect(&boxes, 1000);
    println!("Sum for part 1 is {}", part1);
    println!("Sum for part 2 is {}", part2);
}

fn parse(input: &str) -> Vec<Position> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| Position::from_str(line).expect("Invalid coordinate"))
        .collect()
}

fn sort_by_shortest_distance_pair(boxes: &[Position]) -> Vec<(Position, Position)> {
    let sorted_boxes_distance = {
        let mut res: Vec<(Position, Position, f64)> = boxes
            .iter()
            .enumerate()
            .flat_map(|(index, pos)| {
                boxes
                    .iter()
                    .skip(index + 1)
                    .map(|other| (*pos, *other, pos.distance(other)))
            })
            .collect();

        res.sort_by(|x1, x2| x1.2.total_cmp(&x2.2));
        res
    };

    sorted_boxes_distance
        .iter()
        .map(|(box1, box2, _)| (*box1, *box2))
        .collect()
}

fn connect(boxes: &[Position], shortest_limit: usize) -> (u64, i64) {
    let mut circuits: Vec<HashSet<Position>> = boxes
        .iter()
        .map(|junction| HashSet::from([*junction]))
        .collect();

    let mut last_connected: Option<(Position, Position)> = None;
    let mut shortest_product: Option<u64> = None;
    for (shortest_connection_count, (box1, box2)) in sort_by_shortest_distance_pair(boxes)
        .into_iter()
        .enumerate()
    {
        if shortest_connection_count == shortest_limit {
            let mut sorted_circuits: Vec<u64> = circuits.iter().map(|c| c.len() as u64).collect();
            sorted_circuits.sort();
            sorted_circuits.reverse();

            shortest_product = Some(sorted_circuits[0..3].iter().product());
        }

        if circuits
            .iter()
            .any(|map| map.contains(&box1) && map.contains(&box2))
        {
            continue;
        }

        let existing_circuit: Vec<HashSet<Position>> = circuits
            .iter()
            .filter(|map| map.contains(&box1) || map.contains(&box2))
            .cloned()
            .collect();

        let mut new_circuit = HashSet::new();
        for circuit in existing_circuit {
            let to_remove = circuits
                .iter()
                .enumerate()
                .find(|(_, x)| **x == circuit)
                .map(|(i, _)| i);

            if let Some(index) = to_remove {
                circuits.remove(index);
            }

            circuit.iter().for_each(|junction| {
                new_circuit.insert(*junction);
            });
        }

        circuits.push(new_circuit);

        if circuits.len() == 1 {
            last_connected = Some((box1, box2));
            break;
        }
    }

    (
        shortest_product.unwrap_or(0),
        last_connected
            .map(|(box1, box2)| box1.x * box2.x)
            .unwrap_or(0),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_DATA: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    #[test]
    fn test_example_part_1() {
        let boxes = parse(TEST_DATA);

        assert_eq!(connect(&boxes, 10).0, 40)
    }

    #[test]
    fn test_example_part_2() {
        let boxes = parse(TEST_DATA);

        assert_eq!(connect(&boxes, 10).1, 25272)
    }
}
