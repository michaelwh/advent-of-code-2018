fn main() {
    let lines = include_str!("input.txt").lines().collect::<Vec<_>>();
    println!("Day 2 part 1: {}", checksum(&lines[..]));
    println!("Day 2 part 2: {}", id_common_letters_enfinder(&lines[..]));
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

fn differ(this: &str, that: &str) -> Option<usize> {
    assert_eq!(this.len(), that.len());

    let mut pos_different: Option<usize> = None;

    for (i, (this_char, that_char)) in this.chars().zip(that.chars()).enumerate() {
        if this_char != that_char {
            if pos_different.is_some() {
                // more than one letter different
                return None
            }
            pos_different = Some(i);
        }
    }
    pos_different
}

fn destroyer_of_letter(puny_string: &str, pos: usize) -> String {
    let mut string_under_destruction = puny_string.to_string();
    string_under_destruction.remove(pos);
    string_under_destruction
}

fn id_common_letters_enfinder(ids: &[&str]) -> String {
    for this_id in ids.iter() {
        for that_id in ids.iter() {
            if let Some(i) = differ(this_id, that_id) {
                return destroyer_of_letter(this_id, i)
            }
        }
    }
    "".to_string()
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

#[test]
fn differ_given_identical_gives_returns_false() {
    assert_eq!(differ("aaa", "aaa"), None);
}

#[test]
fn differ_given_two_different_single_letters_returns_0() {
    assert_eq!(differ("a", "b"), Some(0));
}

#[test]
fn differ_given_two_with_one_letter_different_returns_pos() {
    assert_eq!(differ("aa", "ab"), Some(1));
}

#[test]
fn differ_given_two_with_two_different_returns_none() {
    assert_eq!(differ("aa", "bc"), None);
}

#[test]
fn destroyer_of_letter_removes_picked_letter() {
    assert_eq!(destroyer_of_letter("fghij", 2), "fgij");
}

#[test]
fn id_common_letters_enfinder_finds_thing() {
    assert_eq!(id_common_letters_enfinder(&["abcde", "fghij", "klmno", "pqrst", "fguij", "axcye", "wvxyz"]), "fgij");
}