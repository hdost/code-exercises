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
        Ok(file) => {
            assert_eq!(61229, process_file_2(file));
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
fn test_str_list_to_map() {
    let output = str_list_to_map(&"acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab");
    assert_eq!(output.get("abcdefg"), Some(&'8'));
    assert_eq!(output.get("bcdef"), Some(&'5'));
    assert_eq!(output.get("acdfg"), Some(&'2'));
}

#[test]
fn test_str_list_to_map_scrambled() {
    let output = str_list_to_map(&"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb");
    assert_eq!(output.get("bcdef"), Some(&'3'));
    assert_eq!(output.get("bcdefg"), Some(&'9'));
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

fn get_char_for_freq(map: &HashMap<char, i32>, freq: i32) -> char {
    for (k, v) in map {
        if *v == freq {
            return *k;
        }
    }
    ' ' // This is super bad rust but this should never return in my
        // particular program. This should really be an optional output
}

fn get_seq_for_len(input: &str, len: usize) -> Option<String> {
    for a in input.split_whitespace() {
        if a.len() == len {
            return Some(sorted_string(a));
        }
    }
    None
}

fn sorted_string(input: &str) -> String {
    let mut chars: Vec<char> = input.chars().collect();
    chars.sort_by(|a, b| a.cmp(b));
    String::from_iter(chars)
}

//
//  aaaa
// b    c
// b    c
//  dddd
// e    f
// e    f
//  gggg
//
//
// 1 = 2 segments
// 4 = 4 segments
// 7 = 3 segments
// 8 = 7 segments
//    8687497
//    abcdefg
//  1 0010010
//  4 0111010
//  7 1010010
//
//  8 1111111
// 5 Seg
//  2 1011101  // Only one without f
//  3 1011011
//  5 1101011
// 6 Seg
//  0 1110111
//  9 1111011
//  6 1101111

fn str_list_to_map(input: &str) -> HashMap<String, char> {
    let mut output = HashMap::new();
    // Get frequency count of each segment
    let freq_map = get_segment_frequencies(input);
    // Seg Freq 6 = "b"
    let b = get_char_for_freq(&freq_map, 6);
    // Seg Freq 4 = "e"
    let e = get_char_for_freq(&freq_map, 4);
    // Seg Freq 9 = "f"
    let f = get_char_for_freq(&freq_map, 9);
    // Get length of each to determine sequence for:
    output.insert(get_seq_for_len(input, 2).unwrap(), '1');
    output.insert(get_seq_for_len(input, 4).unwrap(), '4');
    output.insert(get_seq_for_len(input, 3).unwrap(), '7');
    output.insert(get_seq_for_len(input, 7).unwrap(), '8');
    let mut num_3 = String::new();
    let mut num_5 = String::new();
    let mut len_6 = Vec::new();
    for group in input.split_whitespace() {
        if !group.contains(f) {
            output.insert(sorted_string(group), '2');
        } else {
            if group.len() == 5 {
                if group.contains(b) {
                    num_5 = sorted_string(group);
                    output.insert(num_5.clone(), '5');
                } else {
                    num_3 = sorted_string(group);
                    output.insert(num_3.clone(), '3');
                }
            } else if group.len() == 6 {
                if group.contains(e) {
                    len_6.push(group);
                } else {
                    output.insert(sorted_string(group), '9');
                }
            }
        }
    }
    // Find 3
    while len_6.len() > 0 {
        let group = len_6.pop().expect("Length of vector is greater than 0");
        output.insert(
            sorted_string(group),
            |num_5: &str, num_3: &str| -> char {
                for c in num_5.chars() {
                    if !group.contains(c) {
                        return '0';
                    }
                }
                for c in num_3.chars() {
                    if !group.contains(c) {
                        return '6';
                    }
                }
                ' '
            }(&num_5, &num_3),
        );
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
        let input = parts.next().expect("There should ");
        let output = parts.next().unwrap().split_whitespace();
        let map = str_list_to_map(input);
        let mut num_str = String::new();
        for digit in output{
            num_str.push(*map.get(&sorted_string(digit)).unwrap());
        }
        let curr_num = num_str.parse::<i32>().unwrap();
        sum += curr_num;
    }

    sum
}
