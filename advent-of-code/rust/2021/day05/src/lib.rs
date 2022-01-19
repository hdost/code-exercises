use regex::Regex;
use std::{
    collections::HashMap,
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

#[derive(Debug, Hash, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

struct Line(Point, Point);

impl Line {
    fn is_perpendicular(&self) -> bool {
        self.0.x == self.1.x || self.0.y == self.1.y
    }
    fn is_diagnal(&self) -> bool {
        (self.0.x - self.1.x).abs() == (self.0.y - self.1.y).abs()
    }
}

#[test]
fn test_parse_line() {
    let l = parse_line(&"0,9 -> 5,9");
    assert_eq!(l.0.x, 0);
    assert_eq!(l.0.y, 9);
    assert_eq!(l.1.x, 5);
    assert_eq!(l.1.y, 9);
}
/// Parses lines like
/// 0,9 -> 5,9
fn parse_line(input: &str) -> Line {
    let pat = Regex::new(r"(?P<x1>\d+),(?P<y1>\d+) -> (?P<x2>\d+),(?P<y2>\d+)").unwrap();
    let caps = pat.captures(input).unwrap();
    let x = caps["x1"].parse::<i32>().unwrap();
    let y = caps["y1"].parse::<i32>().unwrap();
    let p1 = Point { x, y };
    let x = caps["x2"].parse::<i32>().unwrap();
    let y = caps["y2"].parse::<i32>().unwrap();
    let p2 = Point { x, y };
    Line(p1, p2)
}

#[test]
fn test_fill_map() {
    let l = Line(Point { x: 0, y: 9 }, Point { x: 5, y: 9 });
    let mut map = HashMap::new();
    fill_map(&mut map, &l);
    assert_eq!(map.get(&Point { x: 4, y: 9 }), Some(&1));
}

fn sort_pair(a: i32, b: i32) -> (i32, i32) {
    if a > b {
        (b, a)
    } else {
        (a, b)
    }
}

#[test]
fn test_fill_map_diagnal() {
    let l = Line(Point { x: 0, y: 0 }, Point { x: 9, y: 9 });
    let mut map = HashMap::new();
    fill_map_diagnal(&mut map, &l);
    print_map(&map, 9, 9);
    assert_eq!(map.get(&Point { x: 4, y: 4 }), Some(&1));
}

fn fill_map_diagnal(map: &mut HashMap<Point, i32>, line: &Line) {
    let (low, high) = if line.0.x < line.1.x {
        (&line.0, &line.1)
    } else {
        (&line.1, &line.0)
    };
    let y_pos = if high.y - low.y > 0 { 1 } else { -1 };
    for offset in 0..=high.x - low.x {
        let entry = map
            .entry(Point {
                x: low.x + offset,
                y: low.y + offset * y_pos,
            })
            .or_insert(0);
        *entry += 1;
    }
}
fn fill_map(map: &mut HashMap<Point, i32>, line: &Line) {
    let (x1, x2) = sort_pair(line.0.x, line.1.x);
    let (y1, y2) = sort_pair(line.0.y, line.1.y);
    for x in x1..=x2 {
        for y in y1..=y2 {
            let entry = map.entry(Point { x, y }).or_insert(0);
            *entry += 1;
        }
    }
}

fn count_intersections(map: &HashMap<Point, i32>) -> i32 {
    let mut count = 0;

    for e in map.values() {
        if *e > 1 {
            count += 1;
        }
    }
    count
}

fn print_map(map: &HashMap<Point, i32>, max_x: i32, max_y: i32) {
    for y in 0..=max_x {
        for x in 0..=max_y {
            let v = match map.get(&Point { x, y }) {
                Some(x) => *x,
                None => 0,
            };
            print!("{}\t", v);
        }
        println!("");
    }
}

const fn max(f: i32, s: i32, t: i32) -> i32 {
    if f > s && f > t {
        f
    } else if s > t {
        s
    } else {
        t
    }
}

fn process_file(file: File) {
    let reader = BufReader::new(file);
    let lines = reader.lines();
    let mut map = HashMap::new();
    let (mut max_x, mut max_y) = (0, 0);
    for line in lines.flatten() {
        let line = parse_line(&line);
        if line.is_perpendicular() {
            max_x = max(max_x, line.0.x, line.1.x);
            max_y = max(max_y, line.0.y, line.1.y);
            fill_map(&mut map, &line);
        }
    }
    print_map(&map, max_x, max_y);
    println!("Output: {}", count_intersections(&map));
}

fn process_file_2(file: File) {
    let reader = BufReader::new(file);
    let lines = reader.lines();
    let mut map = HashMap::new();
    let (mut max_x, mut max_y) = (0, 0);
    for line in lines.flatten() {
        let line = parse_line(&line);
        max_x = max(max_x, line.0.x, line.1.x);
        max_y = max(max_y, line.0.y, line.1.y);
        if line.is_perpendicular() {
            fill_map(&mut map, &line);
        } else if line.is_diagnal() {
            fill_map_diagnal(&mut map, &line);
        }
    }
    println!("Output: {}", count_intersections(&map));
}
