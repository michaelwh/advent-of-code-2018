use std::fs::File;
use std::io::Read;

fn main() {
    let mut f = File::open("input.txt").expect("error: cannot open file");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("error: something went wrong reading the file");
    println!("{}", thingy(&frequencies_from_string(&contents)))
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