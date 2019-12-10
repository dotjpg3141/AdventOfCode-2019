use std::*;

#[macro_use]
extern crate itertools;

mod intcode_computer;

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;

type DayFunction = fn() -> Result<(), Box<dyn error::Error>>;

fn main() -> Result<(), Box<dyn error::Error>> {
    let days: Vec<DayFunction> = vec![
        day_01::run,
        day_02::run,
        day_03::run,
        day_04::run,
        day_05::run,
        day_06::run,
        day_07::run,
        day_08::run,
        day_09::run,
        day_10::run,
    ];

    let now = time::Instant::now();

    let args = env::args().skip(1).collect::<Vec<_>>();
    if args == vec!["all"] {
        for day in days {
            day()?;
        }
    } else {
        let day = days.into_iter().last().unwrap();
        day()?
    }

    println!("Time: {} s", now.elapsed().as_secs_f32());

    Ok(())
}
