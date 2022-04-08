use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut result: HashSet<&str> = HashSet::new();
    let lower_word = word.to_lowercase();
    let mut sorted_word: Vec<char> = lower_word.chars().collect();
    sorted_word.sort_unstable();

    for possible_anagram in possible_anagrams.iter() {
        let lower_possible_anagram = possible_anagram.to_lowercase();
        let mut sorted_possible_anagram: Vec<char> = lower_possible_anagram.chars().collect();
        sorted_possible_anagram.sort_unstable();
        if sorted_possible_anagram == sorted_word && lower_possible_anagram != lower_word {
            result.insert(possible_anagram);
        }
    }

    result
}
