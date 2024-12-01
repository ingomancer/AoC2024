use std::{collections::HashMap, iter::zip};

use sorted_vec::SortedVec;

pub fn run(input: String) -> (String, String) {
    let mut lefts: SortedVec<i32> = SortedVec::new();
    let mut rights: SortedVec<i32> = SortedVec::new();
    let mut rightcounts: HashMap<i32, u32> = HashMap::new();
    let mut p1sum = 0;
    let mut p2sum = 0;

    for (left, right) in input.lines().map(|l| l.split_once("   ").unwrap()) {
        lefts.insert(left.parse().unwrap());
        let right = right.parse().unwrap();
        rights.insert(right);
        *rightcounts.entry(right).or_default() += 1;
    }

    for (left, right) in zip(lefts.iter(), rights.iter()) {
        p1sum += (left - right).abs();
        p2sum += *left as u32 * *rightcounts.entry(*left).or_default();
    }

    (format!("{p1sum}"), format!("{p2sum}"))
}
