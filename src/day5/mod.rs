use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

pub fn run(input: String) -> (String, String) {
    let mut precedes: HashMap<&str, HashSet<&str>> = HashMap::new();
    let mut mids = 0;
    let mut p2mids = 0;
    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }
        if let Some(x) = line.split_once('|') {
            precedes.entry(x.0).or_default().insert(x.1);
            continue;
        }
        let numbers: Vec<&str> = line.split(',').collect();
        let mut sorted_numbers = numbers.clone();
        sorted_numbers.sort_by(|a, b| precedence_order(a, b, &precedes));
        let middle = sorted_numbers[numbers.len() / 2].parse::<u32>().unwrap();
        if numbers == sorted_numbers {
            mids += middle;
        } else {
            p2mids += middle;
        }
    }
    (format!("{mids}"), format!("{p2mids}"))
}

fn precedence_order(a: &str, b: &str, precedes: &HashMap<&str, HashSet<&str>>) -> Ordering {
    if precedes
        .get(a)
        .or(Some(&HashSet::new()))
        .unwrap()
        .contains(b)
    {
        Ordering::Less
    } else if precedes
        .get(b)
        .or(Some(&HashSet::new()))
        .unwrap()
        .contains(a)
    {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
        "};

    #[test]
    fn works() {
        assert_eq!(run(INPUT.to_owned()), ("143".to_owned(), "123".to_owned()));
    }
}
