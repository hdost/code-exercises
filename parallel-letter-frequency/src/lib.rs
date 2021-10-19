use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::pin::Pin;
pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut children = vec![];
    let map_mutex: Arc<Mutex<HashMap<char, usize>>> = Arc::new(Mutex::new(HashMap::new()));
    let count_mutex = Arc::new(Mutex::new(0));
    let arc_input = Arc::new(Mutex::new(Pin::new(&input)));

    for _ in 1..worker_count {
        let clone_map = Arc::clone(&map_mutex);
        let clone_count = Arc::clone(&count_mutex);
        let clone_input = Arc::clone(&arc_input);
        {
            children.push(thread::spawn(move || {
                loop {
                    let guard = clone_count.lock();
                    let mut output = match guard {
                        Ok(guard) => guard,
                        Err(_) => {
                            continue;
                        }
                    };
                    let current = *output;
                    *output += 1;
                    //drop(clone_count);
                    let guard_input = match clone_input.lock() {
                        Ok(guard) => guard,
                        Err(_) => {
                            continue;
                        }
                    };
                    if current >= guard_input.len() {
                        break;
                    }
                    let phrase = guard_input[current];
                    //drop(clone_input);
                    for c in phrase.chars() {
                        loop {
                            let mut map = match clone_map.lock() {
                                Ok(guard) => guard,
                                Err(_) => {
                                    continue;
                                }
                            };
                            let count = map.entry(c).or_insert(0);
                            *count += 1;
                            //drop(clone_map);
                            break;
                        }
                    }
                }
            }));
        }
    }
    for child in children {
        let _ = child.join();
    }
    let x = &*map_mutex.lock().unwrap();
    x.clone()
}
