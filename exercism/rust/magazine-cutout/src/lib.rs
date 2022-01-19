// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut available = HashMap::new();
    for w in magazine {
        let entry = available.entry(w.to_string()).or_insert(0);
        *entry += 1;
    }
    for w in note {
        let entry = available.entry(w.to_string()).or_default();
        if (*entry > 0) {
            *entry -= 1;
        }
        // No more words in the magazine left
        else {
            return false;
        }
    }

    return true;
}
