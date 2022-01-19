use std::{
    fs::File,
    io::{BufRead, BufReader},
};

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
#[ignore]
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

fn net_difference(crabs:&Vec<i32>, pos:i32) -> i32 {
    let mut fuel_spent = 0;
    for c in crabs {
        fuel_spent += (c - pos).abs()
    }
    fuel_spent
}

fn net_difference_triangle(crabs:&Vec<i32>, pos:i32) -> i32 {
    let mut fuel_spent = 0;
    for c in crabs {
        let n = (c - pos).abs();
        fuel_spent += (n * (n + 1) )/2;
    }
    fuel_spent
}

fn process_file(file: File) {
    let reader = BufReader::new(file);
    let lines = reader.lines();
    let mut crabs = Vec::new();
    for line in lines.into_iter().flatten() {
        println!("{}",line);
        for pos in line.split(',').into_iter() {
            let pos = pos.parse::<i32>().unwrap();
            crabs.push(pos);
        }
    }
    crabs.sort();
    println!("{:?}",crabs);
    let bottom = *crabs.first().unwrap();
    let top = *crabs.last().unwrap();
    let (mut best_cost,mut best_pos) = (net_difference(&crabs,bottom),bottom);
    for i in (bottom+1)..=top {
        let cost = net_difference(&crabs,i);
        if cost < best_cost {
            best_cost = cost;
            best_pos = i;
        }
    }

    println!("Best Cost: {} Best Pos: {}", best_cost, best_pos);
}


fn process_file_2(file: File) {
    let reader = BufReader::new(file);
    let lines = reader.lines();
    let mut crabs = Vec::new();
    for line in lines.into_iter().flatten() {
        println!("{}",line);
        for pos in line.split(',').into_iter() {
            let pos = pos.parse::<i32>().unwrap();
            crabs.push(pos);
        }
    }
    crabs.sort();
    println!("{:?}",crabs);
    let bottom = *crabs.first().unwrap();
    let top = *crabs.last().unwrap();
    let (mut best_cost,mut best_pos) = (net_difference_triangle(&crabs,bottom),bottom);
    for i in (bottom+1)..=top {
        let cost = net_difference_triangle(&crabs,i);
        if cost < best_cost {
            best_cost = cost;
            best_pos = i;
        }
    }

    println!("Best Cost: {} Best Pos: {}", best_cost, best_pos);
}
