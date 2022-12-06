use std::collections::HashSet;

fn find_marker(input: &str, marker_size: usize) -> Option<u32> {
    let slice: Vec<char> = input.chars().collect();
    let windows = slice.windows(marker_size);
    'w: for (ind, window) in windows.enumerate() {
        let mut s: HashSet<char> = HashSet::new();
        for character in window {
            if !s.insert(character.clone()) {
                // s.insert() returns false if the element is already in the set
                // so if we enter this block, we know there's a duplicate char
                // in the window, and should move on to the next window.
                continue 'w;
            }
        }
        // Didn't find any duplicate chars, so they are all unique.
        return Some((ind + marker_size) as u32);
    }
    // Didn't find any windows without duplicate characters
    None
}

fn part_a(input: &str) -> u32 {
    find_marker(input, 4).unwrap()
}

fn part_b(input: &str) -> u32 {
    find_marker(input, 14).unwrap()
}

pub use crate::boilerplate;
boilerplate!(6, 7, 19, u32);

#[test]
fn part_a_test_2() {
    let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    assert_eq!(part_a(input), 5);
}

#[test]
fn part_a_test_3() {
    let input = "nppdvjthqldpwncqszvftbrmjlhg";
    assert_eq!(part_a(input), 6);
}

#[test]
fn part_a_test_4() {
    let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    assert_eq!(part_a(input), 10);
}

#[test]
fn part_a_test_5() {
    let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
    assert_eq!(part_a(input), 11);
}

#[test]
fn part_b_test_2() {
    let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    assert_eq!(part_b(input), 23);
}

#[test]
fn part_b_test_3() {
    let input = "nppdvjthqldpwncqszvftbrmjlhg";
    assert_eq!(part_b(input), 23);
}

#[test]
fn part_b_test_4() {
    let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    assert_eq!(part_b(input), 29);
}

#[test]
fn part_b_test_5() {
    let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
    assert_eq!(part_b(input), 26);
}
