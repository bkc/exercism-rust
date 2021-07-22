use std::u32::MAX;

static PRIMES: [u32; 168] = [
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97,
    101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193,
    197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281, 283, 293, 307,
    311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379, 383, 389, 397, 401, 409, 419, 421,
    431, 433, 439, 443, 449, 457, 461, 463, 467, 479, 487, 491, 499, 503, 509, 521, 523, 541, 547,
    557, 563, 569, 571, 577, 587, 593, 599, 601, 607, 613, 617, 619, 631, 641, 643, 647, 653, 659,
    661, 673, 677, 683, 691, 701, 709, 719, 727, 733, 739, 743, 751, 757, 761, 769, 773, 787, 797,
    809, 811, 821, 823, 827, 829, 839, 853, 857, 859, 863, 877, 881, 883, 887, 907, 911, 919, 929,
    937, 941, 947, 953, 967, 971, 977, 983, 991, 997,
];

pub fn nth(n: u32) -> u32 {
    let mut primecount = PRIMES.len() as u32;
    let mut lastprime = *PRIMES.last().unwrap();
    if n < primecount {
        return PRIMES[n as usize];
    }
    while primecount < n + 1 {
        for i in lastprime + 1..MAX {
            if is_prime(i) {
                primecount += 1;
                lastprime = i;
                break;
            }
        }
    }
    lastprime
}

/// return true if n is prime
fn is_prime(n: u32) -> bool {
    for v in PRIMES.iter() {
        if n % v == 0 {
            return false;
        }
    }
    let mut i: u32 = 5;
    let limit = (n as f32).sqrt() as u32;
    while i <= limit {
        if (n % i) == 0 || (n % (i + 2)) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

#[test]
fn test_is_prime_997() {
    assert!(is_prime(997));
}

#[test]
fn test_is_prime_1009() {
    assert!(is_prime(1009));
}

#[test]
fn test_not_prime_998() {
    assert!(!is_prime(998));
}
