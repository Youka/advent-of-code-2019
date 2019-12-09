// Image (space format)
type Pixels = Vec<u8>;
struct SpaceImage {
    width: u16,
    layers: Vec<Pixels>
}
impl SpaceImage {
    pub fn new_from_bytes(width: u16, height: u16, bytes: &[u8]) -> Result<Self,&str> {
        let layers = bytes.chunks_exact(width as usize * height as usize);
        if layers.remainder().len() > 0 {
            Err("Image bytes doesn't fit in given width * height layers!")
        } else if bytes.iter().any(|pixel| *pixel > 2 ) {
            Err("Image bytes have to be in range 0-2!")
        } else {
            Ok(Self{
                width,
                layers: layers.map(|chunk| chunk.to_vec() ).collect()
            })
        }
    }
    pub fn width(&self) -> u16 {
        self.width
    }
    pub fn layers(&self) -> &[Pixels] {
        &self.layers
    }
}

// Helpers
fn count<Item>(items: &[Item], find: &Item) -> usize where Item: Eq {
    items.iter().filter(|item| *item == find ).count()
}

// Parts
fn part1(image: &SpaceImage) -> Option<usize> {
    let low_zero_layer = image.layers().iter()
        .map(|pixels| (pixels, count(pixels, &0) ))
        .min_by(|pixels1, pixels2| pixels1.1.cmp(&pixels2.1) )
        .map(|(pixels, _)| pixels )?;
    Some( count(low_zero_layer, &1) * count(low_zero_layer, &2) )
}
fn part2(image: &SpaceImage) -> Option<String> {
    let mut merged_layers = image.layers().first()?.clone();
    for layer in image.layers().iter().skip(1) {
        for (i, pixel) in merged_layers.iter_mut().enumerate() {
            if *pixel == 2 && layer[i] != 2 {
                *pixel = layer[i]
            }
        }
    }
    Some(
        merged_layers
        .chunks(image.width() as usize)
        .map(|row|
            String::from_utf8(row.iter().map(|pixel|
                match pixel {
                    1 => b'#',  // White
                    _ => b' '   // Black, transparent, everything else
                }
            ).collect()).expect("Only ascii expected!")
        )
        .collect::<Vec<_>>()
        .join("\n")
    )
}

// Day 8
fn main() {
    // Input
    use std::io::{stdin,Read};
    let input = stdin().lock()
        .bytes()
        .map(|result_byte| result_byte.unwrap() - b'0' )
        .collect::<Vec<_>>();
    let image = SpaceImage::new_from_bytes(25, 6, &input).expect("Image invalid!");
    // Puzzles
    println!("Part 1: {:?}", part1(&image).expect("Image mustn't be empty!"));
    println!("Part 2:\n{}", part2(&image).expect("Image mustn't be empty!"));
}