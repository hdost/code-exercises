use std::{
    collections::HashSet,
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

fn increase_all(matrix: &mut Vec<Vec<i32>>) {
    for x in matrix {
        for y in x {
            *y += 1;
        }
    }
}

fn is_valid_point(matrix: &Vec<Vec<i32>>, point: &(i32, i32)) -> bool {
    match matrix.get(point.0 as usize) {
        Some(l) => match l.get(point.1 as usize) {
            Some(n) => true,
            None => false,
        },
        None => false,
    }
}
fn str_to_num_vec(input: &str) -> Vec<i32> {
    let mut output = Vec::new();
    for c in input.chars() {
        output.push(c as i32 - 48);
    }
    output
}

fn flash_adjacent(matrix: &mut Vec<Vec<i32>>, point: (i32, i32)) {
    for x_off in -1..=1 {
        for y_off in -1..=1 {
            if x_off == 0 && y_off == 0 {
                continue;
            }
            match matrix.get_mut((point.0 + x_off) as usize) {
                Some(row) => match row.get_mut((point.1 + y_off) as usize) {
                    Some(v) => *v += 1,
                    None => {}
                },
                None => {}
            }
        }
    }
}

fn print_matrix(matrix: &Vec<Vec<i32>>) {
    for row in matrix {
        for v in row {
            print!("{}",v);
        }
        println!("");
    }
    println!("");
}
fn should_flash(matrix: &Vec<Vec<i32>>, coords: (i32, i32)) -> bool {
    *matrix
        .get(coords.0 as usize)
        .unwrap()
        .get(coords.1 as usize)
        .unwrap()
        > 9
}

fn process_flashes(matrix: &mut Vec<Vec<i32>>) -> HashSet<(i32, i32)> {
    let mut flashed = HashSet::new();
    let mut flash_occurred = true;
    while flash_occurred {
        flash_occurred = false;
        for x in 0..matrix.len() {
            let row_len = matrix.get(x).unwrap().len();
            for y in 0..row_len {
                let coords = (x as i32, y as i32);
                if !flashed.contains(&coords) && should_flash(matrix, coords) {
                    flash_adjacent(matrix, coords);
                    flashed.insert(coords);
                    flash_occurred = true;
                }
            }
        }
    }
    flashed
}

fn reset_flashed(matrix: &mut Vec<Vec<i32>>, flashed: HashSet<(i32, i32)>) {
    for f in flashed {
        match matrix.get_mut(f.0 as usize) {
            Some(row) => match row.get_mut(f.1 as usize) {
                Some(v) => *v = 0,
                None => {}
            },
            None => {}
        }
    }
}

fn process_file(file: File) {
    let reader = BufReader::new(file);
    let lines = reader.lines();
    let mut matrix = Vec::new();
    for line in lines.flatten() {
        matrix.push(str_to_num_vec(&line));
    }
    let mut total_flashes = 0;
    println!("Before Any:");
        print_matrix(&matrix);
    for step in 1..=100 {
        increase_all(&mut matrix);
        let flashed = process_flashes(&mut matrix);
        total_flashes += flashed.len();
        reset_flashed(&mut matrix, flashed);
        println!("After {}", step);
        print_matrix(&matrix);
    }

        print_matrix(&matrix);



    println!("Output: {}", total_flashes);
}

fn all_flashed(matrix: &Vec<Vec<i32>>) -> bool {
   for row in matrix {
        for v in row {
            if *v != 0 {
                return false;
            }
        }
   }
   true
}

fn process_file_2(file: File) {
    let reader = BufReader::new(file);
    let lines = reader.lines();
    let mut matrix = Vec::new();
    for line in lines.flatten() {
        matrix.push(str_to_num_vec(&line));
    }
    let mut total_flashes = 0;
    println!("Before Any:");
        print_matrix(&matrix);
    let mut steps = 0;
    while !all_flashed(&matrix) {
        steps +=1;
        increase_all(&mut matrix);
        let flashed = process_flashes(&mut matrix);
        total_flashes += flashed.len();
        reset_flashed(&mut matrix, flashed);
        println!("After {}", steps);
        print_matrix(&matrix);
    }

        print_matrix(&matrix);



    println!("Output: {}", steps);
}
