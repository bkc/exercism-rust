use std::str::FromStr;

pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() {
        return String::from_str("").unwrap();
    }
    let mut sentances: Vec<String> = list[1..]
        .iter()
        .scan(list[0], |previous_item, &item| {
            let sentance = format!("For want of a {} the {} was lost.", previous_item, item);
            *previous_item = item;
            Some(sentance)
        })
        .collect();
    sentances.push(format!("And all for the want of a {}.", list[0]));
    sentances.join("\n")
}
