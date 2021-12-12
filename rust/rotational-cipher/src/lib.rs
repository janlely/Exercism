pub fn rotate(input: &str, key: i8) -> String {
    input.chars().map(|c| {
        if c.is_ascii_lowercase() {
            return convert(c, key, 'a' as i8);
        }
        if c.is_ascii_uppercase() {
            return convert(c, key, 'A' as i8);
        }
        c
    }).collect()
}

pub fn convert(c: char, key: i8, base: i8) -> char {
    let ci = c as i8;
    ((ci - base + key) % 26 + base) as u8 as char
}
