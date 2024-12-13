use itertools::Itertools;

pub fn run(input: String) -> (String, String) {
    let mut tokens = 0;
    let mut p2tokens = 0;
    let p2pos = 10000000000000;
    for machine in input.trim().split("\n\n") {
        let mut parts = machine.splitn(3, '\n');
        let a = parts.next().unwrap();
        let b = parts.next().unwrap();
        let prize = parts.next().unwrap();

        let a = parse_row(a, '+');
        let b = parse_row(b, '+');
        let prize = parse_row(prize, '=');
        let cheapest_win = find_cheapest_win(a, b, prize);
        tokens += cheapest_win;
        let cheapest_win = find_cheapest_win(a, b, (prize.0 + p2pos, prize.1 + p2pos));
        p2tokens += cheapest_win;
    }
    (format!("{tokens}"), format!("{p2tokens}"))
}

fn find_cheapest_win(a: (isize, isize), b: (isize, isize), prize: (isize, isize)) -> isize {
    let det = a.0 * b.1 - a.1 * b.0;
    if det == 0 {
        return 0;
    }

    let a_presses = prize.0 * b.1 - prize.1 * b.0;
    let b_presses = prize.1 * a.0 - prize.0 * a.1;

    if a_presses % det != 0 || b_presses % det != 0 {
        return 0;
    }
    3 * (a_presses / det) + b_presses / det
}

fn parse_row(line: &str, sep: char) -> (isize, isize) {
    line.split(':')
        .last()
        .unwrap()
        .split(',')
        .map(|x| x.split(sep).last().unwrap().parse::<isize>().unwrap())
        .take(2)
        .collect_tuple()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
        "};

    #[test]
    fn works() {
        assert_eq!(
            run(INPUT.to_owned()),
            ("480".to_owned(), "875318608908".to_owned())
        );
    }
}
