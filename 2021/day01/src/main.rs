use std::{
    fs::File,
    io::{BufRead, BufReader},
};
fn main() {
    let file = File::open("input.txt");
    match file {
        Err(e) => println!("Cannot find file {}", e),
        Ok(file) => process_file_2(file),
    }
}

#[test]
fn test_first_part_sample_data(){
    let file = File::open("test.txt");
    match file {
        Err(e) => println!("Cannot find file {}", e),
        Ok(file) => process_file(file),
    }
}

#[test]
fn test_first_part_real_data(){
    let file = File::open("input.txt");
    match file {
        Err(e) => println!("Cannot find file {}", e),
        Ok(file) => process_file(file),
    }

}

#[test]
fn test_second_part_sample_data(){
    let file = File::open("test.txt");
    match file {
        Err(e) => println!("Cannot find file {}", e),
        Ok(file) => process_file_2(file),
    }
}

#[test]
fn test_second_part_real_data(){
    let file = File::open("input.txt");
    match file {
        Err(e) => println!("Cannot find file {}", e),
        Ok(file) => process_file_2(file),
    }

}

fn process_file(file: File) {
    let mut count = 0;
    let mut previous: Option<i64> = None;
    let reader = BufReader::new(file);
    let lines = reader.lines();
    for line in lines {
        if let Ok(line) = line {
            if let Ok(current) = line.parse() {
                if let Some(previous) = previous {
                    if previous < current {
                        count += 1;
                    }
                }
                previous = Some(current);
            }
        }
    }
    println!("Count: {}", count);
}

fn process_file_2(file: File) {
    let mut buf_a: Vec<i64> = Vec::new();
    let mut count = 0;
    let reader = BufReader::new(file);
    let lines = reader.lines();
    for line in lines {
        if let Ok(line) = line {
            if let Ok(current) = line.parse() {
                // Push on vecB

                buf_a.insert(0, current);
                // If vecA > 3 pop off the taila
                if buf_a.len() > 4 {
                    buf_a.pop();
                }
                // process both vectors
                if buf_a.len() == 4 {
                    let sum_a:i64 = *&buf_a[1..4].iter().sum();
                    let sum_b:i64 = *&buf_a[0..3].iter().sum();
// If both vectors return values then compare
                    if sum_a < sum_b {
                        count += 1;
                        // If vecB sum > vecA sum increment count
                    }
                }

            }
        }
    }
    println!("Count: {}", count);
}
