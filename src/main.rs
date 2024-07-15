use std::str::FromStr;

struct Isbn {
    raw: String,
    digits: Vec<u8>,
}

// error options are inspired by the course
#[derive(Debug)]
enum InvalidIsbn {
    TooLong,
    TooShort,
    // InvalidCharacters(usize, char),
    FailedCheckSum,
}

impl FromStr for Isbn {
    type Err = InvalidIsbn;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let concatenated: String = s.split('-').collect::<Vec<&str>>().concat();
        let digits: Vec<u8> = concatenated
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect();

        if digits.len() > 13 {
            return Err(InvalidIsbn::TooLong);
        }

        if digits.len() < 13 {
            return Err(InvalidIsbn::TooShort);
        }

        if digits[12] != calculate_check_digit(&digits) {
            return Err(InvalidIsbn::FailedCheckSum);
        }

        return Ok(Isbn {
            raw: s.to_string(),
            digits: digits[0..12].to_vec(),
        });
    }
}

impl std::fmt::Display for Isbn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.raw)
    }
}

// https://en.wikipedia.org/wiki/International_Standard_Book_Number#ISBN-13_check_digit_calculation
fn calculate_check_digit(digits: &[u8]) -> u8 {
    let weighted_sum: u8 = digits
        .iter()
        .enumerate()
        .map(|(index, &value)| {
            let weight = if index % 2 == 0 { 1 } else { 3 };
            value as u8 * weight
        })
        .sum();

    let r = 10 - (weighted_sum) % 10;

    return if r == 10 { 0 } else { r };
}

fn main() {
    let rust_in_action: Isbn = "978-3-16-148410-0".parse().unwrap();

    println!("Rust in Action's ISBN-13 ({})is valid!", rust_in_action);
}

// tests from the course
#[test]
fn can_correctly_calculate_check_digits() {
    let cases = [
        ([9_u8, 7, 8, 1, 8, 6, 1, 9, 7, 8, 7, 6], 9_u8),
        ([9_u8, 7, 8, 3, 1, 6, 1, 4, 8, 4, 1, 0], 0_u8),
    ];

    for (case, check) in cases.iter() {
        let actual = calculate_check_digit(case);
        println!("{:?} -> {}?  {}", &case, check, actual);
        assert_eq!(calculate_check_digit(case), *check)
    }
}

#[test]
fn rust_in_action() {
    let _: Isbn = "978-3-16-148410-0".parse().unwrap();
}
