use itertools::Itertools;
/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    println!("input: {}", code);
    if code.chars().filter(|&c| c != ' ').any(|c| !c.is_digit(10)) {
        false
    }else {
        if code.chars().filter(|&c| c != ' ').count() <= 1 {
            return false;
        }
        let sum = code.chars().filter(|&c| c != ' ')
            .map(|c| c.to_digit(10).unwrap())
            .rev()
            .batching(|it|{
                match it.next() {
                    None => None,
                    Some(x) => match it.next() {
                        None => Some(x),
                        Some(y) => {
                            println!("{}, {}", x,y);
                            if y * 2 > 9 {
                                Some(x + y * 2 - 9)
                            }else {
                                Some(x + y * 2)
                            }
                        },
                    }
                }
            }).sum::<u32>();
        println!("sum: {}", sum);
        if sum % 10 == 0 {
            true
        }else {
            false
        }
    }
}
