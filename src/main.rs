mod day_01;

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
    );
}