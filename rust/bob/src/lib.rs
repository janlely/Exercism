pub fn reply(message: &str) -> &str {
    if message.len() == 0 || message.chars().all(|x| x.is_whitespace()) {
        return "Fine. Be that way!";
    }
    if message.chars().all(|x| !x.is_alphabetic() || x.is_uppercase()) {
        if message.chars().last() == Some('?') {
            return "Calm down, I know what I'm doing!";
        }else {
            return "Whoa, chill out!";
        }
    }else {
        if message.chars().last() == Some('?') {
            return "Sure.";
        }else {
            return "Whatever.";
        }
    }
}
