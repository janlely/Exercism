pub fn is_armstrong_number(num: u32) -> bool {
    let mut digits = vec![];
    let mut len = 0;
    let mut copy = num;
    while copy > 0 {
        len += 1;
        digits.push(copy % 10);
        copy = copy / 10;
    }
    num == digits.iter().fold(0, |acc, x| acc + x.pow(len))
}
