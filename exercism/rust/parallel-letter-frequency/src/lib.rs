use std::collections::HashMap;
use std::thread;
pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let all_chars = input.join("");
    let divisor = match worker_count {
       0..=1 => 1,
       _ => worker_count -1
    };
    let segment_size = all_chars.len() / divisor;
    let segment_size = match segment_size{
        0 => 1,
        _ => segment_size,
    };
    print!("{} {} {}", all_chars.len(), segment_size, worker_count);
    let mut all_chars = all_chars.chars().into_iter();
    let mut children = vec![];

    for _ in 0..worker_count {

        let my_string: String = all_chars.by_ref().take(segment_size).collect();
        children.push(thread::spawn(move || {
            let mut map: HashMap<char, usize> = HashMap::new();
            for c in my_string.chars().into_iter() {
                if c.is_alphabetic() {
                let e = map.entry(c.to_ascii_lowercase()).or_insert(0);
                *e += 1;
                }
            }

            map
        }));
    }
    let mut output = HashMap::new();
    for child in children {
        let map: HashMap<char, usize> = match child.join() {
            Ok(m) => m,
            Err(_) => HashMap::new(),
        };
        for (k, v) in map.iter() {
            let curr = output.entry(*k).or_insert(0);
            *curr += v;
        }
    }
    output
}
