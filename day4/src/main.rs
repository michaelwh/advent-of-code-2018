#[macro_use] extern crate lazy_static;
extern crate regex;
use regex::Regex;
use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

#[derive(Debug, PartialEq)]
enum Message {
    WakesUp,
    FallsAsleep,
    BeginsShift(u32)
}

enum GuardState {
    OffShift,
    Asleep,
    Awake
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

fn most_awake(messages: &[&str]) -> u32 {
    let mut guards = HashMap::new();
    let sorted_messages = sort_messages(messages);
    let mut current_guard = 0;
    let mut asleep_start_time = 0;
    let mut time_asleep = 0;

    for message in messages {
        match classify_message(message) {
            Message::BeginsShift(id) => {
                current_guard = id;
                time_asleep = 0;
            },
            Message::WakesUp => {
                time_asleep += get_minute(message) - asleep_start_time;
                guards.insert(current_guard, guards.get(&current_guard).unwrap_or(&0) + time_asleep);
            },
            Message::FallsAsleep => {
                asleep_start_time = get_minute(message);
            }
        }
    }
    let mut guards_asleep_time: Vec<(&u32, &u32)> = guards.iter().collect();
    guards_asleep_time.sort_by(|(_, time_a), (_, time_b)| time_a.cmp(time_b));
    let (&id, &time) = guards_asleep_time.last().unwrap();
    id
}

fn classify_message(message: &str) -> Message {
    lazy_static! {
        static ref FALLS_ALSEEP_RE: Regex = Regex::new(r"\[.+\] falls asleep").unwrap();
        static ref BEGINS_SHIFT_RE: Regex = Regex::new(r"\[.+\] Guard #(.+) begins shift").unwrap();
    }
    if FALLS_ALSEEP_RE.is_match(message) {
        return Message::FallsAsleep
    } else if BEGINS_SHIFT_RE.is_match(message) {
        return Message::BeginsShift(BEGINS_SHIFT_RE.captures(message).unwrap()[1].to_string().parse().unwrap())
    }
    Message::WakesUp
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

#[test]
fn test_gets_guard_with_most_minutes_awake() {
    let test_data = "[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up".lines().collect::<Vec<_>>();

    assert_eq!(most_awake(&test_data[..]), 10);
}

#[test]
fn test_classify_message() {
    assert_eq!(classify_message("[1518-11-02 00:50] wakes up"),  Message::WakesUp);
    assert_eq!(classify_message("[1518-11-01 00:05] falls asleep"), Message::FallsAsleep);
    assert_eq!(classify_message("[1518-11-01 00:00] Guard #10 begins shift"), Message::BeginsShift(10));
}