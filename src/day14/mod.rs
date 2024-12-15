use std::collections::HashMap;

use itertools::max;

pub fn run(input: String) -> (String, String) {
    let mut robots: HashMap<(isize, isize), Vec<(isize, isize)>> = HashMap::new();
    let mut dims: (isize, isize) = (0, 0);
    for line in input.trim().lines() {
        let mut split = line.split_ascii_whitespace();
        let start = split.next().unwrap();
        let velocity = split.next().unwrap();
        let mut start = start[2..].split(',');
        let start: (isize, isize) = (
            start.next().unwrap().parse().unwrap(),
            start.next().unwrap().parse().unwrap(),
        );
        dims.0 = max([start.0, dims.0]).unwrap();
        dims.1 = max([start.1, dims.1]).unwrap();
        let mut velocity = velocity[2..].split_terminator(',');
        let velocity: (isize, isize) = (
            velocity.next().unwrap().parse().unwrap(),
            velocity.next().unwrap().parse().unwrap(),
        );
        robots.entry(start).or_default().push(velocity);
    }

    let mut i = 0;
    let mut moved_robots;
    loop {
        moved_robots = move_robots(&robots, dims, i);

        let mut stacked_robots = 0;

        for robot in moved_robots.into_values() {
            stacked_robots = max([stacked_robots, robot]).unwrap();
        }
        if stacked_robots == 1 {
            break;
        }
        i += 1;
    }

    moved_robots = move_robots(&robots, dims, 100);
    let quadrants = count_robots(&moved_robots, dims);

    (
        format!("{}", quadrants.0 * quadrants.1 * quadrants.2 * quadrants.3),
        format!("{i}"),
    )
}

fn count_robots(
    robots: &HashMap<(isize, isize), isize>,
    dims: (isize, isize),
) -> (isize, isize, isize, isize) {
    let mut quadrants = (0, 0, 0, 0);

    for pos in robots {
        let count = pos.1;
        let pos = pos.0;
        if pos.0 < dims.0 / 2 && pos.1 < dims.1 / 2 {
            quadrants.0 += count;
        } else if pos.0 > dims.0 / 2 && pos.1 < dims.1 / 2 {
            quadrants.1 += count;
        } else if pos.0 < dims.0 / 2 && pos.1 > dims.1 / 2 {
            quadrants.2 += count;
        } else if pos.0 > dims.0 / 2 && pos.1 > dims.1 / 2 {
            quadrants.3 += count;
        }
    }

    quadrants
}

fn move_robots(
    robot_positions: &HashMap<(isize, isize), Vec<(isize, isize)>>,
    dims: (isize, isize),
    steps: isize,
) -> HashMap<(isize, isize), isize> {
    let mut output: HashMap<(isize, isize), isize> = HashMap::new();
    for robots in robot_positions {
        let pos = robots.0;
        for robot in robots.1 {
            *output
                .entry((
                    (pos.0 + robot.0 * steps).rem_euclid(dims.0 + 1),
                    (pos.1 + robot.1 * steps).rem_euclid(dims.1 + 1),
                ))
                .or_default() += 1;
        }
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
        "};

    #[test]
    fn works() {
        assert_eq!(run(INPUT.to_owned()), ("12".to_owned(), "1".to_owned()));
    }
}
