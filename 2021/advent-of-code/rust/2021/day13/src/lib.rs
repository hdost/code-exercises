use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref PAIR_PATTERN: Regex = Regex::new(r"(?P<x>\d+),(?P<y>\d+)").unwrap();
    static ref FOLD_PATTERN: Regex = Regex::new(r"fold along (?P<axis>x|y)=(?P<loc>\d+)").unwrap();
}

#[test]
fn test_first_part_sample_data() {
    let file = File::open("test.txt");
    match file {
        Err(e) => println!("Cannot find file {}", e),
        Ok(file) => {
            assert_eq!(17, process_file(file));
        }
    }
}

#[test]
fn test_first_part_real_data() {
    let file = File::open("input.txt");
    match file {
        Err(e) => println!("Cannot find file {}", e),
        Ok(file) => {
            assert_eq!(1, process_file(file));
        }
    }
}

#[test]
fn test_second_part_sample_data() {
    let file = File::open("test.txt");
    match file {
        Err(e) => println!("Cannot find file {}", e),
        Ok(file) => {
            assert_eq!(16, process_file_2(file));
        }
    }
}

#[test]
fn test_second_part_real_data() {
    let file = File::open("input.txt");
    match file {
        Err(e) => println!("Cannot find file {}", e),
        Ok(file) => {
            assert_eq!(1, process_file_2(file));
        }
    }
}

#[test]
fn test_parse_location() {
    assert_eq!(Some((5, 6)), parse_location(&"5,6"));
    assert_eq!(None, parse_location(&"x,6"));
    assert_eq!(Some((6, 5)), parse_location(&"6,5"));
}

fn parse_location(input: &str) -> Option<(i32, i32)> {
    match PAIR_PATTERN.captures(input) {
        Some(cap) => {
            let x = cap.name("x").unwrap().as_str().parse::<i32>().unwrap();
            let y = cap.name("y").unwrap().as_str().parse::<i32>().unwrap();
            Some((x, y))
        }
        None => None,
    }
}

#[test]
fn test_parse_transformation() {
    assert_eq!(None, parse_transformation(""));
    assert_eq!(Some((Some(1), None)), parse_transformation("fold along x=1"));
    assert_eq!(Some((None, Some(1))), parse_transformation("fold along y=1"));
}

fn parse_transformation(input: &str) -> Option<(Option<i32>, Option<i32>)> {
    match FOLD_PATTERN.captures(input) {
        Some(cap) => {
            let axis = cap.name("axis").expect("Axis always exists.").as_str();
            let loc = cap
                .name("loc")
                .expect("Location always exists.")
                .as_str()
                .parse::<i32>()
                .expect("Location is always a number.");
            if axis == "x" {
                return Some((Some(loc), None));
            }
            Some((None, Some(loc)))
        }
        None => None,
    }
}

#[test]
fn test_process_fold() {
    let mut loc = vec![(6, 0)];
    process_fold(&mut loc, &(Some(5), None));
    assert_eq!(Some(&(4, 0)), loc.get(0));
}

fn process_fold(locations: &mut Vec<(i32, i32)>, fold: &(Option<i32>, Option<i32>)) {
    for curr in locations.iter_mut() {
        match fold.0 {
            Some(x) => {
                if curr.0 > x {
                    curr.0 = curr.0 - (curr.0 - x) * 2;
                }
            }
            None => {}
        }
        match fold.1 {
            Some(y) => {
                if curr.1 > y {
                    curr.1 = curr.1 - (curr.1 - y) * 2;
                }
            }
            None => {}
        }
    }
}

fn process_file(file: File) -> i64 {
    let reader = BufReader::new(file);
    let lines = reader.lines();
    let mut locations = Vec::new();
    let mut folds = Vec::new();
    for line in lines.flatten() {
        match parse_location(&line) {
            Some(loc) => {
                locations.push(loc);
            }
            None => {}
        }
        match parse_transformation(&line) {
            Some(fold) => {
                folds.push(fold);
            }
            None => {}
        }
    }

    process_fold(&mut locations, folds.get(0).unwrap());
    locations.sort();
    locations.dedup();
    locations.len() as i64
}


fn process_file_2(file: File) -> i64 {
    let reader = BufReader::new(file);
    let lines = reader.lines();
    let mut locations = Vec::new();
    let mut folds = Vec::new();
    for line in lines.flatten() {
        match parse_location(&line) {
            Some(loc) => {
                locations.push(loc);
            }
            None => {}
        }
        match parse_transformation(&line) {
            Some(fold) => {
                folds.push(fold);
            }
            None => {}
        }
    }
    for fold in folds {
        process_fold(&mut locations, &fold);
    }
    locations.sort();
    locations.dedup();
    let max_x = locations.iter().map(|x|{ x.0 }).max().unwrap();
    let max_y = locations.iter().map(|x|{ x.1 }).max().unwrap();
    for x in 0..=max_y {
        for y in 0..=max_x {
            if locations.contains(&(y,x)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
    locations.len() as i64
}
