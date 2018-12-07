use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");
    println!("{}", thingy_day2(&frequencies_from_string(&input)))
}

fn frequencies_from_string(freq_string: &str) -> Vec<i32> {
    let thing = freq_string.lines().map(|s| s.parse().unwrap()).collect::<Vec<i32>>();
    thing
}

fn thingy(freq: &[i32]) -> i32 {
    let mut total = 0;
    for f in freq {
        total += f;
    }
    total
}

fn thingy_day2(freq: &[i32]) -> i32 {
    let mut set = HashSet::new();
    set.insert(0);
    let mut total = 0;
    for f in freq.iter().cycle() {
        total += f;
        if set.contains(&total) {
            return total
        }
        set.insert(total);
    }
    0
}

#[test]
fn day2_given_plusone_minusone_returns_0()
{
    assert_eq!(thingy_day2(&[1, -1]), 0)
}

#[test]
fn given_nothing_returns_zero() {
    assert_eq!(thingy(&[]), 0);
}

#[test]
fn given_single_one_returns_one() {
    assert_eq!(thingy(&[1]), 1);
}

#[test]
fn given_empty_string_returns_empty_list() {
    assert_eq!(frequencies_from_string(""), []);
}

#[test]
fn given_single_number_string_returns_single_element() {
    assert_eq!(frequencies_from_string("1"), [1]);
}

#[test]
fn given_test_cases() {
    assert_eq!(thingy(&[1, 1, 1]), 3);
    assert_eq!(thingy(&[1, 1, -2]), 0);
    assert_eq!(thingy(&[-1, -2, -3]), -6);
}