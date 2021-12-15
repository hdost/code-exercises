use regex::Regex;
use std::{
    borrow::BorrowMut,
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Lines},
    time::SystemTime,
};

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref PAIR_PATTERN: Regex = Regex::new(r"(?P<source>\w+) -> (?P<target>\w)").unwrap();
    static ref SEQUENCE_PATTERN: Regex = Regex::new(r"^\w+$").unwrap();
}
#[test]
fn test_first_part_sample_data() {
    let file = File::open("test.txt");
    match file {
        Err(e) => println!("Cannot find file {}", e),
        Ok(file) => {
            assert_eq!(1588, process_file(file));
        }
    }
}

#[test]
fn test_first_part_real_data() {
    let file = File::open("input.txt");
    match file {
        Err(e) => println!("Cannot find file {}", e),
        Ok(file) => {
            assert_eq!(2509, process_file(file));
        }
    }
}

#[test]
fn test_second_part_sample_data() {
    let file = File::open("test.txt");
    match file {
        Err(e) => println!("Cannot find file {}", e),
        Ok(file) => {
            assert_eq!(2188189693529, process_file_2(file));
        }
    }
}

#[test]
fn test_second_part_real_data() {
    let file = File::open("input.txt");
    match file {
        Err(e) => println!("Cannot find file {}", e),
        Ok(file) => {
            assert_eq!(1, process_file_2(file));
        }
    }
}

// Produces the sequence and source target pairs
fn process_lines(lines: Lines<BufReader<File>>) -> (String, HashMap<String, String>) {
    let mut seq = String::new();
    let mut pairs = HashMap::new();
    for line in lines.flatten() {
        if SEQUENCE_PATTERN.is_match(&line) {
            seq = line.to_string();
        }

        match PAIR_PATTERN.captures(&line) {
            Some(cap) => {
                pairs.insert(
                    cap.name("source")
                        .expect("source must be part of the pattern")
                        .as_str()
                        .to_string(),
                    cap.name("target")
                        .expect("target must be part of the pattern.")
                        .as_str()
                        .to_string(),
                );
            }
            None => {}
        }
    }
    (seq, pairs)
}

#[test]
fn test_count_chars_from_pairs() {
    let mut pairs = HashMap::new();
    pairs.insert("BN".to_string(), 1);
    pairs.insert("NN".to_string(), 1);
    pairs.insert("CB".to_string(), 1);
    let output = count_chars_from_pairs(&pairs);
    assert_eq!(Some(&1), output.get(&'C'));
    assert_eq!(Some(&3), output.get(&'N'));
    assert_eq!(Some(&2), output.get(&'B'));
}

fn count_chars_from_pairs(pairs: &HashMap<String, i64>) -> HashMap<char, i64> {
    let mut map = HashMap::new();
    for (pair, count) in pairs {
        let l = map
            .entry(pair.chars().nth(0).expect("must have first char"))
            .or_insert(0);
        *l += count;
        let r = map
            .entry(pair.chars().nth(1).expect("must have second char"))
            .or_insert(0);
        *r += count;
    }
    map
}

fn count_chars(seq: &str) -> HashMap<char, i64> {
    let mut map = HashMap::new();
    for c in seq.chars() {
        let v = map.entry(c).or_insert(0);
        *v += 1;
    }
    map
}

#[test]
fn test_get_count_range() {
    let seq = "NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB";
    let map = count_chars(&seq);
    let (min, max) = get_count_range(map);
    assert!(min < max);
    assert_eq!(max, 23);
    assert_eq!(min, 11);
}

// Returns the lowest and highest
fn get_count_range(map: HashMap<char, i64>) -> (i64, i64) {
    println!("{:?}", map);
    let min = map
        .iter()
        .min_by(|a, b| a.1.cmp(b.1))
        .map(|(_k, v)| {
            println!("Min {} {}", _k, v);
            v
        })
        .expect("Sequence isn't empty.");
    let max = map
        .iter()
        .max_by(|a, b| a.1.cmp(b.1))
        .map(|(_k, v)| {
            println!("Max {} {}", _k, v);
            v
        })
        .expect("Sequence isn't empty.");
    (*min, *max)
}

