// Traits
use std::str::FromStr;

// Direction
#[derive(Debug,Clone,Copy,Eq,PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}
impl FromStr for Direction {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Self::Up),
            "D" => Ok(Self::Down),
            "L" => Ok(Self::Left),
            "R" => Ok(Self::Right),
            _ => Err(format!("Invalid direction string: {}", s))
        }
    }
}

// Move
#[derive(Debug,Clone,Copy,Eq,PartialEq)]
struct Move {
    pub direction: Direction,
    pub units: isize
}
impl FromStr for Move {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 2 {
            return Err(format!("String is too short for a direction + units: {}", s));
        }
        let (direction, units) = s.split_at(1);
        Ok(Move {
            direction: direction.parse().map_err(|_| format!("Invalid direction for move: {}", direction) )?,
            units: units.parse().map_err(|_| format!("Invalid units for move: {}", units) )?
        })
    }
}
impl Move {
    pub fn len(&self) -> usize {
        self.units.abs() as usize
    }
}

// Point
#[derive(Debug,Clone,Copy,Eq,PartialEq)]
struct Point(isize,isize);
impl Point {
    pub fn movement(&mut self, mov: Move) {
        match mov.direction {
            Direction::Up => self.1 += mov.units,
            Direction::Down => self.1 -= mov.units,
            Direction::Left => self.0 -= mov.units,
            Direction::Right => self.0 += mov.units
        }
    }
    pub fn distance(&self) -> usize {
        self.0.abs() as usize + self.1.abs() as usize
    }
}

// Line
#[derive(Debug,Clone,Copy,Eq,PartialEq)]
struct Line {
    pub point: Point,
    pub mov: Move
}
impl Line {
    fn end_point(&self) -> Point {
        let mut point = self.point.clone();
        point.movement(self.mov);
        point
    }
    pub fn intersect(&self, other: &Line) -> Option<Point> {
        // Find horizontal and vertical line
        let mut hline = None;
        let mut vline = None;
        match self.mov.direction {
            Direction::Up | Direction::Down => vline = Some(self),
            Direction::Left | Direction::Right => hline = Some(self)
        };
        match other.mov.direction {
            Direction::Up | Direction::Down => vline = Some(other),
            Direction::Left | Direction::Right => hline = Some(other)
        };
        // Lines orthogonal?
        if let (Some(hline), Some(vline)) = (hline, vline) {
            // Lines intersect?
            let hline_end_point = hline.end_point();
            let vline_end_point = vline.end_point();
            let hrange = hline.point.0.min(hline_end_point.0)..=hline.point.0.max(hline_end_point.0);
            let vrange = vline.point.1.min(vline_end_point.1)..=vline.point.1.max(vline_end_point.1);
            if hrange.contains(&vline.point.0) && vrange.contains(&hline.point.1) {
                return Some(Point(vline.point.0, hline.point.1))
            }
        }
        None
    }
    pub fn point_inner_distance(&self, point: &Point) -> Option<usize> {
        match self.mov.direction {
            Direction::Up | Direction::Down => if point.0 == self.point.0 {
                let end_point = self.end_point();
                let vrange = self.point.1.min(end_point.1)..=self.point.1.max(end_point.1);
                if vrange.contains(&point.1) {
                    return Some((point.1 - self.point.1).abs() as usize)
                }
            },
            Direction::Left | Direction::Right => if point.1 == self.point.1 {
                let end_point = self.end_point();
                let hrange = self.point.0.min(end_point.0)..=self.point.0.max(end_point.0);
                if hrange.contains(&point.0) {
                    return Some((point.0 - self.point.0).abs() as usize)
                }
            }
        }
        None
    }
}

// Lines of one wire
fn wire_lines_from_str(s: &str) -> Vec<Line> {
    let mut point = Point(0,0);
    s.split(',')
        .map(|token| token.parse().expect(&format!("Token isn't a move: {}", token)) )
        .map(|mov| {
            let line = Line {point, mov};
            point.movement(mov);
            line
        })
        .collect()
}
fn wire_lines_distance_to_point(lines: &[Line], point: &Point) -> Option<usize> {
    let mut distance = 0;
    for line in lines {
        if let Some(point_distance) = line.point_inner_distance(point) {
            return Some(distance + point_distance);
        }
        distance += line.mov.len();
    }
    None
}

// Day 3 puzzle
fn main() {
    use std::io::{stdin,BufRead};
    // Read two wires from input
    let lines = stdin().lock()
        .lines()
        .take(2)
        .map(|result_line| result_line.expect("Text line expected!") )
        .collect::<Vec<String>>();
    if lines.len() != 2 {
        panic!("Two wires expected from input!")
    }
    // Convert wires into lines
    let wire1_lines = wire_lines_from_str(&lines[0]);
    let wire2_lines = wire_lines_from_str(&lines[1]);
    // Part 1: Find the nearest wire intersection to origin
    // Part 2: Find the shortest (& combined) way of intersecting wires from origin
    let mut nearest_distance = None;
    let mut shortest_distance = None;
    for wire1_line in &wire1_lines {
        for wire2_line in &wire2_lines {
            if let Some(point) = wire1_line.intersect(&wire2_line) {
                let point_distance = point.distance();
                if point_distance != 0 {
                    // Part 1
                    match nearest_distance {
                        None => nearest_distance = Some(point_distance),
                        Some(old_point_distance) if old_point_distance > point_distance => nearest_distance = Some(point_distance),
                        _ => ()
                    }
                    // Part 2
                    let wire_distance =
                        wire_lines_distance_to_point(&wire1_lines, &point).expect("Point should be on wire 1!") +
                        wire_lines_distance_to_point(&wire2_lines, &point).expect("Point should be on wire 2!");
                    match shortest_distance {
                        None => shortest_distance = Some(wire_distance),
                        Some(old_wire_distance) if old_wire_distance > wire_distance => shortest_distance = Some(wire_distance),
                        _ => ()
                    }
                }
            }
        }
    }
    println!("Nearest distance: {:?}", nearest_distance);
    println!("Shortest distance: {:?}", shortest_distance);
}