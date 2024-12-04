use std::collections::HashMap;

pub fn run(input: String) -> (String, String) {
    let mut grid: HashMap<(usize, usize), char> = HashMap::new();
    let mut xes = vec![];
    let mut a_cross = vec![];
    let mut xmas = 0;
    let mut x_mas = 0;

    for (x, line) in input.lines().enumerate() {
        for (y, letter) in line.chars().enumerate() {
            if letter == 'X' {
                xes.push((x, y));
            }
            if letter == 'A' {
                a_cross.push((x, y));
            }
            grid.insert((x, y), letter);
        }
    }

    for x in xes {
        xmas += check_x(&x, &grid);
    }

    for a in a_cross {
        x_mas += check_a(&a, &grid);
    }

    (format!("{xmas}"), format!("{x_mas}"))
}

fn check_a(a: &(usize, usize), grid: &HashMap<(usize, usize), char>) -> u32 {
    let mut hits = 0;

    // I am sorry for my crimes, do not look
    let diag_one = ((-1, -1), (1, 1));
    let diag_two = ((-1, 1), (1, -1));
    let diag_one = (
        (
            (a.0 as i32 + diag_one.0 .0) as usize,
            (a.1 as i32 + diag_one.0 .1) as usize,
        ),
        (
            (a.0 as i32 + diag_one.1 .0) as usize,
            (a.1 as i32 + diag_one.1 .1) as usize,
        ),
    );
    let diag_two = (
        (
            (a.0 as i32 + diag_two.0 .0) as usize,
            (a.1 as i32 + diag_two.0 .1) as usize,
        ),
        (
            (a.0 as i32 + diag_two.1 .0) as usize,
            (a.1 as i32 + diag_two.1 .1) as usize,
        ),
    );
    if (grid.get(&diag_one.0) == Some(&'M') || grid.get(&diag_one.0) == Some(&'S'))
        && (grid.get(&diag_one.1) == Some(&'M') || grid.get(&diag_one.1) == Some(&'S'))
        && grid.get(&diag_one.0) != grid.get(&diag_one.1)
        && (grid.get(&diag_two.0) == Some(&'M') || grid.get(&diag_two.0) == Some(&'S'))
        && (grid.get(&diag_two.1) == Some(&'M') || grid.get(&diag_two.1) == Some(&'S'))
        && grid.get(&diag_two.0) != grid.get(&diag_two.1)
    {
        hits += 1;
    }
    // Crime over, feel free to look again
    hits
}

fn check_x(x: &(usize, usize), grid: &HashMap<(usize, usize), char>) -> u32 {
    let directions: Vec<(i32, i32)> = vec![
        (0, 1),
        (0, -1),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
        (1, 0),
        (-1, 0),
    ];
    let expected_letters = ['M', 'A', 'S'];
    let mut hits = 0;
    for direction in directions {
        let mut letter = *x;
        for (i, item) in expected_letters.iter().enumerate() {
            letter = (
                (letter.0 as i32 + direction.0) as _,
                (letter.1 as i32 + direction.1) as _,
            );
            if grid.get(&letter) != Some(item) {
                break;
            }
            if i == 2 {
                hits += 1;
            }
        }
    }
    hits
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
    "};

    #[test]
    fn works() {
        assert_eq!(run(INPUT.to_owned()), ("18".to_owned(), "9".to_owned()));
    }
}