#[test]
fn test_iterate_sequence() {
    let mut map = HashMap::new();
    map.insert("NN".to_string(), "C".to_string());
    map.insert("NC".to_string(), "B".to_string());
    map.insert("CB".to_string(), "H".to_string());

    assert_eq!("NCNBCHB".to_string(), iterate_sequence(&"NNCB", &map));
}

fn iterate_sequence(seq: &str, pairs: &HashMap<String, String>) -> String {
    let mut new_seq = seq.to_string();
    let mut new_chars = Vec::new();
    for i in 0..seq.len() - 1 {
        match pairs.get(&seq[i..=i + 1].to_string()) {
            Some(t) => {
                new_chars.insert(0, (i + 1, t.clone()));
            }
            None => {}
        }
    }
    for n in new_chars {
        new_seq.insert(n.0, n.1.chars().last().unwrap());
    }

    new_seq
}

fn print_time_elapsed(start: &SystemTime) {
    match start.elapsed() {
        Ok(elapsed) => {
            println!("{}", elapsed.as_secs());
        }
        Err(e) => {
            // an error occurred!
            println!("Error: {:?}", e);
        }
    }
}

#[test]
fn test_generate_pairs() {
    let pairs = generate_pairs("NNCB");
    assert_eq!(Some(&1), pairs.get("NN"));
    assert_eq!(Some(&1), pairs.get("NC"));
    assert_eq!(Some(&1), pairs.get("CB"));
    assert_eq!(3, pairs.len());
}

fn generate_pairs(seq: &str) -> HashMap<String, i64> {
    let mut pairs = HashMap::new();
    for i in 0..seq.len() - 1 {
        let e = pairs.entry(seq[i..=i + 1].to_string()).or_insert(0);
        *e += 1;
    }
    pairs
}

#[test]
fn test_iterate_pairs() {
    let mut pairs = HashMap::new();
    pairs.insert("NN".to_string(), 1);
    pairs.insert("NC".to_string(), 1);
    pairs.insert("CB".to_string(), 1);

    let mut map = HashMap::new();
    map.insert("NN".to_string(), "C".to_string());
    map.insert("NC".to_string(), "B".to_string());
    map.insert("CB".to_string(), "H".to_string());

    let output = iterate_pairs(pairs, &map);
    assert_eq!(Some(&1), output.get("NC"));
    assert_eq!(Some(&1), output.get("CN"));
    assert_eq!(Some(&1), output.get("NB"));
}

fn iterate_pairs(
    mut input: HashMap<String, i64>,
    mapping: &HashMap<String, String>,
) -> HashMap<String, i64> {
    let mut output = HashMap::new();
    for (k, count) in input.drain() {
        match mapping.get(&k) {
            Some(target) => {
                let left_name = format!(
                    "{}{}",
                    k.chars()
                        .nth(0)
                        .expect("Should always have a first character."),
                    target
                );
                let right_name = format!(
                    "{}{}",
                    target,
                    k.chars()
                        .nth(1)
                        .expect("Should always have a second character.")
                );

                let left_entry = output.entry(left_name).or_insert(0);
                *left_entry += count;
                let right_entry = output.entry(right_name).or_insert(0);
                *right_entry += count;
            }
            None => {
                let entry = output.entry(k).or_insert(0);
                *entry += count;
            }
        }
    }
    output
}

pub fn process_file(file: File) -> i64 {
    let reader = BufReader::new(file);
    let lines = reader.lines();
    let (mut seq, pairs) = process_lines(lines);

    for i in 0..10 {
        let new_seq = iterate_sequence(&seq, &pairs);
        println!("Step {}: {}", i + 1, seq);
        if seq == new_seq {
            break;
        }
        seq = new_seq;
    }

    let map = count_chars(&seq);
    let (low, high) = get_count_range(map);
    high - low
}

pub fn process_file_2(file: File) -> i64 {
    let reader = BufReader::new(file);
    let lines = reader.lines();
    let (seq, mapping) = process_lines(lines);
    let mut pairs = generate_pairs(&seq);

    let now = SystemTime::now();
    for i in 0..40 {
        println!("Step {} Start", i + 1);
        print_time_elapsed(&now);
        pairs = iterate_pairs(pairs, &mapping);
        println!("{:?}", pairs);
        println!("Step {} End", i + 1);
        print_time_elapsed(&now);
    }
    let count_map = count_chars_from_pairs(&pairs);
    let (low, high) = get_count_range(count_map);
    high / 2 - low / 2 + 1
}
