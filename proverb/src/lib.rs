use std::str::FromStr;

pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() {
        return String::from_str("").unwrap();
    }
    let mut previous_item = list[0];
    let mut result: Vec<String> = list[1..]
        .iter()
        .map(|&item| {
            let result = format!("For want of a {} the {} was lost.", previous_item, item);
            previous_item = item;
            result
        })
        .collect();
    result.push(format!("And all for the want of a {}.", list[0]));
    result.join("\n")
}
