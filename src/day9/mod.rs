use std::usize;

pub fn run(input: String) -> (String, String) {
    let mut fs = vec![];
    let mut block_id = 0;
    let mut p1checksum = 0;
    let mut p2checksum = 0;
    for (i, char) in input.trim().chars().enumerate() {
        let block_size = char.to_digit(10).unwrap();
        if i % 2 == 0 {
            for _ in 0..block_size {
                fs.push(Some(block_id));
            }
            block_id += 1;
        } else {
            for _ in 0..block_size {
                fs.push(None)
            }
        }
    }

    let mut compacted_fs = fs.clone();
    let mut empty_block = 0;
    for (j, file) in fs.iter().enumerate().rev() {
        if file.is_some() {
            while compacted_fs[empty_block].is_some() {
                empty_block += 1;
            }
            if empty_block >= j {
                break;
            }
            compacted_fs[empty_block] = *file;
            compacted_fs[j] = None;
        }
    }

    let mut p2compacted_fs = fs.clone();
    let mut prev = usize::MAX;
    let mut prev_amount: usize = 1;
    let mut prev_index = usize::MAX;
    for (i, file) in fs.iter().enumerate().rev() {
        if let Some(file) = file {
            if *file == prev {
                prev_amount += 1;
            } else if prev != usize::MAX {
                let pos = find_empty_block(&p2compacted_fs, prev_amount, prev_index);
                if !(pos == usize::MAX) {
                    for j in pos..(pos + prev_amount) {
                        p2compacted_fs[j] = Some(prev);
                    }
                    for k in (prev_index - prev_amount + 1)..(prev_index + 1) {
                        p2compacted_fs[k] = None;
                    }
                }
                prev = *file;
                prev_index = i;
                prev_amount = 1;
            } else {
                prev = *file;
                prev_index = i;
                prev_amount = 1;
            }
        }
    }

    for (i, entry) in compacted_fs.iter().enumerate() {
        if entry.is_none() {
            break;
        } else {
            p1checksum += i * entry.unwrap();
        }
    }

    for (i, entry) in p2compacted_fs.iter().enumerate() {
        if entry.is_none() {
            continue;
        } else {
            p2checksum += i * entry.unwrap();
        }
    }
    (format!("{p1checksum}"), format!("{p2checksum}"))
}

fn find_empty_block(fs: &[Option<usize>], block_size: usize, max_index: usize) -> usize {
    let mut cur_block_size = 0;
    let mut cur_block_start = usize::MAX;
    for (i, pos) in fs.iter().enumerate() {
        if pos.is_none() {
            if cur_block_start == usize::MAX {
                cur_block_start = i;
            }
            cur_block_size += 1;
            if cur_block_size == block_size {
                return cur_block_start;
            }
        } else {
            cur_block_start = usize::MAX;
            cur_block_size = 0;
        }

        if i >= max_index {
            return usize::MAX;
        }
    }

    cur_block_start
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
2333133121414131402
        "};

    #[test]
    fn works() {
        assert_eq!(
            run(INPUT.to_owned()),
            ("1928".to_owned(), "2858".to_owned())
        );
    }
}
