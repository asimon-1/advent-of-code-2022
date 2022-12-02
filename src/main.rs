#[macro_use]
extern crate num_derive;

mod day_01;
mod day_02;

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

fn main() {
    run_all!(
        day_01,
        day_02,
    );
}