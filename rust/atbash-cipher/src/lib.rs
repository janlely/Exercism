/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    plain.chars().filter(|c| c.is_ascii_alphabetic())
        .map(|c| if c.is_ascii_uppercase() {c.to_ascii_lowercase()} else {c})
        .map(|c| {
            let base = 'a' as u8;
            let a = c as u8;
            (25 - (a - base) + base)  as char
        }).collect()
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    cipher.chars().filter(|c| c.is_ascii_alphabetic())
        .map(|c| if c.is_ascii_uppercase() {c.to_ascii_lowercase()} else {c})
        .map(|c| {
            let base = 'a' as u8;
            let a = c as u8;
            (25 - (a - base) + base)  as char
        }).collect()
}
