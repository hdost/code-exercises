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
#[ignore]
fn test_second_part_sample_data() {
    let file = File::open("test.txt");
    match file {
        Err(e) => println!("Cannot find file {}", e),
        Ok(file) => {
            assert_eq!(61229, process_file_2(file));
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

#[test]
fn test_str_list_to_map() {
    let output = str_list_to_map(&"acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab");
    assert_eq!(output.get("abcdefg"), Some(&8));
    assert_eq!(output.get("bcdef"), Some(&5));
    assert_eq!(output.get("acdfg"), Some(&2));
}

#[test]
fn test_str_list_to_map_scrambled() {
    let output = str_list_to_map(&"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb");
    assert_eq!(output.get("bcdef"), Some(&3));
    assert_eq!(output.get("bcdefg"), Some(&9));
}

#[test]
fn test_get_segment_frequencies() {
    let output = get_segment_frequencies(&"ab abc abd bd");
    assert_eq!(output.get(&'a'), Some(&3));
    assert_eq!(output.get(&'b'), Some(&4));
    assert_eq!(output.get(&'c'), Some(&1));
    assert_eq!(output.get(&'d'), Some(&2));
}

fn get_segment_frequencies(input: &str) -> HashMap<char, i32> {
    let mut map = HashMap::new();
    for group in input.split_whitespace() {
        for a in group.chars() {
            let e = map.entry(a).or_insert(0);
            *e += 1;
        }
    }

    map
}

fn get_char_for_freq(map:&HashMap<char, i32>, freq: i32) -> char{
    for (k,v) in map {
        if *v == freq {
            return *k
        }
    }
    ' ' // This is super bad rust but this should never return in my
        // particular program. This should really be an optional output
}

fn get_seq_for_len(input: &str, len:usize) -> Option<String> {
    for a in input.split_whitespace() {
        if a.len() == len {
            return  Some(sorted_string(a));
        }
    }
    None
}

fn sorted_string(input: &str) -> String {
    let mut chars: Vec<char> = input.chars().collect();
    chars.sort_by(|a, b| b.cmp(a));
    String::from_iter(chars)
}

//
//  dddd
// e    a
// e    a
//  ffff
// g    b
// g    b
//  cccc
//
//
// 1 = 2 segments
// 4 = 4 segments
// 7 = 3 segments
// 8 = 7 segments
//    0958575
//    abcdefg
//  1 1100000
//  4 1100110
//  7 1101000  // has unique "d"
//
//  8 1111111
//
//  0 1111101
//  9 1111110
//  6 1101111

//  2 1011011 // only one without "b"
//  3 1111010
//  5 1101011
fn str_list_to_map(input: &str) -> HashMap<String, i32> {
    let mut output = HashMap::new();
    // Get frequency count of each segment
    let freq_map = get_segment_frequencies(input);
    // Seg Freq 10 = "a"
    let a = get_char_for_freq(&freq_map,10);
    // Seg Freq 9 = "b"
    let b = get_char_for_freq(&freq_map,9);
    // Seg Freq 8 = "d"
    let d = get_char_for_freq(&freq_map,8);
    // Seg Freq 7 = "f"
    let f = get_char_for_freq(&freq_map,7);
    // Get length of each to determine sequence for:
    output.insert(get_seq_for_len(input, 2).unwrap(), 1);
    output.insert(get_seq_for_len(input, 4).unwrap(), 4);
    output.insert(get_seq_for_len(input, 3).unwrap(), 7);
    output.insert(get_seq_for_len(input, 7).unwrap(), 8);

    for group in input.split_whitespace() {
        if !group.contains(b) {
            output.insert(sorted_string(group), 2);
            break;
        }
    }

    output
}

fn process_file(file: File) {
    let reader = BufReader::new(file);
    let lines = reader.lines();
    let mut count = 0;
    for line in lines.flatten() {
        let output = line.split('|').last().unwrap();
        for num in output.split_whitespace() {
            let len = num.len();
            if len == 2 || len == 4 || len == 3 || len == 7 {
                count += 1;
            }
        }
    }
    println!("Output: {}", count);
}

fn process_file_2(file: File) -> i32 {
    let reader = BufReader::new(file);
    let lines = reader.lines();
    let mut sum = 0;
    for line in lines.flatten() {
        let mut parts = line.split('|');
        let input = parts.next().unwrap();
        let output = parts.next().unwrap().split_whitespace();
        let map = str_list_to_map(input);
    }

    sum
}
