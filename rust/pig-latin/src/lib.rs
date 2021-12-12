use std::collections::HashSet;
pub fn translate(input: &str) -> String {
    if meet_rule1(input) {
        let mut res = String::from(input);
        res.push_str("ay");
        return res;
    }
    let res2 = meet_rule23(input);
    if res2.0 {
        let mut res = String::new();
        res.push_str(&input[res2.1..]);
        res.push_str(&input[0..res2.1]);
        res.push_str("ay");
        return res;
    }
    return String::from(input);
}

pub fn meet_rule1(input: &str) -> bool{
    let vowel = "aeiou".chars().collect::<HashSet<char>>();
    return input[0..2] == *"xr" || input[0..2] == *"yt" || vowel.contains(&input.chars().nth(0).unwrap());
}

pub fn meet_rule23(input: &str) -> (bool, usize) {
    let consonant = "bcdfghjklmnpqrstvwxz".chars().collect::<HashSet<char>>();
    let mut idx:usize = 0;
    for ch in input.chars() {
        if !consonant.contains(&ch) {
            break;
        }
        idx += 1;
    }
    if idx > 0 {
        if input[1..3] == *"qu" {
            idx += 1;
        }
        return (true, idx);
    }
    return (false, idx);
}
