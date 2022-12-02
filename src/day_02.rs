#[derive(Debug)]
enum RPS {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl RPS {
    fn from_char(s: &str) -> RPS {
        match s {
            "A" | "X" => RPS::Rock,
            "B" | "Y" => RPS::Paper,
            "C" | "Z" => RPS::Scissors,
            _ => panic!("Unrecognized RPS character: {}", s),
        }
    }

    fn beats(&self, other: &RPS) -> Outcome {
        match (*self as i32 - *other as i32).rem_euclid(3) {
            0 => Outcome::Draw,
            1 => Outcome::Win,
            2 => Outcome::Loss,
            _ => panic!(
                "Didn't expect outcome value {:?} for self {:?} and other {:?}",
                (*self as i32 - *other as i32).rem_euclid(3),
                self,
                other
            ),
        }
    }

    fn score(&self, other: &RPS) -> u32 {
        *self as u32 + self.beats(other) as u32
    }
}

enum Outcome {
    Win = 6,
    Draw = 3,
    Loss = 0,
}

fn get_round_score(s: &str) -> u32 {
    if let Some((opp, me)) = s.split_once(" ") {
        let opp_rps = RPS::from_char(opp);
        let me_rps = RPS::from_char(me);
        me_rps.score(&opp_rps)
    } else {
        0
    }
}

fn part_a(input: &str) -> u32 {
    input.lines().map(|s| get_round_score(s)).sum()
}

fn part_b(input: &str) -> u32 {
    0
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
