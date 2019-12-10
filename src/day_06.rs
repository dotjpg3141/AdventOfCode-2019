use std::io::BufRead;
use std::*;

pub(crate) fn run() -> Result<(), Box<dyn error::Error>> {
    let file = fs::File::open("input/day06.txt")?;

    let lines = io::BufReader::new(file)
        .lines()
        .collect::<Result<Vec<_>, _>>()?;

    let mut orbits = collections::HashMap::<String, Vec<String>>::new();
    for line in lines {
        let mut parts = line.split(')');
        let a = parts.next().unwrap().to_owned();
        let b = parts.next().unwrap().to_owned();
        orbits.entry(a).or_default().push(b);
    }

    let mut result = 0;
    for obj in orbits.keys() {
        result += count_descendants(&orbits, obj);
    }
    println!("Day 6a: {}", result);

    let mut san_path = Vec::new();
    get_path(&orbits, "COM", "SAN", &mut san_path);

    let mut you_path = Vec::new();
    get_path(&orbits, "COM", "YOU", &mut you_path);

    while san_path.last() == you_path.last() {
        san_path.pop();
        you_path.pop();
    }

    let result = san_path.len() + you_path.len() - 2;
    println!("Day 6b: {}", result);
    Ok(())
}

fn count_descendants<'a>(
    map: &'a collections::HashMap<String, Vec<String>>,
    start: &'a str,
) -> usize {
    let mut count = 0;

    if let Some(children) = map.get(start) {
        for child in children {
            count += count_descendants(map, child) + 1;
        }
    }

    count
}

fn get_path<'a>(
    map: &'a collections::HashMap<String, Vec<String>>,
    start: &'a str,
    dest: &'a str,
    path: &mut Vec<&'a str>,
) -> bool {
    if start == dest {
        path.push(start);
        return true;
    }

    if let Some(children) = map.get(start) {
        for child in children {
            if get_path(map, child, dest, path) {
                path.push(start);
                return true;
            }
        }
    }

    false
}
