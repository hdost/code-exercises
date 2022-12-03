use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

fn get_priority(input: char) -> i64 {
    "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .find(input)
        .unwrap() as i64
        + 1
}

fn process_file(file: File) -> i64 {
    let reader = BufReader::new(file);
    let lines = reader.lines();
    let mut total = 0;
    for line in lines.flatten() {
        let (one, two) = line.split_at(line.len() / 2);
        let mut set = HashSet::new();
        for item in one.chars() {
            set.insert(item);
        }
        for item in two.chars() {
            if set.contains(&item) {
                total += get_priority(item);
                break;
            }
        }
    }

    total
}

fn set_from_line(line: &str) -> HashSet<char> {
    let mut set = HashSet::new();
    for ch in line.chars() {
        set.insert(ch);
    }
    set
}

fn process_file_2(file: File) -> i64 {
    let reader = BufReader::new(file);
    let lines = reader.lines();
    let mut total = 0;
    let mut packs = lines.flatten().peekable();
    loop {
        let first = set_from_line(&packs.next().unwrap());
        let second = set_from_line(&packs.next().unwrap());
        let third = set_from_line(&packs.next().unwrap());

        let intersect: Vec<&char> = first
            .iter()
            .filter(|item| second.contains(item))
            .filter(|item| third.contains(item))
            .collect();
        total += get_priority(**intersect.get(0).unwrap());
        if packs.peek() == None {
            break;
        }
    }
    total
}

#[test]
fn test_first_part_sample_data() {
    let file = File::open("test.txt");
    match file {
        Err(e) => println!("Cannot find file {}", e),
        Ok(file) => {
            assert_eq!(157, process_file(file));
        }
    }
}

#[test]
fn test_first_part_real_data() {
    let file = File::open("input.txt");
    match file {
        Err(e) => println!("Cannot find file {}", e),
        Ok(file) => {
            assert_eq!(7845, process_file(file));
        }
    }
}

#[test]
fn test_second_part_sample_data() {
    let file = File::open("test.txt");
    match file {
        Err(e) => println!("Cannot find file {}", e),
        Ok(file) => {
            assert_eq!(70, process_file_2(file));
        }
    }
}

#[test]
fn test_second_part_real_data() {
    let file = File::open("input.txt");
    match file {
        Err(e) => println!("Cannot find file {}", e),
        Ok(file) => {
            assert_eq!(2790, process_file_2(file));
        }
    }
}
