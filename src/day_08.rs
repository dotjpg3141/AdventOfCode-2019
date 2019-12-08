use itertools::*;
use std::io::Read;
use std::*;

const CHAR_BLACK: char = ' ';
const CHAR_WHITE: char = '█';

pub(crate) fn run() -> Result<(), Box<dyn error::Error>> {
    let file = fs::File::open("input/day8.txt")?;

    let width = 25;
    let height = 6;

    let layers = file
        .bytes()
        .flat_map(|b| {
            let b = b.unwrap();
            if b < b'0' {
                None
            } else {
                Some(b - b'0')
            }
        })
        .chunks(width * height)
        .into_iter()
        .map(|layer| layer.collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let result = layers
        .iter()
        .min_by_key(|layer| count_digit(layer.iter().copied(), 0))
        .map(|layer| {
            let digit1 = count_digit(layer.iter().copied(), 1);
            let digit2 = count_digit(layer.iter().copied(), 2);
            digit1 * digit2
        });
    println!("Day 8a: {:?}", result);

    let mut image = vec![2; width * height];
    for layer in layers {
        for (index, pixel) in layer.iter().copied().enumerate() {
            if image[index] == 2 {
                image[index] = pixel;
            }
        }
    }

    println!("Day 8b:");
    let mut image = image.into_iter();
    for _y in 0..height {
        for _x in 0..width {
            let pixel = image.next().unwrap();
            let color = if pixel == 0 { CHAR_BLACK } else { CHAR_WHITE };
            print!("{}", color);
        }
        println!();
    }

    Ok(())
}

fn count_digit(iter: impl Iterator<Item = u8>, digit: u8) -> usize {
    iter.filter(|curr| *curr == digit).count()
}
