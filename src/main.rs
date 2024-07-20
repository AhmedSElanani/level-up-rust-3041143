extern crate chrono;

use chrono::NaiveDate;

/// Parses a string that represents a date. When a date
/// is unable to be determined, return `None`.
fn flexible_date_parse(text: &str) -> Option<NaiveDate> {
    let supported_formats = ["%Y %m %d", "%Y-%m-%d", "%Y/%B/%d", "%d.%b.%Y", "%b.%d.%Y"];

    for format in &supported_formats {
        if let Ok(date) = NaiveDate::parse_from_str(text, format) {
            return Some(date);
        }
    }

    return None;
}

fn main() {
    let dates = [
        "2010-12-11",
        "1999/Mar/02",
        "01.Mar.2021",
        "Mar.05.2021",
        "not a date",
    ];

    for d in dates.iter() {
        println!("{} -> {:?}", d, flexible_date_parse(d));
    }
}

#[test]
fn ymd_hyphen() {
    assert_eq!(
        flexible_date_parse("2010-12-11"),
        Some(NaiveDate::from_ymd(2010, 12, 11))
    )
}

#[test]
fn ymd_slash() {
    assert_eq!(
        flexible_date_parse("1999/Mar/02"),
        Some(NaiveDate::from_ymd(1999, 3, 2))
    )
}

// tests from the course
#[test]
fn dmy_dot() {
    assert_eq!(
        flexible_date_parse("01.Mar.2021"),
        Some(NaiveDate::from_ymd(2021, 3, 1))
    )
}

#[test]
fn mdy_dot() {
    assert_eq!(
        flexible_date_parse("Apr.05.2021"),
        Some(NaiveDate::from_ymd(2021, 4, 5))
    )
}

#[test]
fn invalid() {
    assert_eq!(flexible_date_parse("not a date"), None)
}
