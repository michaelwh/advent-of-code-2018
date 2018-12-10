use std::collections::HashMap;

fn main() {
    let lines = include_str!("input.txt").lines().collect::<Vec<_>>();
    println!("Day 3 part 1: {}", overlapper(&lines[..]));
    println!("Day 3 part 2: {}", not_overlapper(&lines[..]).unwrap().id);
}

struct Spec {
    id: u32,
    start: (u32, u32),
    size: (u32, u32)
}

fn spec_parser(spec_string: &str) -> Spec {
    let numbers = spec_string.split(['#', '@', ',', ':', 'x'].as_ref()).skip(1)
                             .map(|s| -> u32 {s.trim().parse().unwrap()}).collect::<Vec<_>>();

    return match numbers.as_slice() {
        [id, start_x, start_y, size_x, size_y] => Ok(Spec { id: *id, start: (*start_x, *start_y), size: (*size_x, *size_y)}),
        _ => Err("Could not parse spec")
    }.unwrap()
}

fn cells(spec: &Spec) -> Vec<(u32, u32)> {
    let mut vector = Vec::new();
    for x in spec.start.0..spec.start.0 + spec.size.0 {
        for y in spec.start.1..spec.start.1 + spec.size.1 {
            vector.push((x, y));
        }
    }
    vector
}

fn gridder(spec_strings: &[&str]) -> HashMap<(u32, u32), usize> {
    let mut claimed = HashMap::new();
    for spec_string in spec_strings {
        let spec = spec_parser(spec_string);
        for cell in cells(&spec) {
            match claimed.get(&cell) {
                Some(&num) => claimed.insert(cell, num + 1),
                _ => claimed.insert(cell, 1)
            };
        }
    }
    claimed
}

fn overlapper(spec_strings: &[&str]) -> usize {
    let claimed = gridder(spec_strings);
    let mut num_overlapped: usize = 0;
    for (_cell, num) in claimed {
        if num > 1 {
            num_overlapped += 1
        }
    }
    num_overlapped
}

fn not_overlapper(spec_strings: &[&str]) -> Option<Spec> {
    let claimed = gridder(spec_strings);

    for spec_string in spec_strings {
        let spec = spec_parser(spec_string);
        if cells(&spec).iter().all(|cell| claimed.get(cell).unwrap_or(&0) <= &1) {
            return Some(spec);
        }
    }
    None
}

#[test]
fn given_zeros_spec_gives_zeros() {
    let spec = spec_parser(&"#0 @ 0,0: 0x0");
    assert_eq!(spec.id, 0);
    assert_eq!(spec.start, (0, 0));
    assert_eq!(spec.size, (0, 0));
}

#[test]
fn given_nonzero_single_digit_id() {
    assert_eq!(spec_parser(&"#1 @ 0,0: 0x0").id, 1);
}

#[test]
fn given_nonzero_multi_digit_id() {
    assert_eq!(spec_parser(&"#123 @ 0,0: 0x0").id, 123);
}

#[test]
fn given_nonzero_multi_digit_start_x() {
    assert_eq!(spec_parser(&"#123 @ 456,0: 0x0").start.0, 456);
}

#[test]
fn given_nonzero_multi_digit_start_y() {
    assert_eq!(spec_parser(&"#123 @ 456,789: 0x0").start.1, 789);
}

#[test]
fn given_nonzero_multi_digit_size_x() {
    assert_eq!(spec_parser(&"#123 @ 456,789: 123x0").size.0, 123);
}

#[test]
fn given_nonzero_multi_digit_size_y() {
    assert_eq!(spec_parser(&"#123 @ 456,789: 123x456").size.1, 456);
}

#[test]
fn cells_given_1x1_at_0_0() {
    assert_eq!(cells(&Spec { id: 0, start: (0, 0), size: (1, 1)}), vec![(0, 0)])
}

#[test]
fn cells_given_2x2_at_0_0() {
    let c = cells(&Spec { id: 0, start: (0, 0), size: (2, 2)});
    assert!(c.contains(&(0, 0)));
    assert!(c.contains(&(0, 1)));
    assert!(c.contains(&(1, 0)));
    assert!(c.contains(&(1, 1)));
}

#[test]
fn cells_given_1x1_at_1_3() {
    assert_eq!(cells(&Spec { id: 0, start: (1, 3), size: (1, 1)}), vec![(1, 3)]);
}

#[test]
fn overlapper_given_example() {
    assert_eq!(overlapper(&["#1 @ 1,3: 4x4", "#2 @ 3,1: 4x4", "#3 @ 5,5: 2x2"]), 4);
}

#[test]
fn not_overlapper_given_example() {
    assert_eq!(not_overlapper(&["#1 @ 1,3: 4x4", "#2 @ 3,1: 4x4", "#3 @ 5,5: 2x2"]).unwrap().id, 3);
}

// working, but worse

#[allow(dead_code)]
fn spec_parser_first_attempt(spec_string: &str) -> Spec {
    let mut whitespace_split = spec_string.split_whitespace();
    let id = whitespace_split.next().unwrap().chars().skip(1).collect::<String>().to_string().parse().unwrap();
    let rest_of_string = whitespace_split.collect::<String>().to_string();
    let mut string_iter = rest_of_string.chars();
    let start_x = string_iter.by_ref().skip_while(|c| *c != '@').skip(1).take_while(|c| *c != ',').collect::<String>().parse().unwrap();
    let start_y = string_iter.by_ref().take_while(|c| *c != ':').collect::<String>().parse().unwrap();
    let size_x = string_iter.by_ref().take_while(|c| *c != 'x').collect::<String>().parse().unwrap();
    let size_y = string_iter.by_ref().collect::<String>().parse().unwrap();

    Spec {
        id: id,
        start: (start_x, start_y),
        size: (size_x, size_y)
    }
}

#[allow(dead_code)]
fn spec_parser_second_attempt(spec_string: &str) -> Spec {
    let mut parts = spec_string.split(['#', '@', ',', ':', 'x'].as_ref()).skip(1);
    let id = parts.next().unwrap().trim().parse().unwrap();
    let start_x = parts.next().unwrap().trim().parse().unwrap();
    let start_y = parts.next().unwrap().trim().parse().unwrap();
    let size_x = parts.next().unwrap().trim().parse().unwrap();
    let size_y = parts.next().unwrap().trim().parse().unwrap();
    Spec {
        id: id,
        start: (start_x, start_y),
        size: (size_x, size_y)
    }
}