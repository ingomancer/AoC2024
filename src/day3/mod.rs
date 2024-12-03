use regex::Regex;

pub fn run(input: String) -> (String, String) {
    let mut sum1 = 0;
    let mut sum2 = 0;
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)|do\(\)|don't\(\)").unwrap();
    let mut counts = true;
    for mul in re.captures_iter(&input) {
        match mul.get(1) {
            Some(_) => {
                let x: u32 = mul.get(1).unwrap().as_str().parse().unwrap();
                let y: u32 = mul.get(2).unwrap().as_str().parse().unwrap();
                sum1 += x * y;
                if counts {
                    sum2 += x * y;
                }
            }
            None => match mul.get(0).unwrap().as_str() {
                "do()" => counts = true,
                "don't()" => counts = false,
                _ => (),
            },
        }
    }
    (format!("{sum1}"), format!("{sum2}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
           xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
        "};

    #[test]
    fn works() {
        assert_eq!(run(INPUT.to_owned()), ("161".to_owned(), "48".to_owned()));
    }
}
