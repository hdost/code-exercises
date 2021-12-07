use std::{
    fs::File,
    io::{BufRead, BufReader}, collections::HashMap,
};

#[test]
#[ignore]
fn test_first_part_sample_data() {
    let file = File::open("test.txt");
    match file {
        Err(e) => println!("Cannot find file {}", e),
        Ok(file) => {
            let a = process_file(file);
            assert_eq!(a, 5934);
        }
    }
}

#[test]
#[ignore]
fn test_first_part_real_data() {
    let file = File::open("input.txt");
    match file {
        Err(e) => println!("Cannot find file {}", e),
        Ok(file) => {
            let a = process_file(file);
            assert_eq!(a, 5934);
        }
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

fn iterate_day(fish_pop: &mut Vec<i32>) {
    let mut new_fish = 0;

    for f in fish_pop.iter_mut() {
        if *f == 0 {
            *f = 6;
            new_fish += 1;
        } else {
            *f -= 1;
        }
    }
    for _ in 1..=new_fish {
        fish_pop.push(8);
    }
    //println!("{:?}", fish_pop);
}

fn iterate_day_map(fish_pop: HashMap<i64,i64>) -> HashMap<i64,i64>{
    let mut out = HashMap::new();
    for (k,v) in fish_pop {
        if k == 0 {
            let e = out.entry(6).or_insert(0);
            *e += v;
            out.insert(8, v);
        }
        else {
            let e = out.entry(k-1).or_insert(0);
            *e += v;
        }
    }
    out
}

fn process_file(file: File) -> i32 {
    let reader = BufReader::new(file);
    let lines = reader.lines();
    let mut fish_pop = Vec::new();
    for line in lines.flatten() {
        for f in line.split(',').into_iter() {
            fish_pop.push(f.parse::<i32>().unwrap());
        }
    }
    //println!("Initial State:");
    //println!("{:?}", fish_pop);
    for d in 1..=80 {
        //println!("Day: {}", d);
        iterate_day(&mut fish_pop);
    }
    println!("Output: {:?}", fish_pop);
    fish_pop.len() as i32
}

fn process_file_2(file: File) {
    let reader = BufReader::new(file);
    let lines = reader.lines();
    let mut fish_map = HashMap::new();
    for line in lines.flatten() {
        for f in line.split(',').into_iter() {
            let e = fish_map.entry(f.parse::<i64>().unwrap()).or_insert(0);
            *e +=1;
        }
    }
    println!("Initial State:");
    println!("{:?}", fish_map);
    for d in 1..=256 {
        println!("Day: {}", d);
        fish_map = iterate_day_map(fish_map);
    }
    println!("Output: {:?}", fish_map);
    let mut sum:i64 = 0;
    for (_,v) in fish_map {
        sum += v;
    };
    println!("Output: {:?}", sum);
}
