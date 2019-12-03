use std::str::FromStr;

// Direction
#[derive(Debug,Clone,Copy)]
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
#[derive(Debug,Clone,Copy)]
struct Move {
    pub direction: Direction,
    pub units: isize
}
impl FromStr for Move {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 2 {
            return Err(format!("String is too short: {}", s));
        }
        let (direction, units) = s.split_at(1);
        Ok(Move {
            direction: direction.parse().map_err(|_| format!("Invalid direction for move: {}", direction) )?,
            units: units.parse().map_err(|_| format!("Invalid units for move: {}", units) )?
        })
    }
}

// Point
#[derive(Debug,Clone,Copy)]
struct Point(isize,isize);
impl Point {
    pub fn movement(&mut self, mov: Move) -> &mut Self {
        match mov.direction {
            Direction::Up => self.1 += mov.units,
            Direction::Down => self.1 -= mov.units,
            Direction::Left => self.0 -= mov.units,
            Direction::Right => self.0 += mov.units
        }
        self
    }
    pub fn len(&self) -> usize {
        self.0.abs() as usize + self.1.abs() as usize
    }
}

// Line
#[derive(Debug,Clone,Copy)]
struct Line {
    pub point: Point,
    pub mov: Move
}
impl Line {
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
            // Line end points
            let mut hline_end_point = hline.point.clone();
            hline_end_point.movement(hline.mov);
            let mut vline_end_point = vline.point.clone();
            vline_end_point.movement(vline.mov);
            // Line ranges
            let hrange = hline.point.0.min(hline_end_point.0)..=hline.point.0.max(hline_end_point.0);
            let vrange = vline.point.1.min(vline_end_point.1)..=vline.point.1.max(vline_end_point.1);
            // Lines intersect?
            if hrange.contains(&vline.point.0) && vrange.contains(&hline.point.1) {
                return Some(Point(hline.point.1, vline.point.0))
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

// Find the closest intersection point of wires from input
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
    // Find the nearest wire intersection to origin
    let mut nearest_distance = None;
    for wire1_line in &wire1_lines {
        for wire2_line in &wire2_lines {
            if let Some(point) =  wire1_line.intersect(&wire2_line) {
                let distance = point.len();
                if distance != 0 {
                    match nearest_distance {
                        None => nearest_distance = Some(distance),
                        Some(old_distance) if old_distance > distance => nearest_distance = Some(distance),
                        _ => ()
                    }
                }
            }
        }
    }
    println!("Nearest distance: {:?}", nearest_distance);
}