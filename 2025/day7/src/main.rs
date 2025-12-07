use rayon::prelude::*;
use std::cmp::PartialEq;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::hash::{Hash, Hasher};
use std::str::FromStr;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering::Relaxed;

#[derive(Copy, Clone, Debug)]
struct Position {
    row: usize,
    column: usize,
    timeline: u64,
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.row == other.row && self.column == other.column
    }
}

impl Hash for Position {
    fn hash<H>(&self, h: &mut H)
    where
        H: Hasher,
    {
        self.column.hash(h);
        self.row.hash(h)
    }
}

impl Eq for Position {}

#[derive(Eq, PartialEq, Debug)]
enum Element {
    Source,
    Beam,
    Splitter,
    Empty,
}

#[derive(Eq, PartialEq, Debug)]
struct Manifold {
    source: Position,
    content: HashMap<Position, Element>,
    size: usize,
}

impl FromStr for Element {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "S" => Ok(Element::Source),
            "|" => Ok(Element::Beam),
            "^" => Ok(Element::Splitter),
            "." => Ok(Element::Empty),
            element => Err(format!("Invalid element [{element}]")),
        }
    }
}

impl FromStr for Manifold {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut content = HashMap::new();
        let mut source: Option<Position> = None;
        let mut size = 0;
        for (row, line) in s.lines().enumerate() {
            if line.is_empty() {
                continue;
            }

            size += 1;

            for (column, letter) in line.chars().enumerate() {
                let position = Position {
                    row,
                    column,
                    timeline: 1,
                };
                let element = Element::from_str(&format!("{letter}"))?;

                if element == Element::Source {
                    source = Some(position);
                } else if element == Element::Empty {
                    continue;
                }

                content.insert(position, element);
            }
        }

        Ok(Manifold {
            source: source.ok_or("Missing source")?,
            content,
            size,
        })
    }
}

impl Position {
    fn down(&self) -> Position {
        Position {
            row: self.row + 1,
            column: self.column,
            timeline: self.timeline,
        }
    }

    fn left(&self) -> Position {
        Position {
            row: self.row,
            column: self.column - 1,
            timeline: self.timeline,
        }
    }

    fn right(&self) -> Position {
        Position {
            row: self.row,
            column: self.column + 1,
            timeline: self.timeline,
        }
    }
}

impl Manifold {
    fn launch_beam(&self) -> (u32, u64) {
        let splits = AtomicU32::new(0);
        let mut beam_posistions = HashSet::from([self.source]);

        while beam_posistions
            .par_iter()
            .all(|position| position.row < self.size)
        {
            beam_posistions = beam_posistions
                .iter()
                .flat_map(|position| {
                    let next = position.down();

                    if let Some(element) = self.content.get(&next)
                        && *element == Element::Splitter
                    {
                        splits.fetch_add(1, Relaxed);
                        vec![next.left(), next.right()]
                    } else {
                        vec![next]
                    }
                })
                .fold(HashMap::new(), |acc, position| {
                    let mut new = acc.clone();
                    if let Some(previous) = acc.get(&position) {
                        new.insert(position, *previous + position.timeline);
                    } else {
                        new.insert(position, position.timeline);
                    }

                    new
                })
                .iter()
                .map(|(position, timeline)| Position {
                    row: position.row,
                    column: position.column,
                    timeline: *timeline,
                })
                .collect();
        }

        (
            splits.fetch_add(0, Relaxed),
            beam_posistions
                .iter()
                .map(|position| position.timeline)
                .sum(),
        )
    }
}

fn main() {
    let content = read_to_string("./day7/files/input").expect("Invalid file");
    let manifold = Manifold::from_str(&content).expect("Invalid manifold");
    let (part1, part2) = manifold.launch_beam();

    println!("Sum for part 1 is {}", part1);
    println!("Sum for part 2 is {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_DATA: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[test]
    fn test_parse_manifold() {
        let manifold = Manifold::from_str(TEST_DATA);
        let expected = Manifold {
            source: Position {
                row: 0,
                column: 7,
                timeline: 1,
            },
            content: HashMap::from([
                (
                    Position {
                        row: 0,
                        column: 7,
                        timeline: 1,
                    },
                    Element::Source,
                ),
                (
                    Position {
                        row: 2,
                        column: 7,
                        timeline: 1,
                    },
                    Element::Splitter,
                ),
                (
                    Position {
                        row: 4,
                        column: 6,
                        timeline: 1,
                    },
                    Element::Splitter,
                ),
                (
                    Position {
                        row: 4,
                        column: 8,
                        timeline: 1,
                    },
                    Element::Splitter,
                ),
                (
                    Position {
                        row: 6,
                        column: 5,
                        timeline: 1,
                    },
                    Element::Splitter,
                ),
                (
                    Position {
                        row: 6,
                        column: 7,
                        timeline: 1,
                    },
                    Element::Splitter,
                ),
                (
                    Position {
                        row: 6,
                        column: 9,
                        timeline: 1,
                    },
                    Element::Splitter,
                ),
                (
                    Position {
                        row: 8,
                        column: 4,
                        timeline: 1,
                    },
                    Element::Splitter,
                ),
                (
                    Position {
                        row: 8,
                        column: 6,
                        timeline: 1,
                    },
                    Element::Splitter,
                ),
                (
                    Position {
                        row: 8,
                        column: 10,
                        timeline: 1,
                    },
                    Element::Splitter,
                ),
                (
                    Position {
                        row: 10,
                        column: 3,
                        timeline: 1,
                    },
                    Element::Splitter,
                ),
                (
                    Position {
                        row: 10,
                        column: 5,
                        timeline: 1,
                    },
                    Element::Splitter,
                ),
                (
                    Position {
                        row: 10,
                        column: 9,
                        timeline: 1,
                    },
                    Element::Splitter,
                ),
                (
                    Position {
                        row: 10,
                        column: 11,
                        timeline: 1,
                    },
                    Element::Splitter,
                ),
                (
                    Position {
                        row: 12,
                        column: 2,
                        timeline: 1,
                    },
                    Element::Splitter,
                ),
                (
                    Position {
                        row: 12,
                        column: 6,
                        timeline: 1,
                    },
                    Element::Splitter,
                ),
                (
                    Position {
                        row: 12,
                        column: 12,
                        timeline: 1,
                    },
                    Element::Splitter,
                ),
                (
                    Position {
                        row: 14,
                        column: 1,
                        timeline: 1,
                    },
                    Element::Splitter,
                ),
                (
                    Position {
                        row: 14,
                        column: 3,
                        timeline: 1,
                    },
                    Element::Splitter,
                ),
                (
                    Position {
                        row: 14,
                        column: 5,
                        timeline: 1,
                    },
                    Element::Splitter,
                ),
                (
                    Position {
                        row: 14,
                        column: 7,
                        timeline: 1,
                    },
                    Element::Splitter,
                ),
                (
                    Position {
                        row: 14,
                        column: 9,
                        timeline: 1,
                    },
                    Element::Splitter,
                ),
                (
                    Position {
                        row: 14,
                        column: 13,
                        timeline: 1,
                    },
                    Element::Splitter,
                ),
            ]),
            size: 16,
        };
        assert_eq!(manifold, Ok(expected));
    }

    #[test]
    fn test_example_part_1() {
        let manifold = Manifold::from_str(TEST_DATA).expect("Invalid manifold");

        assert_eq!(manifold.launch_beam().0, 21)
    }

    #[test]
    fn test_example_part_2() {
        let manifold = Manifold::from_str(TEST_DATA).expect("Invalid manifold");

        assert_eq!(manifold.launch_beam().1, 40)
    }
}
