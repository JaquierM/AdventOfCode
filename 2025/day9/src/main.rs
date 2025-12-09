use itertools::Itertools;
use std::cmp::PartialEq;
use std::fs::read_to_string;
use std::str::FromStr;
use std::time::Instant;

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Rectangle {
    corners: (Position, Position),
    area: i64,
}

#[derive(Debug)]
struct Edge {
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
}

impl FromStr for Position {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let coordinates: Vec<_> = input
            .split(",")
            .map(|part| part.parse::<i32>().map_err(|_| String::from("Invalid int")))
            .collect();

        if coordinates.len() != 2 {
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
            })
        }
    }
}

impl Rectangle {
    fn new(corners: (Position, Position)) -> Self {
        let area: i64 = ((corners.1.x - corners.0.x).abs() + 1) as i64
            * ((corners.1.y - corners.0.y).abs() + 1) as i64;
        Rectangle { corners, area }
    }

    fn max_x(&self) -> i32 {
        self.corners.0.x.max(self.corners.1.x)
    }

    fn min_x(&self) -> i32 {
        self.corners.0.x.min(self.corners.1.x)
    }

    fn max_y(&self) -> i32 {
        self.corners.0.y.max(self.corners.1.y)
    }

    fn min_y(&self) -> i32 {
        self.corners.0.y.min(self.corners.1.y)
    }
}

impl Edge {
    fn new(start: &Position, end: &Position) -> Self {
        Self {
            x_max: start.x.max(end.x),
            x_min: start.x.min(end.x),
            y_max: start.y.max(end.y),
            y_min: start.y.min(end.y),
        }
    }
}

fn main() {
    let start_parsing = Instant::now();
    let content = read_to_string("./day9/files/input").expect("Invalid file");
    let corners = parse(&content);
    let rectangles = create_rectangles(&corners);
    let parsing_time = Instant::now() - start_parsing;
    println!("Parsing took [{parsing_time:?}]");

    let start_part1 = Instant::now();
    let part1 = find_biggest(&rectangles);
    let part1_time = Instant::now() - start_part1;

    let start_part2 = Instant::now();
    let part2 = find_biggest_inside_limit(&corners, &rectangles);
    let part2_time = Instant::now() - start_part2;

    println!("[{part1_time:?}] Sum for part 1 is {part1}");
    println!("[{part2_time:?}] Sum for part 2 is {part2}");
}

fn parse(input: &str) -> Vec<Position> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| Position::from_str(line).expect("Invalid coordinate"))
        .collect()
}

fn create_rectangles(corners: &[Position]) -> Vec<Rectangle> {
    corners
        .iter()
        .tuple_combinations()
        .map(|(p1, p2)| Rectangle::new((*p1, *p2)))
        .sorted_by_key(|rect| rect.area)
        .rev()
        .collect()
}

fn find_biggest(rectangles: &[Rectangle]) -> i64 {
    rectangles.first().map(|rect| rect.area).unwrap_or(0)
}

fn find_biggest_inside_limit(corners: &[Position], rectangles: &[Rectangle]) -> i64 {
    let edges: Vec<Edge> = corners
        .iter()
        .circular_tuple_windows()
        .map(|(start, end)| Edge::new(start, end))
        .collect();

    rectangles
        .iter()
        .find(|rectangle| is_fully_contain(&edges, rectangle))
        .map(|rect| rect.area)
        .unwrap_or(0)
}

fn is_fully_contain(edges: &[Edge], rectangle: &Rectangle) -> bool {
    edges.iter().all(|edge| {
        let rect_left_of_edge = rectangle.max_x() <= edge.x_min;
        let rect_right_of_edge = rectangle.min_x() >= edge.x_max;
        let rect_above_of_edge = rectangle.max_y() <= edge.y_min;
        let rect_below_of_edge = rectangle.min_y() >= edge.y_max;

        rect_left_of_edge || rect_right_of_edge || rect_above_of_edge || rect_below_of_edge
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_DATA: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    #[test]
    fn test_rectangle() {
        assert_eq!(
            Rectangle::new((Position { x: 2, y: 5 }, Position { x: 9, y: 7 })).area,
            24
        );
        assert_eq!(
            Rectangle::new((Position { x: 7, y: 1 }, Position { x: 11, y: 7 })).area,
            35
        );
        assert_eq!(
            Rectangle::new((Position { x: 7, y: 3 }, Position { x: 2, y: 3 })).area,
            6
        );
    }

    #[test]
    fn test_example_part_1() {
        let corners = parse(TEST_DATA);
        let biggest_area = create_rectangles(&corners)
            .iter()
            .max_by_key(|rect| rect.area)
            .map(|rect| rect.area)
            .unwrap_or(0);

        assert_eq!(biggest_area, 50);
    }

    #[test]
    fn test_example_part_2() {
        let corners = parse(TEST_DATA);
        let rectangles = create_rectangles(&corners);

        assert_eq!(find_biggest_inside_limit(&corners, &rectangles), 24);
    }
}
