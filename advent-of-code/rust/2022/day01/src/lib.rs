use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[test]
fn test_first_part_sample_data() {
    let file = File::open("test.txt");
    match file {
        Err(e) => println!("Cannot find file {}", e),
        Ok(file) => {
            assert_eq!(24000, process_file(file));
        }
    }
}

#[test]
fn test_first_part_real_data() {
    let file = File::open("input.txt");
    match file {
        Err(e) => println!("Cannot find file {}", e),
        Ok(file) => {
            assert_eq!(75501, process_file(file));
        }
    }
}

#[test]
fn test_second_part_sample_data() {
    let file = File::open("test.txt");
    match file {
        Err(e) => println!("Cannot find file {}", e),
        Ok(file) => {
            assert_eq!(41000, process_file_2(file));
        }
    }
}

#[test]
fn test_second_part_real_data() {
    let file = File::open("input.txt");
    match file {
        Err(e) => println!("Cannot find file {}", e),
        Ok(file) => {
            assert_eq!(215594, process_file_2(file));
        }
    }
}

fn process_file(file: File) -> i64 {
    let reader = BufReader::new(file);
    let lines = reader.lines();
    let mut elves = Vec::new();
    let mut elf = 0;
    let mut max_elf = 0;
    for line in lines.flatten() {
        if let Ok(food) = line.parse::<i64>() {
            elf += food;
        } else {
            if elf > 0 {
                if elf > max_elf {
                    max_elf = elf;
                }
                elves.push(elf);
                elf = 0;
            }
        }
    }

    max_elf
}

fn process_file_2(file: File) -> i64 {
    let reader = BufReader::new(file);
    let lines = reader.lines();
    let mut elves = Vec::new();
    let mut elf = 0;
    for line in lines.flatten() {
        if let Ok(food) = line.parse::<i64>() {
            elf += food;
        } else {
            if elf > 0 {
                elves.push(elf);
                elf = 0;
            }
        }
    }
    elves.sort_by(|a, b| b.cmp(a));
    elves[..3].iter().sum()
}
