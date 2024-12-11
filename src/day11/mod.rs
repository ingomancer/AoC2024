use std::{collections::HashMap, mem::swap};

pub fn run(input: String) -> (String, String) {
    let mut stones: HashMap<u64, u64> = HashMap::new();
    let mut stones_next: HashMap<u64, u64> = HashMap::new();
    let mut p1 = 0;
    let mut p2 = 0;
    for stone in input
        .trim()
        .split_ascii_whitespace()
        .map(|stone| stone.parse::<u64>().unwrap())
    {
        stones.insert(stone, 1);
    }
    for i in 0..75 {
        for (stone, amount) in stones.iter() {
            if *stone == 0 {
                *stones_next.entry(1).or_default() += amount;
            } else {
                let stone_str = stone.to_string();
                if stone_str.len() % 2 == 0 {
                    let split = stone_str.split_at(stone_str.len() / 2);
                    *stones_next.entry(split.0.parse().unwrap()).or_default() += amount;
                    *stones_next.entry(split.1.parse().unwrap()).or_default() += amount;
                } else {
                    *stones_next.entry(stone * 2024).or_default() += amount;
                }
            }
        }
        swap(&mut stones, &mut stones_next);
        stones_next.clear();
        if i == 24 {
            for (_, amount) in stones.iter() {
                p1 += amount;
            }
        }
    }

    for (_, amount) in stones.iter() {
        p2 += amount;
    }
    (format!("{p1}"), format!("{p2}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
125 17
        "};

    #[test]
    fn works() {
        assert_eq!(
            run(INPUT.to_owned()),
            ("55312".to_owned(), "65601038650482".to_owned())
        );
    }
}
