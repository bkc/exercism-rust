pub fn reply(message: &str) -> &str {
    let trim_message = message.trim_end();
    let is_question = trim_message.ends_with('?');
    let is_yell = trim_message.chars().filter(|ch| ch.is_alphabetic()).count() > 0
        && trim_message
            .chars()
            .filter(|ch| ch.is_alphabetic())
            .all(|ch| ch.is_uppercase());
    match (is_question, is_yell, trim_message) {
        (true, true, _) => "Calm down, I know what I'm doing!",
        (false, true, _) => "Whoa, chill out!",
        (true, false, _) => "Sure.",
        (false, false, "") => "Fine. Be that way!",
        _ => "Whatever.",
    }
}
