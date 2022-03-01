use std::collections::{hash_map::Entry, HashMap};

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut magazine_map = HashMap::new();
    for word in magazine.iter() {
        *magazine_map.entry(word).or_insert(0) += 1;
    }

    for word in note.iter() {
        match magazine_map.entry(word) {
            Entry::Occupied(mut entry) => {
                *entry.get_mut() -= 1;
                if *entry.get_mut() < 0 {
                    return false;
                }
            }
            Entry::Vacant(_) => {
                return false;
            }
        }
        /* instead of the above match statement I could have used the following method, however
        I don't like it because it creates a HashMap entry for words that were not in magazine.

        let magazine_counter = magazine_map.entry(word).or_insert(0);
        *magazine_counter -= 1;
        if *magazine_counter < 0 {
            return false;
        }
        */
    }
    true
}
