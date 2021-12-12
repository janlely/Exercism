pub fn factors(n: u64) -> Vec<u64> {
    let mut res = vec![];
    let mut factor = 2;
    let mut remain = n;
    loop {
        if remain == 1 {
            break res
        }
        if remain % factor == 0 {
            res.push(factor);
            while remain % factor == 0 {
                remain = remain / factor;
            }
        }
        factor += 1;
    }
}
