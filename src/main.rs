use std::*;

mod day_01;
mod day_02;

type DayFunction = fn() -> Result<(), Box<dyn error::Error>>;

fn main() -> Result<(), Box<dyn error::Error>> {
    let days: Vec<DayFunction> = vec![day_01::run, day_02::run];

    let args = env::args().skip(1).collect::<Vec<_>>();
    if args == vec!["all"] {
        for day in days {
            day()?;
        }
    } else {
        let day = days.into_iter().last().unwrap();
        day()?
    }
    Ok(())
}
