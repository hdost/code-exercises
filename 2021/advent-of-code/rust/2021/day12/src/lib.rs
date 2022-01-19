use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

#[test]
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

fn parse_line(line: &str) -> (String, String) {
    let mut parts = line.split('-');
    let a = parts.next().expect("Only valid lines please");
    let b = parts.next().expect("Only valid lines please");
    (String::from(a), String::from(b))
}

fn add_edge(map: &mut HashMap<String, HashSet<String>>, edge: (String, String)) {
    let start = map.entry(edge.0.clone()).or_insert(HashSet::new());
    start.insert(edge.1.clone());
    let end = map.entry(edge.1).or_insert(HashSet::new());
    end.insert(edge.0);
}

#[test]
fn test_is_valid_node_with_start() {
    assert_eq!(false,is_valid_node(&vec![String::from("start")],"start",1));
}

#[test]
fn test_is_valid_node_small_cave() {
    assert_eq!(true,is_valid_node(&vec![String::from("start"),String::from("A")],"b",1));
}

#[test]
fn test_is_valid_node_small_cave_visited() {
    assert_eq!(false,is_valid_node(&vec![String::from("start"),String::from("b"),String::from("A")],"b",1));
}

#[test]
fn test_is_valid_node_small_cave_visited_multiple() {
    assert_eq!(true,is_valid_node(&vec![String::from("start"),String::from("b"),String::from("A")],"b",2));
    assert_eq!(false,is_valid_node(&vec![String::from("start"),String::from("b"),String::from("A"),String::from("b"),String::from("A")],"b",2));
}

#[test]
fn test_is_valid_node_big_cave() {
    assert_eq!(true,is_valid_node(&vec![String::from("start")],"A",1));
}
#[test]
fn test_is_valid_node_big_cave_visited() {
    assert_eq!(true,is_valid_node(&vec![String::from("start"),String::from("A"),String::from("b")],"A",1));
}

fn other_small_caves(visited: &Vec<String>,small_visits:usize) -> bool {
    let mut output = true;
    if small_visits > 1 {
        for v in visited {
            // Is a small cave
            if v.to_lowercase() == *v {
                output = output && visited.iter().filter(|x| **x== *v).count() < small_visits
            }
        }
    }
    output
}
fn is_valid_node(visited: &Vec<String>, node: &str,small_visits:usize) -> bool {
    node != "start" && (node == node.to_uppercase() || visited.iter().filter(|x| **x== String::from(node)).count() == 0 || other_small_caves(visited,small_visits))

}

fn find_path(
    map: &HashMap<String, HashSet<String>>,
    visited: Vec<String>,small_visits:usize) -> HashSet<Vec<String>> {
    println!("{:?}",visited);
    // Get the last node in the list.
    let node = visited.last().unwrap();
    let mut output = HashSet::new();
    // If: last node is "end"
    if node == "end" {
        output.insert(visited);
        return output;
    }
    // Else:
    else {
    //   Get options of all paths available
        let nodes = map.get(node).expect("If a node has been visited then it should exist in the map.");
        for next in nodes {
            if is_valid_node(&visited,&next,small_visits) {
                let mut cloned = visited.clone();
                cloned.push(next.to_string());
                let mut paths = find_path(map,cloned,small_visits);
                for p in paths {
                    output.insert(p);
                }
            }
        }
    //
    }

    output
}

fn print_edges(map:&HashMap<String, HashSet<String>>) {
    for (k,v) in map {
        println!("{} {:?}",k,v);
    }
}

pub fn process_file(file: File) {
    let reader = BufReader::new(file);
    let lines = reader.lines();
    let mut map = HashMap::new();
    for line in lines.flatten() {
        let (a, b) = parse_line(&line);
        add_edge(&mut map, (a, b));
    }
    print_edges(&map);
    let paths = find_path(&map, vec![String::from("start")],1);
    println!("Output: {}", paths.len());
}

pub fn process_file_2(file: File) {
    let reader = BufReader::new(file);
    let lines = reader.lines();
    let mut map = HashMap::new();
    for line in lines.flatten() {
        let (a, b) = parse_line(&line);
        add_edge(&mut map, (a, b));
    }
    print_edges(&map);
    let paths = find_path(&map, vec![String::from("start")],2);
    println!("Done!");
    for n in &paths {
        println!("{:?}",n);
    }
    println!("Output: {}", paths.len());}
