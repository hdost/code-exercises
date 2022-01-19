use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[test]
#[ignore]
fn test_first_part_sample_data() {
    let file = File::open("test.txt");
    match file {
        Err(e) => println!("Cannot find file {}", e),
        Ok(file) => process_file(file),
    }
}

#[test]
#[ignore]
fn test_first_part_real_data() {
    let file = File::open("input.txt");
    match file {
        Err(e) => println!("Cannot find file {}", e),
        Ok(file) => process_file(file),
    }
}

#[test]
#[ignore]
fn test_second_part_sample_data() {
    let file = File::open("test.txt");
    match file {
        Err(e) => println!("Cannot find file {}", e),
        Ok(file) => process_file_2(file),
    }
}

#[test]
fn test_second_part_real_data() {
    let file = File::open("input.txt");
    match file {
        Err(e) => println!("Cannot find file {}", e),
        Ok(file) => process_file_2(file),
    }
}

fn is_open_char(c: char) -> bool {
    c == '(' || c == '[' || c == '<' || c == '{'
}

fn matched_group(open: char, close: char) -> bool {
    open == '(' && close == ')'
        || open == '[' && close == ']'
        || open == '<' && close == '>'
        || open == '{' && close == '}'
}

fn first_bad_close(input: &str) -> Option<char> {
    let mut stack = Vec::new();
    for c in input.chars() {
        if is_open_char(c) {
            stack.push(c);
        } else {
            let open = stack.pop().unwrap();
            if !matched_group(open, c) {
                return Some(c);
            }
        }
    }
    None
}

const fn good_char_score(c: char) -> i64 {
    match c {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => 0,
    }
}
const fn bad_char_score(c: Option<char>) -> i64 {
    match c {
        Some(')') => 3,
        Some(']') => 57,
        Some('}') => 1197,
        Some('>') => 25137,
        _ => 0,
    }
}

#[test]
fn test_complete_the_line() {
    assert_eq!(complete_the_line(&"<{([{{}}[<[[[<>{}]]]>[]]"), 294);
}

// This expects that a good line is being passed in.
fn complete_the_line(input: &str) -> i64 {
    let mut stack = Vec::new();
    let mut score = 0;
    for c in input.chars() {
        if is_open_char(c) {
            stack.push(c);
        } else {
            stack.pop().unwrap();
        }
    }
    while stack.len() > 0 {
        let curr = stack.pop().unwrap();
        score = score * 5;
        score += good_char_score(curr);
    }
    score
}

fn process_file(file: File) {
    let reader = BufReader::new(file);
    let lines = reader.lines();
    let mut score = 0;
    for line in lines.flatten() {
        let bad_char = first_bad_close(&line);
        score += bad_char_score(bad_char);
    }
    println!("Output: {}", score);
}

fn process_file_2(file: File) {
    let reader = BufReader::new(file);
    let lines = reader.lines();
    let mut scores = Vec::new();
    for line in lines.flatten() {
        let bad_char = first_bad_close(&line);
        if bad_char == None {
            scores.push(complete_the_line(&line));
        }
    }
    scores.sort();
    println!("Output: {}", scores.get(scores.len() / 2).unwrap());
}
