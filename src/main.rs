use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Rgb {
    red: u8,
    green: u8,
    blue: u8,
}

trait RgbChannels {
    fn r(&self) -> u8;

    fn g(&self) -> u8;

    fn b(&self) -> u8;
}

impl RgbChannels for Rgb {
    fn r(&self) -> u8 {
        return self.red;
    }

    fn g(&self) -> u8 {
        return self.green;
    }

    fn b(&self) -> u8 {
        return self.blue;
    }
}

// error options are inspired by the course
#[derive(Debug)]
enum InvalidRgb {
    TooLong,
    TooShort,
    NoLeadingHash,
    RedOutOfBounds,
    GreenOutOfBounds,
    BlueOutOfBounds,
}

impl FromStr for Rgb {
    type Err = InvalidRgb;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() > 7 {
            return Err(InvalidRgb::TooLong);
        }

        if s.len() < 7 {
            return Err(InvalidRgb::TooShort);
        }

        if s.chars().nth(0) != Some('#') {
            return Err(InvalidRgb::NoLeadingHash);
        }

        let red = u8::from_str_radix(&s[1..3], 16).map_err(|_| InvalidRgb::RedOutOfBounds)?;
        let green = u8::from_str_radix(&s[3..5], 16).map_err(|_| InvalidRgb::GreenOutOfBounds)?;
        let blue = u8::from_str_radix(&s[5..7], 16).map_err(|_| InvalidRgb::BlueOutOfBounds)?;

        return Ok(Rgb { red, green, blue });
    }
}

impl Display for Rgb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{:02x}{:02x}{:02x}", self.r(), self.g(), self.b())
    }
}

fn main() {
    //
}

#[test]
fn every_color() {
    let colors = (0_u8..255).zip(0_u8..255).zip(0_u8..255);

    for ((r, g), b) in colors {
        let hex = format!("#{:02x}{:02x}{:02x}", r, g, b);
        let color: Rgb = hex.parse().unwrap();
        assert_eq!(hex, format!("{}", color));
    }
}

#[test]
#[should_panic]
fn too_short() {
    let _: Rgb = "1234".parse().unwrap();
}

#[test]
#[should_panic]
fn not_a_hex_code() {
    let _: Rgb = "?".parse().unwrap();
}

#[test]
#[should_panic]
fn invalid_literals() {
    let _: Rgb = "?".parse().unwrap();
}

#[test]
#[should_panic]
fn no_leading_hash() {
    let _: Rgb = "aabbcc".parse().unwrap();
}

#[test]
#[should_panic]
fn out_of_bounds() {
    let _: Rgb = "00gg00".parse().unwrap();
}
