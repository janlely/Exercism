use regex::Regex;

pub struct WordProblem;

#[derive(Copy, Clone, PartialEq)]
pub enum Operator {
    PLUS,
    MINUS,
    MULTIPLIED_BY,
    DIVIDED_BY,
    POWER
}
pub fn answer(command: &str) -> Option<i32> {
    let mut src = command.to_string();
    src.pop();
    let words: Vec<&str> = src.split_whitespace().collect();
    println!("{:?}", words);
    let mut idx = 0;
    let mut started = false;
    let mut left_value: Option<i32> = None;
    let mut operator: Option<Operator> = None;
    while idx < words.len() {
        match words[idx] {
            "What" => {
                match started {
                    true => {return None;},
                    false => {
                        started = true;
                        idx += 1;
                        if words[idx] != "is" {
                            return None;
                        }
                    }
                }
            },
            "plus" => {
                match left_value{
                    None => {
                        println!("missing left number to plus");
                        return None;
                    },
                    Some(_) => {
                        match operator {
                            Some(_) => {
                                println!("operator existed before plus");
                                return None;
                            },
                            None => {
                                operator = Some(Operator::PLUS);
                            }
                        }
                    }
                }
            },
            "minus" => {
                match left_value{
                    None => {
                        println!("missing left number to minus");
                        return None;
                    },
                    Some(_) => {
                        match operator {
                            Some(_) => {
                                println!("operator existed before minus");
                                return None;
                            },
                            None => {
                                operator = Some(Operator::MINUS);
                            }
                        }
                    }
                }
            },
            "multiplied" => {
                match left_value{
                    None => {
                        println!("missing left number to multiplied by");
                        return None;
                    },
                    Some(_) => {
                        match operator {
                            Some(_) => {
                                println!("operator existed before multiplied by");
                                return None;
                            },
                            None => {
                                idx += 1;
                                if words[idx] != "by" {
                                    println!("multiplied mast be followed by 'by'");
                                    return None;
                                }else {
                                    operator = Some(Operator::MULTIPLIED_BY);
                                }
                            }
                        }
                    }
                }
            },
            "divided" => {
                match left_value{
                    None => {
                        println!("missing left number to divided by");
                        return None;
                    },
                    Some(_) => {
                        match operator {
                            Some(_) => {
                                println!("operator existed before divided by");
                                return None;
                            },
                            None => {
                                idx += 1;
                                if words[idx] != "by" {
                                    println!("divided mast be followed by 'by'");
                                    return None;
                                }else {
                                    operator = Some(Operator::DIVIDED_BY);
                                }
                            }
                        }
                    }
                }
            },
            "raised" => {
                match left_value{
                    None => {
                        println!("missing left number to power");
                        return None;
                    },
                    Some(_) => {
                        match operator {
                            Some(_) => {
                                println!("operator existed before power");
                                return None;
                            },
                            None => {
                                idx += 1;
                                if words[idx] != "to" {
                                    idx += 1;
                                    if words[idx] != "the" {
                                        println!("raise mast be followed by 'to the'");
                                        return None;
                                    }
                                }else {
                                    operator = Some(Operator::POWER);
                                }
                            }
                        }
                    }
                }
            },
            number => {
                if is_number(number) {
                    match left_value {
                        None => left_value = Some(number.parse().unwrap()),
                        Some(v) => {
                            if operator == None {
                                println!("missing operator");
                                return None;
                            }else {
                                left_value = cal(v, operator.unwrap(), number.parse().unwrap());
                                operator = None;
                            }
                        }
                    }
                }else if is_nth(number) {
                    let mut the_num: String = number.to_string();
                    the_num.pop();
                    the_num.pop();
                    idx += 1;
                    if words[idx] != "power" {
                        return None;
                    }else{
                        match left_value {
                            None => {return None;},
                            Some(v) => {
                                if operator == None {
                                    println!("missing operator");
                                    return None;
                                }else {
                                    left_value = cal(v, operator.unwrap(), the_num.parse().unwrap());
                                    operator = None;
                                }
                            }
                        }
                    }
                }else {
                    println!("invalid string: {}", words[idx]);
                    return None;
                }
            }
        }
        idx += 1;
    }
    if operator != None  {
        None
    }else {
        left_value
    }

}

fn cal(left: i32, op: Operator, right: i32) -> Option<i32> {
    Some(match op {
        Operator::PLUS => left + right,
        Operator::MINUS => left - right,
        Operator::MULTIPLIED_BY => left * right,
        Operator::DIVIDED_BY => left / right,
        Operator::POWER => left.pow(right as u32)
    })
}

fn is_nth(s: &str) -> bool {
    Regex::new("\\d+th").unwrap().is_match(s)
}

fn is_number(s: &str) -> bool {
    Regex::new("-?\\d+").unwrap().is_match(s)
}


