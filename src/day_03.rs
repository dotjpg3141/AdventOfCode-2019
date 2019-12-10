use std::io::BufRead;
use std::*;

pub(crate) fn run() -> Result<(), Box<dyn error::Error>> {
    let file = fs::File::open("input/day03.txt")?;
    let mut lines = io::BufReader::new(file).lines();

    let path1 = parse_path(&lines.next().unwrap()?);
    let path2 = parse_path(&lines.next().unwrap()?);

    let intersections = {
        let path1 = path1.iter().cloned().collect::<collections::HashSet<_>>();
        let path2 = path2.iter().cloned().collect::<collections::HashSet<_>>();
        path1.intersection(&path2).cloned().collect::<Vec<_>>()
    };

    let min = intersections.iter().map(Vector2::manhatten_distance).min();
    println!("Day 3a: {:?}", min);

    let min = intersections
        .into_iter()
        .map(|intersection| {
            let steps1 = path1
                .iter()
                .cloned()
                .position(|pos| pos == intersection)
                .unwrap();
            let steps2 = path2
                .iter()
                .cloned()
                .position(|pos| pos == intersection)
                .unwrap();

            steps1 + steps2 + 2
        })
        .min();
    println!("Day 3b: {:?}", min);

    Ok(())
}

fn parse_path(s: &str) -> Vec<Vector2> {
    s.split(',')
        .map(parse_direction)
        .scan(Vector2::default(), |state, dir| {
            let start = *state;
            state.x += dir.x;
            state.y += dir.y;
            Some((start, dir))
        })
        .flat_map(|(start, dir)| {
            let xs = dir.x.signum();
            let ys = dir.y.signum();
            let magnitude = dir.manhatten_distance();

            (1..=magnitude).map(move |m| Vector2 {
                x: start.x + m * xs,
                y: start.y + m * ys,
            })
        })
        .collect()
}

fn parse_direction(s: &str) -> Vector2 {
    let mut s = s.chars();
    let mut v = Vector2::default();
    let direction = s.next();
    let magnitude = s.as_str().parse::<isize>().unwrap();

    match direction {
        Some('L') => v.x = -magnitude,
        Some('R') => v.x = magnitude,
        Some('U') => v.y = -magnitude,
        Some('D') => v.y = magnitude,
        other => panic!("Invalid direction: {:?}", other),
    };

    v
}

#[derive(Default, Copy, Clone, PartialEq, Eq, Hash)]
struct Vector2 {
    x: isize,
    y: isize,
}

impl Vector2 {
    fn manhatten_distance(&self) -> isize {
        self.x.abs() + self.y.abs()
    }
}
