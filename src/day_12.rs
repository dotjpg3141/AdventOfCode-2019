use std::io::BufRead;
use std::*;

pub(crate) fn run() -> Result<(), Box<dyn error::Error>> {
    let file = fs::File::open("input/day12.txt")?;

    let mut moons = io::BufReader::new(file)
        .lines()
        .map(|line| -> Result<_, num::ParseIntError> {
            Ok(Moon {
                position: V3::parse(&line.unwrap()),
                velocity: V3::default(),
            })
        })
        .collect::<Result<Vec<_>, _>>()?;

    let mut gravity = vec![V3::default(); moons.len()];

    for _step in 0..1000 {
        // println!("After {} steps:", _step);
        // for moon in &moons {
        //     println!("{}", moon);
        // }
        // println!();

        // calculate gravity
        for gravity in &mut gravity {
            *gravity = V3::default();
        }
        for (index_a, moon_a) in moons.iter().enumerate() {
            for (index_b, moon_b) in moons.iter().enumerate().skip(index_a + 1) {
                let (dx_a, dx_b) = apply_gravity(moon_a.position.x, moon_b.position.x);
                let (dy_a, dy_b) = apply_gravity(moon_a.position.y, moon_b.position.y);
                let (dz_a, dz_b) = apply_gravity(moon_a.position.z, moon_b.position.z);

                gravity[index_a].x += dx_a;
                gravity[index_a].y += dy_a;
                gravity[index_a].z += dz_a;

                gravity[index_b].x += dx_b;
                gravity[index_b].y += dy_b;
                gravity[index_b].z += dz_b;
            }
        }

        // calculate velocity
        for (index, moon) in moons.iter_mut().enumerate() {
            let gravity = gravity[index];
            moon.velocity.x += gravity.x;
            moon.velocity.y += gravity.y;
            moon.velocity.z += gravity.z;
        }

        // calculate position
        for moon in &mut moons {
            moon.position.x += moon.velocity.x;
            moon.position.y += moon.velocity.y;
            moon.position.z += moon.velocity.z;
        }
    }

    let mut result = 0;
    for moon in &moons {
        let potential_energy = moon.position.manhatten_len();
        let kinetic_energy = moon.velocity.manhatten_len();
        let total_energy = potential_energy * kinetic_energy;
        result += total_energy;
    }

    println!("Day 12: {:?}", result);

    Ok(())
}

fn apply_gravity(a: i32, b: i32) -> (i32, i32) {
    match a.cmp(&b) {
        cmp::Ordering::Less => (1, -1),
        cmp::Ordering::Equal => (0, 0),
        cmp::Ordering::Greater => (-1, 1),
    }
}

#[derive(Debug, Copy, Clone, Default)]
struct V3 {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Debug, Copy, Clone)]
struct Moon {
    position: V3,
    velocity: V3,
}

impl V3 {
    fn parse(s: &str) -> V3 {
        let mut items = s.split(|c| c == '=' || c == ',' || c == '>');

        items.next().unwrap();
        let x = items.next().unwrap().parse::<i32>().unwrap();

        items.next().unwrap();
        let y = items.next().unwrap().parse::<i32>().unwrap();

        items.next().unwrap();
        let z = items.next().unwrap().parse::<i32>().unwrap();

        V3 { x, y, z }
    }

    fn manhatten_len(&self) -> i32 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

impl fmt::Display for V3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<x={:>3},y={:>3},z={:>3}>", self.x, self.y, self.z)
    }
}

impl fmt::Display for Moon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "pos={}, vel={}", self.position, self.position)
    }
}
