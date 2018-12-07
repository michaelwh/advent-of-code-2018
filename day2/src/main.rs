fn main() {
    let lines = include_str!("input.txt").lines().collect::<Vec<_>>();
    println!("Day 2 part 1: {}", checksum(&lines[..]));
}

fn finder(num_unique: usize, id: &str) -> bool {
    for character in id.chars() {
        if id.chars().filter(|c| *c == character).count() == num_unique {
            return true
        }
    }
    false
}

fn checksum(ids: &[&str]) -> i32 {
    let twos = ids.iter().filter(|&id| finder(2, &id)).count() as i32;
    let threes = ids.iter().filter(|&id| finder(3, &id)).count() as i32;
    twos * threes
}

#[test]
fn two_finder_given_empty_string_finds_nothing() {
    assert_eq!(finder(2, ""), false);
}

#[test]
fn two_finder_given_string_with_just_two_repeated_letters_returns_true() {
    assert_eq!(finder(2, "aa"), true);
}

#[test]
fn two_finder_given_string_with_embedded_two_repeat_returns_true() {
    assert_eq!(finder(2, "abccd"), true);
}

#[test]
fn three_finder_given_string_with_embedded_two_repeat_returns_false() {
    assert_eq!(finder(3, "abccd"), false);
}

#[test]
fn given_no_ids_return_0() {
    assert_eq!(checksum(&[]), 0);
}

#[test]
fn given_ids_with_two_and_three_return_1() {
    assert_eq!(checksum(&["aa", "aaa"]), 1);
}

#[test]
fn given_ids_with_two_and_two_threes_return_2() {
    assert_eq!(checksum(&["aa", "aaa", "bbb"]), 2);
}

#[test]
fn given_ids_with_two_twos_and_two_threes_return_4() {
    assert_eq!(checksum(&["aa", "bb", "aaa", "bbb"]), 4);
}