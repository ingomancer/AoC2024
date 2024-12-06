use std::{char, collections::HashMap};

use rayon::iter::{ParallelBridge, ParallelIterator};

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
enum GuardDirection {
    Up,
    Down,
    Left,
    Right,
}

pub fn run(input: String) -> (String, String) {
    let dimension = input.lines().next().unwrap().len();
    let mut grid: Vec<char> = vec!['.'; dimension * dimension];
    let mut guard = ((0, 0), GuardDirection::Up);
    for (x, line) in input.lines().enumerate() {
        for (y, char) in line.trim().chars().enumerate() {
            if char == '#' {
                grid[x + y * dimension] = char;
            } else if char == '^' {
                guard.0 = (x, y);
            }
        }
    }

    let visited = steps_to_exit(&grid, dimension, guard).unwrap();

    let p2pos: u32 = visited
        .iter()
        .par_bridge()
        .map(|position| {
            let mut new_grid = grid.clone();
            new_grid[position.0 .0 + position.0 .1 * dimension] = '#';
            let start = match position.1[0] {
                GuardDirection::Up => (position.0 .0 + 1, position.0 .1),
                GuardDirection::Down => (position.0 .0 - 1, position.0 .1),
                GuardDirection::Left => (position.0 .0, position.0 .1 + 1),
                GuardDirection::Right => (position.0 .0, position.0 .1 - 1),
            };
            if steps_to_exit(&new_grid, dimension, (start, position.1[0])).is_none() {
                1
            } else {
                0
            }
        })
        .sum();

    (format!("{}", visited.len()), format!("{p2pos}"))
}

fn steps_to_exit(
    grid: &[char],
    dimension: usize,
    initial_position: ((usize, usize), GuardDirection),
) -> Option<HashMap<(usize, usize), Vec<GuardDirection>>> {
    let mut guard = initial_position;
    let mut visited: HashMap<(usize, usize), Vec<GuardDirection>> = HashMap::new();
    loop {
        if let Some(pos) = visited.get(&guard.0) {
            if pos.contains(&guard.1) {
                return None;
            }
        }
        if guard.0 .0 == dimension || guard.0 .1 == dimension {
            break;
        }
        visited.entry(guard.0).or_default().push(guard.1);
        match guard.1 {
            GuardDirection::Up => {
                if guard.0 .0 == 0 {
                    break;
                }
                let (x, y) = (guard.0 .0 - 1, guard.0 .1);
                if let Some('#') = grid.get(x + y * dimension) {
                    guard.1 = GuardDirection::Right;
                } else {
                    guard.0 = (x, y);
                };
            }
            GuardDirection::Down => {
                let (x, y) = (guard.0 .0 + 1, guard.0 .1);
                if let Some('#') = grid.get(x + y * dimension) {
                    guard.1 = GuardDirection::Left;
                } else {
                    guard.0 = (x, y);
                }
            }
            GuardDirection::Left => {
                if guard.0 .1 == 0 {
                    break;
                }
                let (x, y) = (guard.0 .0, guard.0 .1 - 1);
                if let Some('#') = grid.get(x + y * dimension) {
                    guard.1 = GuardDirection::Up;
                } else {
                    guard.0 = (x, y);
                };
            }
            GuardDirection::Right => {
                let (x, y) = (guard.0 .0, guard.0 .1 + 1);
                if let Some('#') = grid.get(x + y * dimension) {
                    guard.1 = GuardDirection::Down;
                } else {
                    guard.0 = (x, y);
                }
            }
        }
    }
    Some(visited)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
        "};

    #[test]
    fn works() {
        assert_eq!(run(INPUT.to_owned()), ("41".to_owned(), "6".to_owned()));
    }
}
