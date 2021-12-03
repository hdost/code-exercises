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
#[ignore]
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

#[test]
fn test_calculate_delta_forward() {
    let (dx,dy) = calculate_delta("forward 1");
    assert_eq!(dx, 1);
    assert_eq!(dy, 0);
}

#[test]
fn test_calculate_delta_up() {
    let (dx,dy) = calculate_delta("up 1");
    assert_eq!(dx, 0);
    assert_eq!(dy, 1);
}

#[test]
fn test_calculate_delta_down() {
    let (dx,dy) = calculate_delta("down 1");
    assert_eq!(dx, 0);
    assert_eq!(dy, -1);
}



fn calculate_delta(input:&str) -> (i32, i32) {
    let mut temp = input.split(" ");
    let operation = temp.next().unwrap();
    let number = temp.next().unwrap().parse().unwrap();
    match operation {
        "forward" => {return (number,0)},
        "up" => {return (0,-number)},
        "down" => {return (0,number)},
        _ => {return (0,0)},
    }
}


fn process_file(file: File) {
    let reader = BufReader::new(file);
    let lines = reader.lines();
    let (mut x, mut y) = (0,0);
    for line in lines {
        if let Ok(line) = line {
            let (dx,dy) = calculate_delta(&line);
            x += dx;
            y += dy;
        }
    }

    println!("Output: {}", x*y);
}

fn process_file_2(file: File) {
    let reader = BufReader::new(file);
    let lines = reader.lines();
    let (mut x, mut y, mut aim) = (0,0,0);
    for line in lines {
        if let Ok(line) = line {
            let (dx,d_aim) = calculate_delta(&line);
            x += dx;
            aim += d_aim;
            y += dx * aim;
        }
    }
    println!("Count: {}", x*y);
}
