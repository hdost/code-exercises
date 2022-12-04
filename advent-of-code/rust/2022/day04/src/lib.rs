use std::{
    fs::File,
    io::{BufRead, BufReader},
};
struct Assignment {
    start: i64,
    end: i64,
}
impl Assignment {
    fn from(src:&str) -> Option<Assignment> {
        let (start,end) = src.split_once("-")?;
        let start = start.parse().ok()?;
        let end = end.parse().ok()?;
        Some(Assignment{start,end})
    }
    /// This methods returns true if either assignment contains the other
    fn contains_eachother(&self, other:&Assignment) -> bool {
        self.contains(other) || other.contains(self)

    }

    fn contains(&self, other:&Assignment) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    /// When
    fn overlap(&self, other:&Assignment) -> bool {
        !(self.start > other.end || other.start > self.end)
    }
}

fn process_file(file: File) -> i64 {
    let reader = BufReader::new(file);
    let lines = reader.lines();
    let mut total = 0;
    for line in lines.flatten() {
        if let Some((left,right)) = line.split_once(",") {
            if let (Some(left),Some(right)) = (Assignment::from(left), Assignment::from(right)) {
                if left.contains_eachother(&right) {
                    total+=1;
                }
            }

        }
    }

    total
}

fn process_file_2(file: File) -> i64 {
    let reader = BufReader::new(file);
    let lines = reader.lines();
    let mut total = 0;
    for line in lines.flatten() {
        if let Some((left,right)) = line.split_once(",") {
            if let (Some(left),Some(right)) = (Assignment::from(left), Assignment::from(right)) {
                if left.overlap(&right) {
                    total+=1;
                }
            }

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
            assert_eq!(2, process_file(file));
        }
    }
}

#[test]
fn test_first_part_real_data() {
    let file = File::open("input.txt");
    match file {
        Err(e) => println!("Cannot find file {}", e),
        Ok(file) => {
            assert_eq!(490, process_file(file));
        }
    }
}

#[test]
fn test_second_part_sample_data() {
    let file = File::open("test.txt");
    match file {
        Err(e) => println!("Cannot find file {}", e),
        Ok(file) => {
            assert_eq!(4, process_file_2(file));
        }
    }
}

#[test]
fn test_second_part_real_data() {
    let file = File::open("input.txt");
    match file {
        Err(e) => println!("Cannot find file {}", e),
        Ok(file) => {
            assert_eq!(921, process_file_2(file));
        }
    }
}
