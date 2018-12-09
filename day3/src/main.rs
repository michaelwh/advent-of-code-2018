fn main() {
    println!("Hello, world!");
}

struct Spec {
    id: u32,
    start: (u32, u32),
    size: (u32, u32)
}

fn spec_parser_old(spec_string: &str) -> Spec {
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

fn spec_parser_newer_old(spec_string: &str) -> Spec {
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

fn spec_parser(spec_string: &str) -> Spec {
    let numbers = spec_string.split(['#', '@', ',', ':', 'x'].as_ref()).skip(1)
                             .map(|s| -> u32 {s.trim().parse().unwrap()}).collect::<Vec<_>>();

    return match numbers.as_slice() {
        [id, start_x, start_y, size_x, size_y] => Ok(Spec { id: *id, start: (*start_x, *start_y), size: (*size_x, *size_y)}),
        _ => Err("Could not parse spec")
    }.unwrap()
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