use std::{
    fs::File,
    io::{BufRead, BufReader}, collections::HashSet,
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

fn is_valid_point(matrix:&Vec<Vec<i32>>,point:&(i32,i32)) -> bool {
     match matrix.get(point.0 as usize) {
        Some(l) => match l.get(point.1 as usize) {
            Some(n) => {
                true
            }
            None => false,
        },
        None => false,
    }
}

fn adjacent_basin(matrix:&Vec<Vec<i32>>,visited:&mut HashSet<(i32,i32)>,point:(i32,i32)) -> i32 {
   if !visited.contains(&point) {
        if is_valid_point(matrix,&point) {
            visited.insert(*&point);
            let curr = match matrix.get(point.0 as usize) {
                Some(v) => {
                    match v.get(point.1 as usize) {
                        Some(v) => v,
                        None => panic!("No Value!!"),
                    }
                },
                None => panic!("No Value!!"),
            };

            if *curr < 9 {
                let left = adjacent_basin(matrix, visited, (point.0 - 1,point.1));
                let right = adjacent_basin(matrix, visited, (point.0 +1,point.1));
                let up = adjacent_basin(matrix, visited, (point.0 ,point.1 -1));
                let down = adjacent_basin(matrix, visited, (point.0 ,point.1 + 1));
                return left + right + up + down + 1;
            }

        }
   }
   0
}

fn basin_size(matrix:&Vec<Vec<i32>>,visited:&mut HashSet<(i32,i32)>,point:(i32,i32)) -> i32 {
    adjacent_basin(matrix, visited, point)
}

fn process_file_2(file: File) {
    let reader = BufReader::new(file);
    let lines = reader.lines();
    let mut matrix = vec![];
    for line in lines.flatten() {
        matrix.push(str_to_num_vec(&line));
    }
    let low_points = find_low_points(&matrix);
    let mut product = 1;
    let mut visited = HashSet::new();
    let mut basins = Vec::new();
    for point in low_points.iter() {
        let size = basin_size(&matrix,&mut visited,*point);
        basins.push(size);
    }
    basins.sort();
    for n in basins.iter().rev().take(3){
        product *= *n;
    }
    println!("Output: {}", product);
}
