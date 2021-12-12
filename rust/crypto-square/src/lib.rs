pub fn encrypt(input: &str) -> String {
    let norm: String = input.chars().filter(|c| c.is_ascii_alphabetic()).collect();
    let row = closest_sqr(norm.len());
    let col = if row * row == norm.len() {row} else {row + 1};
    let mut resp: String = String::new();
    for i in 0..row {
        for j in 0..col {
            let idx = i * row + j;
            if idx < norm.len() {
                resp.push(norm.chars().nth(idx).unwrap());
            }
        }
        if i < row - 1 {
            resp.push(' ');
        }
    }
    resp
}

pub fn closest_sqr(num: usize) -> usize {
    let mut left:usize  = 0;
    let mut right: usize = num;
    while left + 1 < right {
        let mid = (left + right) / 2;
        let mid2 = mid * mid;
        if mid2 == num {
            return mid;
        }
        if mid2 > num {
            right = mid;
        }else {
            left = mid;
        }
    }
    if num - left * left > right * right - num {
        right
    }else {
        left
    }
}
