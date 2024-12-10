use std::collections::HashSet;

pub fn run(input: String) -> (String, String) {
    let dimension = input.lines().next().unwrap().len();
    let mut map = vec![11; dimension * dimension];
    let mut trailheads = vec![];
    let mut scores = 0;
    let mut p2scores = 0;
    for (x, line) in input.trim().lines().enumerate() {
        for (y, height) in line
            .chars()
            .map(|char| char.to_digit(10).unwrap() as usize)
            .enumerate()
        {
            map[x + y * dimension] = height;
            if height == 0 {
                trailheads.push((x, y));
            }
        }
    }

    for trailhead in trailheads {
        let mut visited = HashSet::new();
        p2scores += find_peaks(
            &map,
            trailhead,
            dimension,
            map[trailhead.0 + trailhead.1 * dimension],
            &mut visited,
            false,
        );
        scores += find_peaks(
            &map,
            trailhead,
            dimension,
            map[trailhead.0 + trailhead.1 * dimension],
            &mut visited,
            true,
        );
    }
    (format!("{scores}"), format!("{p2scores}"))
}

fn find_peaks(
    map: &[usize],
    pos: (usize, usize),
    dim: usize,
    cur_height: usize,
    visited: &mut HashSet<(usize, usize)>,
    ignore_repeats: bool,
) -> usize {
    let mut score = 0;
    if ignore_repeats {
        visited.insert(pos);
    }
    if cur_height == 9 {
        return 1;
    } else {
        for neighbour in get_neighbours(pos, dim) {
            if visited.contains(&neighbour) {
                continue;
            }
            if map[neighbour.0 + neighbour.1 * dim] == cur_height + 1 {
                score += find_peaks(map, neighbour, dim, cur_height + 1, visited, ignore_repeats);
            }
        }
    }
    score
}

fn get_neighbours(pos: (usize, usize), dim: usize) -> Vec<(usize, usize)> {
    let mut neighbours = vec![];
    for possible_neighbour in [
        (pos.0.wrapping_sub(1), pos.1),
        (pos.0 + 1, pos.1),
        (pos.0, pos.1.wrapping_sub(1)),
        (pos.0, pos.1 + 1),
    ] {
        if possible_neighbour.0 < dim && possible_neighbour.1 < dim {
            neighbours.push(possible_neighbour);
        }
    }
    neighbours
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
        "};

    #[test]
    fn works() {
        assert_eq!(run(INPUT.to_owned()), ("36".to_owned(), "81".to_owned()));
    }
}
