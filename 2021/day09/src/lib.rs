use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn str_to_num_vec(input: &str) -> Vec<i32> {
    let mut output = Vec::new();
    for c in input.chars() {
        output.push(c as i32 - 48);
    }
    output
}

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
#[ignore]
fn test_second_part_real_data() {
    let file = File::open("input.txt");
    match file {
        Err(e) => println!("Cannot find file {}", e),
        Ok(file) => process_file_2(file),
    }
}

fn num_lower_than_point(num: i32, vec: &Vec<Vec<i32>>, x: i32, y: i32) -> bool {
    match vec.get(x as usize) {
        Some(l) => match l.get(y as usize) {
            Some(n) => {
                if num < *n {
                    true
                } else {
                    false
                }
            }
            None => true,
        },
        None => true,
    }
}

fn is_low_point(vec: &Vec<Vec<i32>>, x: i32, y: i32) -> bool {
    let curr = *vec.get(x as usize).unwrap().get(y as usize).unwrap();
    let below = num_lower_than_point(curr, vec, x, y + 1);
    let above = num_lower_than_point(curr, vec, x, y - 1);
    let left = num_lower_than_point(curr, vec, x - 1, y);
    let right = num_lower_than_point(curr, vec, x + 1, y);
    below && above && left & right
}

fn find_low_points(vec: &Vec<Vec<i32>>) -> Vec<(i32, i32)> {
    let mut points = Vec::new();
    for (x, l) in vec.iter().enumerate() {
        for (y, _) in l.iter().enumerate() {
            if is_low_point(vec, x as i32, y as i32) {
                points.push((x as i32, y as i32));
            }
        }
    }

    points
}

fn sum_points(matrix:&Vec<Vec<i32>>,points:&Vec<(i32,i32)>) -> i32 {
    let mut sum = 0;

    for p in points {
        sum += matrix.get(p.0 as usize).unwrap().get(p.1 as usize).unwrap();
    }
    sum
}

fn process_file(file: File) {
    let reader = BufReader::new(file);
    let lines = reader.lines();
    let mut matrix = vec![];
    for line in lines.flatten() {
        matrix.push(str_to_num_vec(&line));
    }
    let low_points = find_low_points(&matrix);

    println!("Output: {}", sum_points(&matrix,&low_points) + low_points.len() as i32);
}

fn process_file_2(file: File) {
    let reader = BufReader::new(file);
    let lines = reader.lines();
    let mut matrix = vec![];
    for line in lines.flatten() {
        matrix.push(str_to_num_vec(&line));
    }
    let low_points = find_low_points(&matrix);

    println!("Output: {}", sum_points(&matrix,&low_points) + low_points.len() as i32);
}
