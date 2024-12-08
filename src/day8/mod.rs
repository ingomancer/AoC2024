use std::collections::{HashMap, HashSet};

pub fn run(input: String) -> (String, String) {
    let mut antennae: HashMap<char, HashSet<(usize, usize)>> = HashMap::new();
    let dims = (
        input.trim().lines().count() - 1,
        input.lines().next().unwrap().len() - 1,
    );
    let mut antinodes = HashSet::new();
    let mut p2antinodes = HashSet::new();
    for (x, line) in input.trim().lines().enumerate() {
        for (y, char) in line.chars().enumerate() {
            if char != '.' {
                if let Some(prev_antennaes) = antennae.get_mut(&char) {
                    p2antinodes.insert((x, y));
                    for antenna in prev_antennaes.iter() {
                        p2antinodes.insert(*antenna);
                        let distance = point_difference((x, y), *antenna);
                        let mut node1 = (x as i32 + distance.0, y as i32 + distance.1);
                        let mut node2 =
                            (antenna.0 as i32 - distance.0, antenna.1 as i32 - distance.1);
                        let mut first = true;
                        while !(node1.0 < 0
                            || node1.0 as usize > dims.0
                            || node1.1 < 0
                            || node1.1 as usize > dims.1)
                        {
                            if first {
                                antinodes.insert(node1);
                                first = false;
                            }
                            p2antinodes.insert((node1.0 as usize, node1.1 as usize));
                            node1 = (node1.0 + distance.0, node1.1 + distance.1)
                        }
                        first = true;
                        while !(node2.0 < 0
                            || node2.0 as usize > dims.0
                            || node2.1 < 0
                            || node2.1 as usize > dims.1)
                        {
                            if first {
                                antinodes.insert(node2);
                                first = false;
                            }
                            p2antinodes.insert((node2.0 as usize, node2.1 as usize));
                            node2 = (node2.0 - distance.0, node2.1 - distance.1)
                        }
                    }
                    prev_antennaes.insert((x, y));
                } else {
                    let mut set = HashSet::new();
                    set.insert((x, y));
                    antennae.insert(char, set);
                }
            }
        }
    }
    (
        format!("{}", antinodes.len()),
        format!("{}", p2antinodes.len()),
    )
}

fn point_difference(point_a: (usize, usize), point_b: (usize, usize)) -> (i32, i32) {
    (
        (point_a.0 as i32 - point_b.0 as i32),
        (point_a.1 as i32 - point_b.1 as i32),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
        "};

    #[test]
    fn works() {
        assert_eq!(run(INPUT.to_owned()), ("14".to_owned(), "34".to_owned()));
    }
}
