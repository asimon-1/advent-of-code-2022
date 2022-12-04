use std::ops::Range;

fn to_range(s: &str) -> Range<u32> {
    let (lower, upper) = s.split_once("-").unwrap();
    Range {
        start: lower.parse::<u32>().unwrap(),
        end: upper.parse::<u32>().unwrap() + 1,
    }
}

fn is_contained(a: &Range<u32>, b: &Range<u32>) -> bool {
    // Checks both (a in b) and (b in a)
    a.contains(&b.start) && a.contains(&(b.end - 1))
        || b.contains(&a.start) && b.contains(&(a.end - 1))
}

fn is_overlapped(a: &Range<u32>, b: &Range<u32>) -> bool {
    a.contains(&b.start)
        || a.contains(&(b.end - 1))
        || b.contains(&a.start)
        || b.contains(&(a.end - 1))
}

fn part_a(input: &str) -> u32 {
    input
        .lines()
        .map(|x| x.split_once(",").unwrap())
        .map(|x| (to_range(x.0), to_range(x.1)))
        .filter(|x| is_contained(&x.0, &x.1))
        .count() as u32
}

fn part_b(input: &str) -> u32 {
    input
        .lines()
        .map(|x| x.split_once(",").unwrap())
        .map(|x| (to_range(x.0), to_range(x.1)))
        .filter(|x| is_overlapped(&x.0, &x.1))
        .count() as u32
}

pub fn run_part_a() -> u32 {
    let input = include_str!("../input/4.txt");
    part_a(input)
}

pub fn run_part_b() -> u32 {
    let input = include_str!("../input/4.txt");
    part_b(input)
}

#[test]
fn part_a_test() {
    let input = include_str!("../input_test/4.txt");
    assert_eq!(part_a(input), 2);
}

#[test]
fn part_b_test() {
    let input = include_str!("../input_test/4.txt");
    assert_eq!(part_b(input), 4);
}
