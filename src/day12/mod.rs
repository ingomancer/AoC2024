use std::collections::HashSet;

pub fn run(input: String) -> (String, String) {
    let dimension = input.lines().next().unwrap().len();
    let mut map = vec!['.'; dimension * dimension];
    for (x, line) in input.trim().lines().enumerate() {
        for (y, char) in line.chars().enumerate() {
            map[x + y * dimension] = char;
        }
    }

    let mut visited = HashSet::new();
    let mut fencecosts = 0;
    let mut bulk_costs = 0;
    for x in 0..dimension {
        for y in 0..dimension {
            let prev_visited = visited.clone();
            let (area, perimiter) = count_area(
                (x, y),
                &map,
                dimension,
                &mut visited,
                map[x + y * dimension],
            );
            fencecosts += area * perimiter;
            let region: HashSet<_> = visited.difference(&prev_visited).collect();
            let sides = count_region_sides(&region);
            bulk_costs += area * sides;
        }
    }
    (format!("{fencecosts}"), format!("{bulk_costs}"))
}

fn count_region_sides(region: &HashSet<&(usize, usize)>) -> usize {
    let mut side_count = 0;
    for dir in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
        let mut sides = HashSet::new();
        for pos in region {
            let neighbour = (
                pos.0.wrapping_add_signed(dir.0),
                pos.1.wrapping_add_signed(dir.1),
            );
            if !region.contains(&neighbour) {
                sides.insert(neighbour);
            }
        }
        let mut remove = HashSet::new();
        for side in &sides {
            let mut neighbour = (
                side.0.wrapping_add_signed(dir.1),
                side.1.wrapping_add_signed(dir.0),
            );
            while sides.contains(&neighbour) {
                remove.insert(neighbour);
                neighbour = (
                    neighbour.0.wrapping_add_signed(dir.1),
                    neighbour.1.wrapping_add_signed(dir.0),
                );
            }
        }
        side_count += sides.len() - remove.len();
    }

    side_count
}

fn count_area(
    pos: (usize, usize),
    map: &Vec<char>,
    dim: usize,
    visited: &mut HashSet<(usize, usize)>,
    plot_type: char,
) -> (usize, usize) {
    let mut area = 1;
    let mut perimeter = 4;
    if visited.contains(&pos) {
        return (0, 0);
    }
    visited.insert(pos);
    let neighbours = get_neighbours(pos, dim);
    for neighbour in neighbours {
        if map[neighbour.0 + neighbour.1 * dim] == plot_type {
            perimeter -= 1;
            let (n_area, n_perim) = count_area(neighbour, map, dim, visited, plot_type);
            area += n_area;
            perimeter += n_perim;
        }
    }

    (area, perimeter)
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
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
        "};

    #[test]
    fn works() {
        assert_eq!(
            run(INPUT.to_owned()),
            ("1930".to_owned(), "1206".to_owned())
        );
    }
}
