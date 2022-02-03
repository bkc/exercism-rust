pub fn verse(n: u32) -> String {
    fn bottle_and_target(n: u32) -> (String, String) {
        match n {
            1 => ("bottle".to_string(), "it".to_string()),
            _ => ("bottles".to_string(), "one".to_string()),
        }
    }

    let (this_bottle, this_target) = bottle_and_target(n);
    let (next_bottle, _next_target) = match n {
        0 => ("none".to_string(), "none".to_string()),
        1 => ("none".to_string(), "none".to_string()),
        _ => bottle_and_target(n - 1),
    };

    match n {
        1 => "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n".to_string(),
        0 => "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".to_string(),
        _ => format!("{} {} of beer on the wall, {} {} of beer.\nTake {} down and pass it around, {} {} of beer on the wall.\n", 
        n, this_bottle, n, this_bottle, this_target, n-1, next_bottle)
    }
}

pub fn sing(start: u32, end: u32) -> String {
    let mut result = Vec::<String>::new();
    for n in (end..=start).rev() {
        result.push(verse(n));
    }
    result.join("\n")
}
