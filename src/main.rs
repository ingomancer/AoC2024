use std::{error::Error, fs, path::Path, time::Instant};

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn main() -> Result<(), Box<dyn Error>> {
    let day: i32 = std::env::args()
        .nth(1)
        .expect("Must supply a day")
        .parse()
        .unwrap_or(-1);
    if day == -1 {
        let now = Instant::now();
        for i in 1..=14 {
            run_day(i, false)?;
        }
        println!("{}", now.elapsed().as_secs_f32());
    } else {
        run_day(day, true)?;
    }
    Ok(())
}

fn unknown_day(_input: String) -> (String, String) {
    ("Unknown".to_owned(), "Day".to_owned())
}

fn run_day(day: i32, print: bool) -> Result<(), Box<dyn Error>> {
    let path = format!("src/day{}/input", day);
    let input = fs::read_to_string(Path::new(&path))?;

    let day_func = match day {
        1 => day1::run,
        2 => day2::run,
        3 => day3::run,
        4 => day4::run,
        5 => day5::run,
        6 => day6::run,
        7 => day7::run,
        8 => day8::run,
        9 => day9::run,
        10 => day10::run,
        11 => day11::run,
        12 => day12::run,
        13 => day13::run,
        14 => day14::run,
        _ => unknown_day,
    };
    let now = Instant::now();
    let (part1, part2) = day_func(input);
    println!("{}", now.elapsed().as_secs_f32());
    if print {
        println!("{}, {}", part1, part2);
    }
    Ok(())
}
