pub fn factors(n: u64) -> Vec<u64> {
    let mut remaining = n;
    let mut prime_factors: Vec<u64> = Vec::new();
    let mut divisor: u64 = 2;

    while divisor <= n && remaining > 1 {
        if remaining % divisor != 0 {
            divisor += 1;
            continue;
        }
        prime_factors.push(divisor);
        remaining /= divisor;
    }
    prime_factors
}
