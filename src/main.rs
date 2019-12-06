use std::*;

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;

type DayFunction = fn() -> Result<(), Box<dyn error::Error>>;

fn main() -> Result<(), Box<dyn error::Error>> {
    let days: Vec<DayFunction> = vec![
        day_01::run,
        day_02::run,
        day_03::run,
        day_04::run,
        day_05::run,
        day_06::run,
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
