use num_traits::FromPrimitive;

#[derive(Debug, FromPrimitive)]
enum RPS {
    Rock = 0,
    Paper = 1,
    Scissors = 2,
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

    fn from_round_outcome(opp: &RPS, out: &Outcome) -> RPS {
        FromPrimitive::from_u32((*opp as u32 + *out as u32).rem_euclid(3)).unwrap()
    }

    fn match_against(&self, other: &RPS) -> Outcome {
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

    fn score(&self) -> u32 {
        match self {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
        }
    }
}

enum Outcome {
    Draw = 0,
    Win = 1,
    Loss = 2,
}

impl Outcome {
    fn from_char(s: &str) -> Outcome {
        match s {
            "X" => Outcome::Loss,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => panic!("Unrecognized outcome character: {}", s),
        }
    }

    fn score(&self) -> u32 {
        match self {
            Outcome::Loss => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        }
    }
}

fn get_round_score(me: RPS, outcome: Outcome) -> u32 {
    me.score() + outcome.score()
}

fn get_round_score_part_a(s: &str) -> u32 {
    if let Some((opp, me)) = s.split_once(" ") {
        let opp_rps = RPS::from_char(opp);
        let me_rps = RPS::from_char(me);
        let out = me_rps.match_against(&opp_rps);
        get_round_score(me_rps, out)
    } else {
        0
    }
}

fn get_round_score_part_b(s: &str) -> u32 {
    if let Some((opp, out)) = s.split_once(" ") {
        let opp_rps = RPS::from_char(opp);
        let out_outcome = Outcome::from_char(out);
        let me_rps = RPS::from_round_outcome(&opp_rps, &out_outcome);
        get_round_score(me_rps, out_outcome)
    } else {
        0
    }
}

fn part_a(input: &str) -> u32 {
    input.lines().map(|s| get_round_score_part_a(s)).sum()
}

fn part_b(input: &str) -> u32 {
    input.lines().map(|s| get_round_score_part_b(s)).sum()
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
