use std::{
    char,
    collections::{HashMap, HashSet},
};

use rayon::iter::{ParallelBridge, ParallelIterator};

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
enum GuardDirection {
    Up,
    Down,
    Left,
    Right,
}

pub fn run(input: String) -> (String, String) {
    let mut grid: HashMap<(usize, usize), char> = HashMap::new();
    let mut guard = ((0, 0), GuardDirection::Up);
    for (x, line) in input.lines().enumerate() {
        for (y, char) in line.trim().chars().enumerate() {
            grid.insert((x, y), char);
            if char == '^' {
                guard = ((x, y), GuardDirection::Up);
            }
        }
    }

    let visited = steps_to_exit(&grid, guard).unwrap();

    let p2pos: u32 = visited
        .keys()
        .par_bridge()
        .map(|position| {
            let mut new_grid = grid.clone();
            *new_grid.get_mut(position).unwrap() = '#';
            if steps_to_exit(&new_grid, guard).is_none() {
                1
            } else {
                0
            }
        })
        .sum();

    (format!("{}", visited.len()), format!("{p2pos}"))
}

fn steps_to_exit(
    grid: &HashMap<(usize, usize), char>,
    initial_position: ((usize, usize), GuardDirection),
) -> Option<HashMap<(usize, usize), HashSet<GuardDirection>>> {
    let mut guard = initial_position;
    let mut visited: HashMap<(usize, usize), HashSet<GuardDirection>> = HashMap::new();
    while grid.contains_key(&guard.0) {
        if let Some(pos) = visited.get(&guard.0) {
            if pos.contains(&guard.1) {
                return None;
            }
        }
        visited.entry(guard.0).or_default().insert(guard.1);
        match guard.1 {
            GuardDirection::Up => {
                let next = (guard.0 .0.wrapping_sub(1), guard.0 .1);
                if let Some('#') = grid.get(&next) {
                    guard.1 = GuardDirection::Right;
                } else {
                    guard.0 = next;
                }
            }
            GuardDirection::Down => {
                let next = (guard.0 .0 + 1, guard.0 .1);
                if let Some('#') = grid.get(&next) {
                    guard.1 = GuardDirection::Left;
                } else {
                    guard.0 = next;
                }
            }
            GuardDirection::Left => {
                let next = (guard.0 .0, guard.0 .1.wrapping_sub(1));
                if let Some('#') = grid.get(&next) {
                    guard.1 = GuardDirection::Up;
                } else {
                    guard.0 = next;
                }
            }
            GuardDirection::Right => {
                let next = (guard.0 .0, guard.0 .1 + 1);
                if let Some('#') = grid.get(&next) {
                    guard.1 = GuardDirection::Down;
                } else {
                    guard.0 = next;
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
