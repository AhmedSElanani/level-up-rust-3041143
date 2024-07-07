#[derive(Debug, PartialEq)]
enum Pulse {
    Short,
    Long,
}

/// Represents a single character
type Letter = Vec<Pulse>;

/// Represents a string of characters
type Message = Vec<Letter>;

trait MorseCode {
    fn to_morse_code(&self) -> Message;
}

impl MorseCode for String {
    fn to_morse_code(&self) -> Message {
        // This is the course solution
        let mut msg = Vec::with_capacity(self.len());
        for c in self.chars() {
            let letter = match c {
                'A' | 'a' => vec![Pulse::Short, Pulse::Long],
                'B' | 'b' => vec![Pulse::Long, Pulse::Short, Pulse::Short, Pulse::Short],
                'C' | 'c' => vec![Pulse::Long, Pulse::Short, Pulse::Long, Pulse::Short],
                'D' | 'd' => vec![Pulse::Long, Pulse::Short, Pulse::Short],
                'E' | 'e' => vec![Pulse::Short],
                'F' | 'f' => vec![Pulse::Short, Pulse::Short, Pulse::Long, Pulse::Short],
                'G' | 'g' => vec![Pulse::Long, Pulse::Long, Pulse::Short],
                'H' | 'h' => vec![
                    Pulse::Short,
                    Pulse::Short,
                    Pulse::Short,
                    Pulse::Short,
                    Pulse::Short,
                ],
                'I' | 'i' => vec![Pulse::Short, Pulse::Short],
                'J' | 'j' => vec![Pulse::Short, Pulse::Long, Pulse::Long, Pulse::Long],
                'K' | 'k' => vec![Pulse::Long, Pulse::Short, Pulse::Long],
                'L' | 'l' => vec![Pulse::Short, Pulse::Long, Pulse::Short, Pulse::Short],
                'M' | 'm' => vec![Pulse::Long, Pulse::Long],
                'N' | 'n' => vec![Pulse::Long, Pulse::Short],
                'O' | 'o' => vec![Pulse::Long, Pulse::Long, Pulse::Long],
                'P' | 'p' => vec![Pulse::Short, Pulse::Long, Pulse::Long, Pulse::Short],
                'Q' | 'q' => vec![Pulse::Long, Pulse::Long, Pulse::Short, Pulse::Long],
                'R' | 'r' => vec![Pulse::Short, Pulse::Long, Pulse::Short],
                'S' | 's' => vec![Pulse::Short, Pulse::Short, Pulse::Short],
                'T' | 't' => vec![Pulse::Long],
                'U' | 'u' => vec![Pulse::Short, Pulse::Short, Pulse::Long],
                'V' | 'v' => vec![Pulse::Short, Pulse::Short, Pulse::Short, Pulse::Long],
                'W' | 'w' => vec![Pulse::Short, Pulse::Long, Pulse::Long],
                'X' | 'x' => vec![Pulse::Long, Pulse::Short, Pulse::Short, Pulse::Long],
                'Y' | 'y' => vec![Pulse::Long, Pulse::Short, Pulse::Long, Pulse::Long],
                'Z' | 'z' => vec![Pulse::Long, Pulse::Long, Pulse::Short, Pulse::Short],

                '1' => vec![
                    Pulse::Short,
                    Pulse::Long,
                    Pulse::Long,
                    Pulse::Long,
                    Pulse::Long,
                ],
                '2' => vec![
                    Pulse::Short,
                    Pulse::Short,
                    Pulse::Long,
                    Pulse::Long,
                    Pulse::Long,
                ],
                '3' => vec![
                    Pulse::Short,
                    Pulse::Short,
                    Pulse::Short,
                    Pulse::Long,
                    Pulse::Long,
                ],
                '4' => vec![
                    Pulse::Short,
                    Pulse::Short,
                    Pulse::Short,
                    Pulse::Short,
                    Pulse::Long,
                ],
                '5' => vec![
                    Pulse::Short,
                    Pulse::Short,
                    Pulse::Short,
                    Pulse::Short,
                    Pulse::Short,
                ],
                '6' => vec![
                    Pulse::Long,
                    Pulse::Short,
                    Pulse::Short,
                    Pulse::Short,
                    Pulse::Short,
                ],
                '7' => vec![
                    Pulse::Long,
                    Pulse::Long,
                    Pulse::Short,
                    Pulse::Short,
                    Pulse::Short,
                ],
                '8' => vec![
                    Pulse::Long,
                    Pulse::Long,
                    Pulse::Long,
                    Pulse::Short,
                    Pulse::Short,
                ],
                '9' => vec![
                    Pulse::Long,
                    Pulse::Long,
                    Pulse::Long,
                    Pulse::Long,
                    Pulse::Short,
                ],
                '0' => vec![
                    Pulse::Long,
                    Pulse::Long,
                    Pulse::Long,
                    Pulse::Long,
                    Pulse::Long,
                ],
                _ => continue,
            };

            msg.push(letter);
        }

        return msg;
    }
}

// Implement the Display trait for Pulse
impl std::fmt::Display for Pulse {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // Inspired by the course solution
        return match self {
            Pulse::Short => write!(f, "."),
            Pulse::Long => write!(f, "_"),
        };
    }
}

fn print_morse_code(morse_code: &Message) {
    // Inspired by the course solution
    for letter in morse_code.iter() {
        for pulse in letter.iter() {
            print!("{}", pulse);
        }
        print!(" ");
    }
    println!();
}

fn main() {
    let greeting = "Hello, world".to_string().to_morse_code();

    print_morse_code(&greeting);
}

// Tests from the course
#[test]
fn hello_world() {
    let expected = vec![
        vec![
            Pulse::Short,
            Pulse::Short,
            Pulse::Short,
            Pulse::Short,
            Pulse::Short,
        ],
        vec![Pulse::Short],
        vec![Pulse::Short, Pulse::Long, Pulse::Short, Pulse::Short],
        vec![Pulse::Short, Pulse::Long, Pulse::Short, Pulse::Short],
        vec![Pulse::Long, Pulse::Long, Pulse::Long],
        vec![Pulse::Short, Pulse::Long, Pulse::Long],
        vec![Pulse::Long, Pulse::Long, Pulse::Long],
        vec![Pulse::Short, Pulse::Long, Pulse::Short],
        vec![Pulse::Short, Pulse::Long, Pulse::Short, Pulse::Short],
        vec![Pulse::Long, Pulse::Short, Pulse::Short],
    ];

    let actual = "Hello, world".to_string().to_morse_code();
    assert_eq!(actual, expected);
}

#[test]
fn whole_alphabet() {
    let alphabet = "abcdefghijklmnopqrstuvwxyz1234567890".to_string();

    alphabet.to_morse_code();
    alphabet.to_uppercase().to_morse_code();
}
