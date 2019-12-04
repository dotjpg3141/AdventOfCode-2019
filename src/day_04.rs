use std::io::Read;
use std::*;

pub(crate) fn run() -> Result<(), Box<dyn error::Error>> {
    let mut file = fs::File::open("input/day4.txt")?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;

    let mut input = input.split('-').map(|s| s.parse::<usize>());
    let start = input.next().unwrap()?;
    let end = input.next().unwrap()?;

    let count = count_passwords_candidates(start, end, |d1, d2, d3, d4, d5, d6| {
        d1 == d2 || d2 == d3 || d3 == d4 || d4 == d5 || d5 == d6
    });
    println!("Day 4a: {}", count);

    let count = count_passwords_candidates(start, end, |d1, d2, d3, d4, d5, d6| {
        (d1 == d2 && d2 != d3)
            || (d1 != d2 && d2 == d3 && d3 != d4)
            || (d2 != d3 && d3 == d4 && d4 != d5)
            || (d3 != d4 && d4 == d5 && d5 != d6)
            || (d4 != d5 && d5 == d6)
    });
    println!("Day 4b: {}", count);

    Ok(())
}

fn count_passwords_candidates<P>(start: usize, end: usize, predicate: P) -> usize
where
    P: Fn(usize, usize, usize, usize, usize, usize) -> bool,
{
    let mut count = 0;

    for d1 in 1..=9 {
        for d2 in d1..=9 {
            for d3 in d2..=9 {
                for d4 in d3..=9 {
                    for d5 in d4..=9 {
                        for d6 in d5..=9 {
                            if !predicate(d1, d2, d3, d4, d5, d6) {
                                continue;
                            }

                            let num = d1 * 1e5 as usize
                                + d2 * 1e4 as usize
                                + d3 * 1e3 as usize
                                + d4 * 1e2 as usize
                                + d5 * 1e1 as usize
                                + d6 * 1e0 as usize;

                            if num < start || num > end {
                                continue;
                            }

                            count += 1;
                        }
                    }
                }
            }
        }
    }
    count
}
