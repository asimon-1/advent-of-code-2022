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

pub fn run_part_a() -> u32 {
    let input = include_str!("../input/3.txt");
    part_a(input)
}

pub fn run_part_b() -> u32 {
    let input = include_str!("../input/3.txt");
    part_b(input)
}

#[test]
fn part_a_test() {
    let input = include_str!("../input_test/3.txt");
    assert_eq!(part_a(input), 157);
}

#[test]
fn part_b_test() {
    let input = include_str!("../input_test/3.txt");
    assert_eq!(part_b(input), 70);
}
