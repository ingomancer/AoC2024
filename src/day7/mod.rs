use std::iter::repeat_n;

use itertools::Itertools;
use rayon::iter::ParallelBridge;
use rayon::iter::ParallelIterator;

#[derive(Clone, Debug, PartialEq)]
enum Op {
    Add,
    Mul,
    Cat,
}

pub fn run(input: String) -> (String, String) {
    let (p1sum, p2sum) = input
        .trim()
        .lines()
        .par_bridge()
        .map(|line| {
            let (sum, nums) = line.split_once(": ").expect("All lines start with {num}:");
            let sum: u64 = sum.parse().expect("The initial number is parseable");
            let nums: Vec<u64> = nums
                .split_ascii_whitespace()
                .map(|x| {
                    x.parse::<u64>().unwrap_or_else(|_| {
                        panic!("Every line has only numbers separated by whitespace. {x}, {line}")
                    })
                })
                .collect();
            let possible_ops = vec![Op::Add, Op::Mul, Op::Cat];
            let ops = repeat_n(possible_ops, nums.len() - 1).multi_cartesian_product();
            for oplist in ops {
                let mut cumsum: u64 = 0;
                let mut prev = None;
                for (i, num) in nums.iter().enumerate() {
                    if let Some(x) = prev {
                        let val: u64 = match oplist[i - 1] {
                            Op::Add => x + num,
                            Op::Mul => x * num,
                            Op::Cat => format!("{x}{num}").parse().unwrap(),
                        };
                        cumsum = val;
                        prev = Some(val);
                    } else {
                        prev = Some(*num);
                    }
                }

                if cumsum == sum {
                    if oplist.contains(&Op::Cat) {
                        return (0, sum);
                    } else {
                        return (sum, 0);
                    }
                }
            }
            (0, 0)
        })
        .reduce(|| (0, 0), |acc, val| (acc.0 + val.0, acc.1 + val.1));
    (format!("{p1sum}"), format!("{}", p1sum + p2sum))
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
        "};

    #[test]
    fn works() {
        assert_eq!(
            run(INPUT.to_owned()),
            ("3749".to_owned(), "11387".to_owned())
        );
    }
}
