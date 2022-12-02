enum Round {
    AX,
    AY,
    AZ,
    BX,
    BY,
    BZ,
    CX,
    CY,
    CZ,
}

impl Round {
    fn from_str(s: &str) -> Round {
        match s {
            "A X" => Round::AX,
            "A Y" => Round::AY,
            "A Z" => Round::AZ,
            "B X" => Round::BX,
            "B Y" => Round::BY,
            "B Z" => Round::BZ,
            "C X" => Round::CX,
            "C Y" => Round::CY,
            "C Z" => Round::CZ,
            _ => panic!("Unrecognized str for Round: {}", s)
        }
    }

    fn score_part_a(self) -> u32 {
        match self {
            Round::AX => { 1 + 3 },
            Round::AY => { 2 + 6 },
            Round::AZ => { 3 + 0 },
            Round::BX => { 1 + 0 },
            Round::BY => { 2 + 3 },
            Round::BZ => { 3 + 6 },
            Round::CX => { 1 + 6 },
            Round::CY => { 2 + 0 },
            Round::CZ => { 3 + 3 },
        }
    }

    fn score_part_b(self) -> u32 {
        match self {
            Round::AX => { 3 + 0 },
            Round::AY => { 1 + 3 },
            Round::AZ => { 2 + 6 },
            Round::BX => { 1 + 0 },
            Round::BY => { 2 + 3 },
            Round::BZ => { 3 + 6 },
            Round::CX => { 2 + 0 },
            Round::CY => { 3 + 3 },
            Round::CZ => { 1 + 6 },
        }
    }
}

fn part_a(input: &str) -> u32 {
    input.lines().map(|s| Round::from_str(s).score_part_a()).sum()
}

fn part_b(input: &str) -> u32 {
    input.lines().map(|s| Round::from_str(s).score_part_b()).sum()
}

pub fn run_part_a() -> u32 {
    let input = include_str!("../input/2.txt");
    part_a(input)
}

pub fn run_part_b() -> u32 {
    let input = include_str!("../input/2.txt");
    part_b(input)
}

#[test]
fn part_a_test() {
    let input = include_str!("../input_test/2.txt");
    assert_eq!(part_a(input), 15);
}

#[test]
fn part_b_test() {
    let input = include_str!("../input_test/2.txt");
    assert_eq!(part_b(input), 12);
}
