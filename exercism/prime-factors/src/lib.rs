pub fn factors(mut n: u64) -> Vec<u64> {
    let mut res = vec![];
    let mut candidates = 2..;
    while n != 1 {
        let x = candidates.next().unwrap();
        while n % x == 0 {
            n /= x;
            res.push(x);
        }
    }
    res
}