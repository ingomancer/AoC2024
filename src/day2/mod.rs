enum Direction {
    Increasing,
    Decreasing,
    Neither,
}

fn strictly_monotonic_with_max_change(input: &[u32], max_change: u32) -> Result<(), usize> {
    let mut prev = None;
    let mut direction = Direction::Neither;
    for (index, num) in input.iter().enumerate() {
        if let Some(prev) = prev {
            match num.cmp(prev) {
                std::cmp::Ordering::Less => {
                    match direction {
                        Direction::Increasing => return Err(index),
                        Direction::Decreasing => {}
                        Direction::Neither => direction = Direction::Decreasing,
                    }
                    if (prev - num) > max_change {
                        return Err(index);
                    }
                }
                std::cmp::Ordering::Greater => {
                    match direction {
                        Direction::Decreasing => return Err(index),
                        Direction::Increasing => {}
                        Direction::Neither => direction = Direction::Increasing,
                    }
                    if (num - prev) > max_change {
                        return Err(index);
                    }
                }
                std::cmp::Ordering::Equal => return Err(index),
            }
        }
        prev = Some(num);
    }
    Ok(())
}

pub fn run(input: String) -> (String, String) {
    let mut safe_reports = 0;
    let mut safe_reports_2 = 0;
    let max_change = 3;

    for line in input.lines() {
        let nums = line
            .split_ascii_whitespace()
            .map(|x| x.parse().unwrap())
            .collect::<Vec<u32>>();
        match strictly_monotonic_with_max_change(&nums, max_change) {
            Ok(_) => safe_reports += 1,
            Err(index) => {
                let mut vec1 = nums.clone();
                let mut vec2 = nums.clone();
                let mut vec3 = nums.clone();
                vec1.remove(index);
                vec2.remove(index - 1);
                if index >= 2 {
                    vec3.remove(index - 2);
                }

                match strictly_monotonic_with_max_change(&vec1, max_change) {
                    Ok(_) => {
                        safe_reports_2 += 1;
                        continue;
                    }
                    Err(_) => (),
                }
                match strictly_monotonic_with_max_change(&vec2, max_change) {
                    Ok(_) => {
                        safe_reports_2 += 1;
                        continue;
                    }
                    Err(_) => (),
                }
                match strictly_monotonic_with_max_change(&vec3, max_change) {
                    Ok(_) => {
                        safe_reports_2 += 1;
                    }
                    Err(_) => (),
                }
            }
        }
    }

    (
        format!("{safe_reports}"),
        format!("{}", safe_reports + safe_reports_2),
    )
}
