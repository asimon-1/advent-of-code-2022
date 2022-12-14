use itertools::Itertools; // Needed for .chunks() method

const LETTERS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn get_priority(c: char) -> u32 {
    if let Some(ind) = LETTERS.find(c) {
        (ind + 1) as u32
    } else {
        0
    }
}

fn find_common_char(v: Vec<&str>) -> char {
    let mut letters: Vec<char> = LETTERS.chars().collect();
    for elem in v.into_iter() {
        letters.retain(|x| elem.contains(*x));
    }
    letters[0]
}

fn part_a(input: &str) -> u32 {
    input
        .lines()
        .map(|x| {
            let (first, second) = x.split_at(x.len() / 2);
            vec![first, second]
        })
        .map(|x| find_common_char(x))
        .map(|x| get_priority(x))
        .sum()
}

fn part_b(input: &str) -> u32 {
    input
        .lines()
        .chunks(3)
        .into_iter()
        .map(|x| x.into_iter().collect_vec())
        .map(|x| find_common_char(x))
        .map(|x| get_priority(x))
        .sum()
}

pub use crate::boilerplate;
boilerplate!(3, 157, 70, u32);