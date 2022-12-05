use regex::Regex;

fn initialize_stacks(initial: &str) -> Vec<Vec<String>> {
    let re = Regex::new(r"[ \[]([^\d])[ \]](?: |\n)").unwrap();
    // Each column is 3 characters plus a delimiter,
    // except the last column which is missing a delimiter
    let num_stacks = (initial.lines().next().unwrap().len() + 1) / 4;
    let mut stacks: Vec<Vec<String>> = Vec::with_capacity(num_stacks);
    for _ in 0..num_stacks {
        stacks.push(Vec::new());
    }
    for (ind, cap) in re.captures_iter(initial).enumerate() {
        if &cap[1] != " " {
            stacks[ind.rem_euclid(num_stacks)].insert(0, cap[1].to_owned());
        }
    }
    stacks
}

fn execute_instructions_9000(instructions: &str, mut stacks: Vec<Vec<String>>) -> Vec<Vec<String>> {
    let re = Regex::new(r"move (\d+) from (\d) to (\d)").unwrap();
    for cap in re.captures_iter(instructions) {
        let num: usize = (&cap[1]).parse().unwrap();
        let from: usize = (&cap[2]).parse().unwrap();
        let to: usize = (&cap[3]).parse().unwrap();
        for _ in 0..num {
            let e = stacks[from - 1].pop().unwrap();
            stacks[to - 1].push(e);
        }
    }
    stacks
}

fn execute_instructions_9001(instructions: &str, mut stacks: Vec<Vec<String>>) -> Vec<Vec<String>> {
    let re = Regex::new(r"move (\d+) from (\d) to (\d)").unwrap();
    for cap in re.captures_iter(instructions) {
        let num: usize = (&cap[1]).parse().unwrap();
        let from: usize = (&cap[2]).parse().unwrap();
        let to: usize = (&cap[3]).parse().unwrap();
        // Pop into a temporary vector so that the order is retained
        let mut temp: Vec<String> = Vec::new();
        for _ in 0..num {
            let e = stacks[from - 1].pop().unwrap();
            temp.push(e);
        }
        for _ in 0..temp.len() {
            let e = temp.pop().unwrap();
            stacks[to - 1].push(e);
        }
    }
    stacks
}

fn get_top_crate(mut stacks: Vec<Vec<String>>) -> String {
    let mut s = String::new();
    for stack in stacks.iter_mut() {
        let e = stack.pop().unwrap();
        s.push_str(&e);
    }
    s
}

fn part_a(input: &str) -> String {
    let (initial, instructions) = input.split_once("\n\n").unwrap();
    let stacks = initialize_stacks(initial);
    let moved_stacks = execute_instructions_9000(instructions, stacks);
    get_top_crate(moved_stacks)
}

fn part_b(input: &str) -> String {
    let (initial, instructions) = input.split_once("\n\n").unwrap();
    let stacks = initialize_stacks(initial);
    let moved_stacks = execute_instructions_9001(instructions, stacks);
    get_top_crate(moved_stacks)
}

pub use crate::boilerplate;
boilerplate!(5, "CMZ", "MCD", String);
