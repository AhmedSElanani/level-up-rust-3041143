extern crate chrono;

trait Deadline {
    fn is_passed(&self) -> bool;
}

use chrono::prelude::*;

#[derive(Debug)]
struct ImportantEvent {
    name: String,
    date: DateTime<Utc>,
}

impl Deadline for ImportantEvent {
    fn is_passed(&self) -> bool {
        return self.date < Utc::now();
    }
}

fn main() {
    let event = ImportantEvent {
        name: String::from("Time now"),
        date: Utc::now(),
    };

    println!("{:#?}", event);
}

// Tests are inspired from the course
#[test]
fn in_past() {
    use chrono::Duration;

    let event = ImportantEvent {
        name: String::from("friend's birthday"),
        date: (Utc::now() - Duration::hours(25)).into(),
    };

    assert!(event.is_passed())
}

#[test]
fn in_future() {
    use chrono::Duration;

    let event = ImportantEvent {
        name: String::from("friend's birthday"),
        date: (Utc::now() + Duration::hours(25)).into(),
    };

    println!("{:#?}", Local::now());

    assert!(!event.is_passed())
}
