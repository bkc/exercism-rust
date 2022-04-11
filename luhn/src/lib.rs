/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let mut is_valid = true;
    let mut digit_count: i32 = 0;
    // turns out I should have used try_fold here instead of
    // these two function level vars
    let total: u32 = code
        .chars()
        .rev()
        .filter(|&ch| ch != ' ')
        .enumerate()
        .map(|(idx, ch)| {
            if let Some(val) = ch.to_digit(10) {
                digit_count += 1;
                if (idx % 2) == 0 {
                    val
                } else if val >= 5 {
                    val * 2 - 9
                } else {
                    val * 2
                }
            } else {
                is_valid = false;
                0
            }
        })
        .sum();

    (total % 10) == 0 && is_valid && digit_count > 1
}
