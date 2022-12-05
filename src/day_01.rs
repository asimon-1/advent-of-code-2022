use std::cmp::max;

fn part_a(input: &str) -> u32 {
    let mut answer: u32 = 0;
    let mut input: Vec<u32> = input
        .lines()
        .map(|x| x.parse::<u32>().unwrap_or(u32::MAX))
        .collect();
    input.push(u32::MAX); // Add delimiter after the last element

    input
        .into_iter()
        .reduce(|accum, x| {
            if x == u32::MAX {
                // At a delimiter, store the sum of that elf's calories
                // if it is the largest, then reset the accumulator to 0.
                answer = max(accum, answer);
                0
            } else {
                accum + x
            }
        });
    answer
}

fn part_b(input: &str) -> u32 {
    let mut top_three: [u32; 3] = [0, 0, 0];
    let mut input: Vec<u32> = input
        .lines()
        .map(|x| x.parse::<u32>().unwrap_or(u32::MAX))
        .collect();
    input.push(u32::MAX); // Add delimiter after the last element

    input
        .into_iter()
        .reduce(|accum, x| {
            if x == u32::MAX {
                let (index, min) = top_three.iter().enumerate().min_by(
                    |a, b| a.1.cmp(b.1)
                ).unwrap();
                if &accum > min {
                    top_three[index] = accum
                }
                0
            } else {
                accum + x
            }
        });
    top_three.iter().sum()
}

pub use crate::boilerplate;
boilerplate!(1, 24_000, 45_000, u32);