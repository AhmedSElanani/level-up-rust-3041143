mod run_length_encoding {
    pub fn encode(text: &str) -> String {
        let mut result = String::with_capacity(text.len());

        let mut count = 0;
        let mut prev_char = text.chars().nth(0).unwrap();
        for c in text.chars() {
            if c == prev_char {
                count += 1;

                if count == 10 {
                    result.push_str(&format!("{}{}", count - 1, prev_char));
                    count = 1;
                }
            } else {
                result.push_str(&format!("{}{}", count, prev_char));
                count = 1;
            }

            prev_char = c;
        }

        result.push_str(&format!("{}{}", count, prev_char));
        count = 1;
        return result;
    }

    pub fn decode(text: &str) -> String {
        let mut result = String::with_capacity(text.len());
        let mut chars = text.chars();

        while let (Some(n), Some(c)) = (chars.next(), chars.next()) {
            let n = n.to_digit(10).unwrap();
            for _ in 0..n {
                result.push(c);
            }
        }

        return result;

        // My Solution
        // let mut result = String::with_capacity(text.len());
        // let mut i = 0;
        // while i < text.len() {
        //     // Read the number
        //     let number_str = &text[i..i + 1];
        //     let count: u8 = number_str.parse().expect("Invalid number in input");

        //     // Move the index to the character
        //     i += 1;
        //     let char_to_repeat = text[i..i + 1]
        //         .chars()
        //         .next()
        //         .expect("Invalid character in input");

        //     // Append the repeated character to the result string
        //     result.push_str(&char_to_repeat.to_string().repeat(count as usize));

        //     // Move past the current character
        //     i += 1;
        // }

        // return result;
    }
}

fn main() {
    //
}

#[test]
fn abc() {
    use run_length_encoding::*;

    assert_eq!(encode("abc"), "1a1b1c");
}

#[test]
fn round_trip() {
    use run_length_encoding::*;

    let input = "LinkedIn";
    println!("{}", encode(input));
    assert_eq!(decode(&encode(input)), input);
}

#[test]
fn long_run() {
    use run_length_encoding::*;

    let input = "AAAAA AAAAAAAAAA AAAAAAAAAAAAAAAAAAAA";
    assert_eq!(encode(input), "5A1 9A1A1 9A9A2A");
}
