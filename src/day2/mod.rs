enum Direction {
    Increasing,
    Decreasing,
    Neither,
}

pub fn run(input: String) -> (String, String) {
    let mut safe_reports = 0;
    'outer: for line in input.lines() {
        let mut prev: Option<u32> = None;
        let mut direction = Direction::Neither;
        for num in line.split_ascii_whitespace().map(|x| x.parse().unwrap()) {
            if let Some(prev) = prev {
                match direction {
                    Direction::Increasing => {
                        if num > prev {
                            match num - prev {
                                1..=3 => (),
                                _ => continue 'outer,
                            }
                        } else {
                            continue 'outer;
                        }
                    }
                    Direction::Decreasing => {
                        if num < prev {
                            match prev - num {
                                1..=3 => (),
                                _ => continue 'outer,
                            }
                        } else {
                            continue 'outer;
                        }
                    }
                    Direction::Neither => {
                        if num > prev {
                            direction = Direction::Increasing;
                            match num - prev {
                                1..=3 => (),
                                _ => continue 'outer,
                            }
                        } else if num < prev {
                            direction = Direction::Decreasing;
                            match prev - num {
                                1..=3 => (),
                                _ => continue 'outer,
                            }
                        } else {
                            continue 'outer;
                        }
                    }
                }
            }
            prev = Some(num);
        }

        safe_reports += 1;
    }

    let mut safe_reports_2 = 0;
    'outer: for line in input.lines() {
        let mut first_miss = true;
        let mut prev: Option<u32> = None;
        let mut direction = Direction::Neither;
        for num in line.split_ascii_whitespace().map(|x| x.parse().unwrap()) {
            let mut miss = false;
            if let Some(prev) = prev {
                match direction {
                    Direction::Increasing => {
                        if num > prev {
                            match num - prev {
                                1..=3 => (),
                                _ => {
                                    if first_miss {
                                        first_miss = false;
                                        miss = true;
                                    } else {
                                        continue 'outer;
                                    };
                                }
                            }
                        } else if first_miss {
                            first_miss = false;
                            miss = true;
                        } else {
                            continue 'outer;
                        }
                    }
                    Direction::Decreasing => {
                        if num < prev {
                            match prev - num {
                                1..=3 => (),
                                _ => {
                                    if first_miss {
                                        first_miss = false;
                                        miss = true;
                                    } else {
                                        continue 'outer;
                                    }
                                }
                            }
                        } else if first_miss {
                            first_miss = false;
                            miss = true;
                        } else {
                            continue 'outer;
                        }
                    }
                    Direction::Neither => {
                        if num > prev {
                            direction = Direction::Increasing;
                            match num - prev {
                                1..=3 => (),
                                _ => {
                                    if first_miss {
                                        first_miss = false;
                                        miss = true;
                                    } else {
                                        if first_miss {
                                            first_miss = false;
                                            miss = true;
                                        } else {
                                            continue 'outer;
                                        };
                                    }
                                }
                            }
                        } else if num < prev {
                            direction = Direction::Decreasing;
                            match prev - num {
                                1..=3 => (),
                                _ => {
                                    if first_miss {
                                        first_miss = false;
                                        miss = true;
                                    } else {
                                        continue 'outer;
                                    }
                                }
                            }
                        } else {
                            if first_miss {
                                first_miss = false;
                                miss = true;
                            } else {
                                continue 'outer;
                            };
                        }
                    }
                }
            }
            if !miss {
                prev = Some(num);
            }
        }

        safe_reports_2 += 1;
    }
    (format!("{safe_reports}"), format!("{safe_reports_2}"))
}
