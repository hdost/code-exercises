use pcre2::bytes::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn process_file(file: File) -> i64 {
    let reader = BufReader::new(file);
    let lines = reader.lines();

    lines
        .flatten()
        .map(|line| {
            let first: i64 = line
                .chars()
                .find(|x| x.is_numeric())
                .expect("Always a number")
                .to_digit(10)
                .unwrap()
                .into();
            let last: i64 = line
                .chars()
                .rev()
                .find(|x| x.is_numeric())
                .expect("Always a number")
                .to_digit(10)
                .unwrap()
                .into();
            10 * first + last
        })
        .sum()
}

fn process_file_2(file: File) -> i64 {
    let reader = BufReader::new(file);
    let lines = reader.lines();
    lines
        .flatten()
        .map(|line| {
            let (first, last) = parse_mixed_line(line);
            println!("{},{}", first, last);
            (10 * first) + last
        })
        .sum()
}

fn parse_digit(digit: &str) -> Option<i64> {
    println!("{}", digit);
    if digit.chars().next().expect("Always something").is_numeric() {
        return digit.parse().ok();
    }

    match digit {
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        _ => None,
    }
}

fn parse_mixed_line(line: String) -> (i64, i64) {
    println!("{}", line);
    let re = Regex::new("((?=(one|two|three|four|five|six|seven|eight|nine))|[0-9])").unwrap();
    let m: Vec<i64> = re
        .find_iter(line.as_bytes())
//        .filter_map(|x| x.ok())
        .map(|x| {
            println!("Current Capture: {:?}", x);

            //println!("Capture Len: {}", x.len());
            let mut digit: &[u8] = &[];
            //for i in 0..x.len() {
            //    match x.get(i) {
            //        None => digit = &x.get(i - 1).unwrap().as_bytes(),
            //        Some(_) => continue,
            //    }
            //}
            //parse_digit(std::str::from_utf8(x.as_bytes()).unwrap()).unwrap()
                1
        })
        .collect();

    let first = m[0];
    println!("{:?}", m);
    match m.last() {
        Some(n) => (first, *n),
        None => (first, first),
    }
}

#[test]
fn test_parse_mixed_line() {
    assert_eq!(parse_mixed_line("1one1nine".to_owned()), (1, 9));
    assert_eq!(parse_mixed_line("one1".to_owned()), (1, 1));
    assert_eq!(parse_mixed_line("1nine".to_owned()), (1, 9));
    assert_eq!(parse_mixed_line("1".to_owned()), (1, 1));
    assert_eq!(parse_mixed_line("eightwo".to_owned()), (8, 2));
    assert_eq!(parse_mixed_line("one1eightnine".to_owned()), (1, 9));
    assert_eq!(parse_mixed_line("one1niasdlkasdjfne".to_owned()), (1, 1));
}

#[test]
fn test_first_part_sample_data() {
    let file = File::open("test.txt");
    match file {
        Err(e) => println!("Cannot find file {}", e),
        Ok(file) => {
            assert_eq!(142, process_file(file));
        }
    }
}

#[test]
fn test_first_part_real_data() {
    let file = File::open("input.txt");
    match file {
        Err(e) => println!("Cannot find file {}", e),
        Ok(file) => {
            assert_eq!(54644, process_file(file));
        }
    }
}

#[test]
fn test_second_part_sample_data() {
    let file = File::open("test2.txt");
    match file {
        Err(e) => println!("Cannot find file {}", e),
        Ok(file) => {
            assert_eq!(281, process_file_2(file));
        }
    }
}

#[test]
#[ignore]
fn test_second_part_real_data() {
    let file = File::open("input.txt");
    match file {
        Err(e) => println!("Cannot find file {}", e),
        Ok(file) => {
            assert_eq!(1, process_file_2(file));
        }
    }
}
