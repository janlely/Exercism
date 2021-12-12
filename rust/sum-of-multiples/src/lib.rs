pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (1..limit).fold(0, |acc, x| {
        if factors.iter().any(|y| x % y == 0) {
            acc + x
        }else{
            acc
        }
    })
}
