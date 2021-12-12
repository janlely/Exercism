pub fn brackets_are_balanced(string: &str) -> bool {
    //let mut left_brackets = String::new();
    //let mut str_iter = string.chars().into_iter();
    //while let Some(c) = str_iter.next() {
        //match c {
            //'[' | '{' | '(' => left_brackets.push(c),
            //']' | '}' | ')' if left_brackets.pop() != Some(c) => return false,
            //_ => {}
        //}
    //}
    //left_brackets.is_empty()
    let res = string.chars().any(|c| {
        match c {
            '[' | '{' | '(' => {
                left_brackets.push(c);
                false
            },
            ']' => {
                if Some('[') == left_brackets.pop() {
                    false
                }else {
                    true
                }
            },
            '}' => {
                if Some('{') == left_brackets.pop() {
                    false
                }else {
                    true
                }
            },
            ')' => {
                if Some('(') == left_brackets.pop() {
                    false
                }else {
                    true
                }
            },
            _ => false,
        }
    });
    left_brackets.is_empty() && !res
}
