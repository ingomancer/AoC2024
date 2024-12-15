use std::{cmp::max, collections::HashMap};

use aoc2024::add_tuple;

pub fn run(input: String) -> (String, String) {
    let (initial_map, moves) = input.trim().split_once("\n\n").unwrap();
    let mut map: HashMap<(isize, isize), char> = HashMap::new();
    let mut robot_pos = (0, 0);
    let mut dims = (0, 0);
    for (x, line) in (-0..).zip(initial_map.lines()) {
        dims.0 = max(dims.0, x);
        for (y, pos) in (-0..).zip(line.chars()) {
            dims.1 = max(dims.1, y);
            map.insert((x, y), pos);
            if pos == '@' {
                robot_pos = (x, y);
            }
        }
    }
    #[cfg(debug_assertions)]
    print_map(&map, dims);

    for instruction in moves.chars() {
        let next = match instruction {
            '^' => attempt_move(robot_pos, &mut map, (-1, 0)),
            'v' => attempt_move(robot_pos, &mut map, (1, 0)),
            '<' => attempt_move(robot_pos, &mut map, (0, -1)),
            '>' => attempt_move(robot_pos, &mut map, (0, 1)),
            _ => continue,
        };
        if let Some(next) = next {
            robot_pos = next;
        }
    }
    #[cfg(debug_assertions)]
    print_map(&map, dims);

    let mut gpssum = 0;
    for pos_box in map {
        if pos_box.1 == 'O' {
            gpssum += pos_box.0 .0 * 100 + pos_box.0 .1;
        }
    }
    (format!("{gpssum}"), format!(""))
}
#[cfg(debug_assertions)]
pub fn print_map(map: &HashMap<(isize, isize), char>, dims: (isize, isize)) {
    for x in 0..dims.0 + 1 {
        for y in 0..dims.1 + 1 {
            print!("{}", map.get(&(x, y)).unwrap());
        }
        println!();
    }
}
fn attempt_move(
    pos: (isize, isize),
    map: &mut HashMap<(isize, isize), char>,
    dir: (isize, isize),
) -> Option<(isize, isize)> {
    let next_pos = add_tuple(pos, dir);
    let next = map.get_mut(&next_pos).unwrap();

    if next == &'#' {
        None
    } else if next == &'.' || attempt_move(next_pos, map, dir).is_some() {
        let swap = map.insert(next_pos, *map.get(&pos).unwrap()).unwrap();
        map.insert(pos, swap);
        Some(next_pos)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn works_small() {
        const INPUT: &str = indoc! {"
    ########
    #..O.O.#
    ##@.O..#
    #...O..#
    #.#.O..#
    #...O..#
    #......#
    ########
    
    <^^>>>vv<v>>v<<
            "};
        assert_eq!(run(INPUT.to_owned()), ("2028".to_owned(), "".to_owned()));
    }

    #[test]
    fn works_large() {
        const INPUT: &str = indoc! {"
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
            "};
        assert_eq!(run(INPUT.to_owned()), ("10092".to_owned(), "".to_owned()));
    }
}
