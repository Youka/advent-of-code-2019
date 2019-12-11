// Map
type Dimension = (u16,u16);
struct AsteroidMap {
    data: Vec<u8>,
    dimension: Dimension
}
impl AsteroidMap {
    pub fn new(grid: &Vec<Vec<u8>>) -> Result<Self,&'static str> {
        // Get dimension
        let width = grid.first().and_then(|first_row|
            if grid.iter().skip(1).all(|row| row.len() == first_row.len()) {Some(first_row.len())} else {None}
        ).ok_or("Invalid map format!")?;
        let height = grid.len();
        // Create new instance
        Ok(Self{
            data: grid.concat(),
            dimension: (width as u16,height as u16)
        })
    }
    #[allow(dead_code)]
    pub fn dimension(&self) -> Dimension {
        self.dimension
    }
    pub fn data(&self) -> &[u8] {
        &self.data
    }
    pub fn index_to_xy(&self, index: usize) -> Dimension {
        ( (index % self.dimension.0 as usize) as u16, (index / self.dimension.0 as usize) as u16 )
    }
    #[allow(dead_code)]
    pub fn xy_to_index(&self, xy: Dimension) -> usize {
        xy.1 as usize * self.dimension.0 as usize + xy.0 as usize
    }
}

// Helpers
fn point_on_line(line: (Dimension, Dimension), point: Dimension) -> bool {
    let l1_p = ( ((line.0).0 as f32 - point.0 as f32), ((line.0).1 as f32 - point.1 as f32) );
    let l1_l2 = ( ((line.0).0 as f32 - (line.1).0 as f32), ((line.0).1 as f32 - (line.1).1 as f32) );
    let angle_l1_p = l1_p.0.atan2(l1_p.1);
    let angle_l1_l2 = l1_l2.0.atan2(l1_l2.1);
    let distance_l1_p = l1_p.0.hypot(l1_p.1);
    let distance_l1_l2 = l1_l2.0.hypot(l1_l2.1);
    angle_l1_p == angle_l1_l2 && distance_l1_p <= distance_l1_l2
}

// Parts
fn part1(map: &AsteroidMap) -> Option<u32> {
    // Collect asteroid positions
    let asteroids = map.data().into_iter()
        .enumerate()
        .filter(|(_,field)| *field == &b'#' )
        .map(|(index, _)| map.index_to_xy(index) )
        .collect::<Vec<_>>();
    // Find asteroid with most observings
    asteroids.iter().map(|observer|
        // Check possible observing
        asteroids.iter().filter(|observable| *observable != observer ).fold(0_u32, |mut amount, observable| {
            // Any blockage between observer and possible observable?
            if !asteroids.iter().filter(|blockade| *blockade != observable && *blockade != observer ).any(|blockade| point_on_line((*observer, *observable), *blockade) ) {
                amount += 1;
            }
            amount
        })
    ).max()
}

// Day 10
fn main() {
    // Read input
    use std::io::{stdin,BufRead};
    let map = AsteroidMap::new(
        &stdin().lock()
            .lines()
            .map(|result_line| result_line.expect("Text line expected!").into_bytes() )
            .collect()
    ).expect("Couldn't construct AsteroidMap from input!");
    // Puzzles
    println!("[Part 1] Asteroids visible: {:?}", part1(&map));
}