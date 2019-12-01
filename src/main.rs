use std::*;

mod day_01;

fn main() -> Result<(), Box<dyn error::Error>> {
    let days = vec![day_01::run];
    days.last().unwrap()()
}
