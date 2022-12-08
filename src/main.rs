use std::fs;

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;

macro_rules! run_all {
    ($($l:ident),+ $(,)?) => {
        {
            $(
                println!("{}a: {}", stringify!($l), $l::run_part_a());
                println!("{}b: {}\n", stringify!($l), $l::run_part_b());
            )+
        }
    }
}

#[macro_export]
macro_rules! boilerplate {
    ($day:literal, $test_part_a:literal, $test_part_b: literal, $ret_type: ty) => {
        use crate::get_input;
        pub fn run_part_a() -> $ret_type {
            let input = get_input($day, false).unwrap();
            let input = input.as_str();
            part_a(input)
        }

        pub fn run_part_b() -> $ret_type {
            let input = get_input($day, false).unwrap();
            let input = input.as_str();
            part_b(input)
        }

        #[test]
        fn part_a_test() {
            let input = get_input($day, true).unwrap();
            let input = input.as_str();
            assert_eq!(part_a(input), $test_part_a);
        }

        #[test]
        fn part_b_test() {
            let input = get_input($day, true).unwrap();
            let input = input.as_str();
            assert_eq!(part_b(input), $test_part_b);
        }
    };
}

fn get_input(day: u8, test: bool) -> Result<String, std::io::Error> {
    let path: String = if test {
        format!("./input_test/{}.txt", day)
    } else {
        format!("./input/{}.txt", day)
    };
    fs::read_to_string(path)
}

fn main() {
    run_all!(day_01, day_02, day_03, day_04, day_05, day_06, day_07, day_08);
}
