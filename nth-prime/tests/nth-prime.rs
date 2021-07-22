use nth_prime as np;

#[test]
fn test_first_prime() {
    assert_eq!(np::nth(0), 2);
}

#[test]
#[ignore]
fn test_second_prime() {
    assert_eq!(np::nth(1), 3);
}

#[test]
#[ignore]
fn test_sixth_prime() {
    assert_eq!(np::nth(5), 13);
}

#[test]
#[ignore]
fn test_167_prime() {
    assert_eq!(np::nth(167), 997);
}

#[test]
#[ignore]
fn test_168_prime() {
    assert_eq!(np::nth(168), 1_009);
}

#[test]
#[ignore]
fn test_big_prime() {
    assert_eq!(np::nth(10_000), 104_743);
}

#[test]
#[ignore]
fn test_36_000_prime() {
    assert_eq!(np::nth(36_000), 427_993);
}
