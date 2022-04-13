pub fn is_armstrong_number(num: u32) -> bool {
    let text_num = num.to_string();
    let power_factor = text_num.len() as u32;
    num == text_num
        .chars()
        .map(|char| char.to_digit(10).unwrap().pow(power_factor))
        .sum()
}
