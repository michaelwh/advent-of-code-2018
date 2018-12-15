#[macro_use] extern crate lazy_static;
extern crate regex;
use regex::Regex;

fn main() {
    println!("Hello, world!");
}

fn get_timestamp(message: &str) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\[(.+)\].+").unwrap();
    }
    let caps = RE.captures(message).unwrap();
    caps[1].to_string()
}

fn sort_messages(messages: &[&str]) -> Vec<String> {
    let mut sortable_messages: Vec<String> = messages.iter().map(|s| s.to_string()).collect();
    sortable_messages.sort();
    sortable_messages
}

fn get_minute(message: &str) -> u32 {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\[.+:(..)\].+").unwrap();
    }
    let caps = RE.captures(message).unwrap();
    println!("{:?}", caps);
    caps[1].to_string().parse().unwrap()
}

#[test]
fn given_timestamped_message_extracts_timestamp() {
    assert_eq!(get_timestamp(&"[1518-11-01 00:00] Guard #10 begins shift"),  *"1518-11-01 00:00");
}


#[test]
fn test_sort_messages() {
    let unsorted = vec!["[1518-11-01 00:00] Guard #10 begins shift",
                       "[1518-11-01 00:25] wakes up",
                       "[1518-11-01 00:05] falls asleep",
                       "[1518-10-01 00:30] falls asleep"];
    assert_eq!(sort_messages(&unsorted), vec!["[1518-10-01 00:30] falls asleep",
                                             "[1518-11-01 00:00] Guard #10 begins shift",
                                            "[1518-11-01 00:05] falls asleep",
                                             "[1518-11-01 00:25] wakes up"]);
}

#[test]
fn test_get_message_minute() {
    assert_eq!(get_minute("[1518-11-01 00:25] wakes up"), 25);
}