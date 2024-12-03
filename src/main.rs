use std::{error::Error, fs, path::Path, time::Instant};

mod day1;
mod day2;
mod day3;

fn main() -> Result<(), Box<dyn Error>> {
    let day: i32 = std::env::args()
        .nth(1)
        .expect("Must supply a day")
        .parse()
        .unwrap_or(-1);
    if day == -1 {
        let now = Instant::now();
        for i in 1..=12 {
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
