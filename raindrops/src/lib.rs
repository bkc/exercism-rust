pub fn raindrops(n: u32) -> String {
    let mut parts = Vec::new();
    if n % 3 == 0 {
        parts.push("Pling");
    }
    if n % 5 == 0 {
        parts.push("Plang");
    }
    if n % 7 == 0 {
        parts.push("Plong");
    }
    if parts.is_empty() {
        return n.to_string();
    }
    parts.join("")
}

pub fn raindrops2(n: u32) -> String {
    let nums = [3, 5, 7];
    let sounds = ["Pling", "Plang", "Plong"];
    let zipped = nums.iter().zip(sounds.iter());
    let mut res = String::new();
    for (num, sound) in zipped {
        if n % num == 0 {
            res.push_str(sound)
        }
    }
    if res != "" {
        return res;
    } else {
        return n.to_string();
    }
}
