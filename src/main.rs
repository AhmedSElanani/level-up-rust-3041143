mod vigenere {
    const ALPHABET_BYTES: &[u8] = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".as_bytes();
    const ALPHABET_LEN: u8 = ALPHABET_BYTES.len() as u8;

    pub fn encrypt(plaintext: &str, key: &str) -> String {
        String::new()
    }

    pub fn decrypt(ciphertext: &str, key: &str) -> String {
        let key_bytes = key.as_bytes();

        let mut key_index = 0;
        let mut plaintext = String::new();
        for &c in ciphertext.as_bytes().iter() {
            if let Some(c_index) = ALPHABET_BYTES.iter().position(|&x| x == c) {
                let offset =
                    (ALPHABET_BYTES[c_index] + ALPHABET_LEN - key_bytes[key_index]) % ALPHABET_LEN;
                let result = ALPHABET_BYTES[offset as usize];

                key_index = (key_index + 1) % key_bytes.len();
                plaintext.push(result as char);
            }
        }

        return plaintext;
    }
}

fn main() {
    let key = "WHYRUST";
    let ciphertext = "
    PVCDJG
    PAYCMY
    JR KUC
    ";
    let plaintext = vigenere::decrypt(&ciphertext, key);

    println!("{}", plaintext);
}
