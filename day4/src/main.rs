fn main() {
    println!("Hello, world!");
}

fn get_timestamp(message: &str) -> &str {
    message
}

#[test]
fn given_timestamped_message_extracts_timestamp() {
    assert_eq!(get_timestamp(&"[1518-11-01 00:00] Guard #10 begins shift"),  "21518-11-01 00:00")
}
