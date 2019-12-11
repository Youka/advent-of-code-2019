// Helpers
type Point = (u16,u16);
fn angle_along_points(p1: Point, p2: Point) -> f32 {
    (p2.0 as f32 - p1.0 as f32).atan2(p1.1 as f32 - p2.1 as f32)
}
fn distance_between_points(p1: Point, p2: Point) -> f32 {
    (p2.0 as f32 - p1.0 as f32).hypot(p2.1 as f32 - p1.1 as f32)
}
fn point_on_line(line: (Point, Point), point: Point) -> bool {
    angle_along_points(line.0, point) == angle_along_points(line.0, line.1) &&
    distance_between_points(line.0, point) <= distance_between_points(line.0, line.1)
}

// Parts
fn part1(asteroids: &[Point]) -> Option<(&Point,u32)> {
    // Find observables per asteroid
    asteroids.iter().map(|asteroid|
        (
            asteroid,
            // Check possible observings
            asteroids.iter().filter(|observable| *observable != asteroid ).fold(0, |mut amount, observable| {
                // Any blockade between asteroid and possible observable?
                if !asteroids.iter().filter(|blockade| *blockade != observable && *blockade != asteroid ).any(|blockade| point_on_line((*asteroid, *observable), *blockade) ) {
                    amount += 1;
                }
                amount
            })
        )
    )
    // Find asteroid with most observings
    .max_by(|o1, o2| o1.1.cmp(&o2.1) )
}
fn part2<'a>(asteroids: &'a [Point], observer: &Point) -> Option<u16> {
    use std::{
        cmp::Ordering,
        f32::consts::PI
    };
    // Get all targets by coordinate, angle and distance
    let mut targets = asteroids.iter()
        .filter(|asteroid| *asteroid != observer )
        .map(|asteroid| (
            asteroid,
            {
                let mut angle = angle_along_points(*observer, *asteroid);
                if angle < 0.0 {
                    angle = 2.0 * PI + angle;
                }
                angle
            },
            distance_between_points(*observer, *asteroid)
        ))
        .collect::<Vec<_>>();
    // Sort targets clockwise first, distance second
    targets.sort_by(|t1, t2| {
        let cmp_angles = t1.1.partial_cmp(&t2.1).unwrap();
        match cmp_angles {
            Ordering::Equal => t1.2.partial_cmp(&t2.2).unwrap(),
            _ => cmp_angles
        }
    });
    // Find 200th target hit
    let mut hits = vec![];
    let mut angle = -1.0;
    for target in targets.iter().cycle() {
        if !hits.contains(&target) && target.1 != angle {
            hits.push(target);
            if hits.len() == 200 {
                return Some((target.0).0 * 100 + (target.0).1);
            }
            angle = target.1;
        }
    }
    None
}

// Day 10
fn main() {
    // Input
    use std::io::{stdin,Read};
    let asteroids = {
        let mut pos = (0,0);
        stdin().lock()
            .bytes()
            .map(Result::unwrap)
            .filter_map(|byte|
                match byte {
                    b'#' => {
                        let asteroid = Some(pos.clone());
                        pos.0 += 1;
                        asteroid
                    }
                    b'\n' => {
                        pos = (0,pos.1+1);
                        None
                    }
                    _ => {
                        pos.0 += 1;
                        None
                    }
                }
            ).collect::<Vec<_>>()
    };
    // Puzzles
    let observer = part1(&asteroids).expect("Observer needs to be found!");
    println!("[Part 1] Asteroids visible: {:?}", observer.1);
    println!("[Part 2] 200th hit: {:?}", part2(&asteroids, observer.0));
}