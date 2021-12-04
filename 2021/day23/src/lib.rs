use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[test]
fn test_first_part_sample_data(){
    let file = File::open("test.txt");
    match file {
        Err(e) => println!("Cannot find file {}", e),
        Ok(file) => process_file(file),
    }
}

#[test]
#[ignore]
fn test_first_part_real_data(){
    let file = File::open("input.txt");
    match file {
        Err(e) => println!("Cannot find file {}", e),
        Ok(file) => process_file(file),
    }

}

#[test]
#[ignore]
fn test_second_part_sample_data(){
    let file = File::open("test.txt");
    match file {
        Err(e) => println!("Cannot find file {}", e),
        Ok(file) => process_file_2(file),
    }
}

#[test]
#[ignore]
fn test_second_part_real_data(){
    let file = File::open("input.txt");
    match file {
        Err(e) => println!("Cannot find file {}", e),
        Ok(file) => process_file_2(file),
    }

}

fn process_file(file: File) {
    let reader = BufReader::new(file);
    let lines = reader.lines();
    let mut sums = Vec::new();
    let mut max = 0;
    for (i,line) in lines.enumerate() {
        if let Ok(line) = line {
        }
    }
    println!("Output: {}", gamma * epsilon);
}


fn process_file_2(file: File) {
    let reader = BufReader::new(file);
    let lines = reader.lines();
    for (i,line) in lines.enumerate() {
        if let Ok(line) = line {
        }
    }

    println!("Output: {}", o2 * co2);
}
