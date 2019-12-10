use std::convert::TryInto;
use std::io::BufRead;
use std::*;

pub(crate) fn run() -> Result<(), Box<dyn error::Error>> {
    let file = fs::File::open("input/day10.txt")?;
    let lines = io::BufReader::new(file).lines();

    let mut asteroids = Vec::new();
    for (y, line) in lines.enumerate() {
        for (x, character) in line?.chars().enumerate() {
            if character == '#' {
                asteroids.push(Vec2 {
                    x: x.try_into()?,
                    y: y.try_into()?,
                });
            }
        }
    }

    let (source, asteroids_by_direction) = asteroids
        .iter()
        .copied()
        .map(|source| {
            let mut map = collections::HashMap::<Vec2, Vec<Vec2>>::new();
            for dest in asteroids.iter().copied() {
                let Vec2 {
                    x: mut dx,
                    y: mut dy,
                } = dest - source;

                if dx == 0 && dy == 0 {
                    continue;
                }

                if dx == 0 {
                    dy = dy.signum();
                } else if dy == 0 {
                    dx = dx.signum();
                } else {
                    let gcd = gcd(dx.abs(), dy.abs());
                    dx /= gcd;
                    dy /= gcd;
                }

                let direction = Vec2 { x: dx, y: dy };
                map.entry(direction).or_default().push(dest);
            }
            (source, map)
        })
        .max_by_key(|(_, map)| map.len())
        .unwrap();
    println!("Day 10a: {:?}", asteroids_by_direction.len());

    let mut sorted_asteroids_buckets = asteroids_by_direction.into_iter().collect::<Vec<_>>();
    sorted_asteroids_buckets.sort_unstable_by(|(direction1, _), (direction2, _)| {
        partial_cmp(&direction1.angle(), &direction2.angle())
    });

    let mut n = 200;
    let mut depth = 0;

    let position = 'outer: loop {
        for (_direction, bucket) in sorted_asteroids_buckets.iter_mut() {
            if depth >= bucket.len() {
                continue;
            }

            n -= 1;
            if n == 0 {
                bucket.sort_unstable_by_key(|asteroid| (*asteroid - source).len_sqared());
                break 'outer bucket[depth];
            }
        }

        depth += 1;
    };
    let result = position.x * 100 + position.y;
    println!("Day 10b: {:?}", result);

    Ok(())
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Vec2 {
    x: i32,
    y: i32,
}

impl Vec2 {
    pub fn angle(self) -> f32 {
        let angle = f32::atan2(self.y as f32, self.x as f32) + f32::consts::FRAC_PI_2;
        if angle < 0.0 {
            angle + f32::consts::PI * 2.0
        } else {
            angle
        }
    }

    pub fn len_sqared(self) -> i32 {
        self.x * self.x + self.y * self.y
    }
}

impl ops::Sub<Vec2> for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let temp = a;
        a = b;
        b = temp % a;
    }
    a
}

fn partial_cmp<Lhs, Rhs>(lhs: &Lhs, rhs: &Rhs) -> cmp::Ordering
where
    Lhs: PartialOrd<Rhs>,
{
    lhs.partial_cmp(rhs).unwrap_or(cmp::Ordering::Equal)
}
