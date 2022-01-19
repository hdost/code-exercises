use std::{
    fs::File,
    io::{BufRead, BufReader},
};
fn main() {
    let file = File::open("input.txt");
    match file {
        Err(e) => println!("Cannot find file {}", e),
        Ok(file) => process_file_2(file),
    }
}

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

//#[test]
//fn test_update_sums() {
//
//    let (dx,dy) = calculate_delta("forward 1");
//    assert_eq!(dx, 1);
//    assert_eq!(dy, 0);
//}


fn update_sums(sum_vec:&mut Vec<i32>, input:&str){
    if sum_vec.len() < input.len() {
        sum_vec.resize_with(input.len(),Default::default);
    }
    for (i,c) in input.chars().enumerate(){
        let curr: &mut i32 = sum_vec.get_mut(i).unwrap();
        *curr += c as i32 - 48;
    }
}


fn process_file(file: File) {
    let reader = BufReader::new(file);
    let lines = reader.lines();
    let mut sums = Vec::new();
    let mut max = 0;
    for (i,line) in lines.enumerate() {
        if let Ok(line) = line {
            update_sums(&mut sums,&line);
            max = i;
        }
    }
    let mut gamma = String::new();
    let mut epsilon = String::new();
    for d in sums {
        println!("{}",d);
        if d > ((max / 2) as i32).try_into().unwrap() {
            gamma.push('1');
            epsilon.push('0');
        }
        else {
            gamma.push('0');
            epsilon.push('1');
        }
    }

    let gamma = isize::from_str_radix(&gamma, 2).unwrap();
    let epsilon = isize::from_str_radix(&epsilon, 2).unwrap();

    println!("Output: {}", gamma * epsilon);
}

fn get_index_sum(input_vec:&Vec<String>, index:usize) -> i32 {
   let mut output = 0;
   for line in input_vec {
     output += line.chars().nth(index).unwrap() as i32 - 48;
   }
   output
}

//#[test]
//fn test_get_index_sum

fn process_file_2(file: File) {
    let reader = BufReader::new(file);
    let lines = reader.lines();
    let mut sums = Vec::new();
    let mut max = 0;
    let mut o2_vec = Vec::new();
    let mut co2_vec = Vec::new();
    for (i,line) in lines.enumerate() {
        if let Ok(line) = line {
            o2_vec.push(String::from(&line));
            co2_vec.push(String::from(&line));
            update_sums(&mut sums,&line);
            max = i + 1;
        }
    }
    let mut o2 = String::new();
    for d in sums {
        println!("{}",d);
        let curr = (((max+1) / 2) as i32).try_into().unwrap();
        if d > curr {
            o2.push('1');
        }
        else if d < curr {
            o2.push('0');
        }
        else {
            o2.push('-');
        }
    }
    let mut index = 0;
    while o2_vec.len() > 1 {
        println!("O2: {:?}",o2_vec);
        let cur_len = o2_vec.len() as i32;
        let maj = cur_len / 2 + cur_len % 2;
        let o2_sum = get_index_sum(&o2_vec, index);
        println!("{} {}",cur_len, o2_sum);
        o2_vec.retain(|entry|{
            let e_char = entry.chars().nth(index).unwrap();

            if o2_sum > maj {
e_char == '1'
            }
            else if o2_sum < maj {
e_char == '0'
            }
            else {
             e_char == '1'
            }
        });
        index +=1;
    }
        println!("O2: {:?}",o2_vec);
    index = 0;
    while co2_vec.len() > 1 {
        println!("CO2: {:?}",co2_vec);
        let cur_len = co2_vec.len() as i32;
        let maj = cur_len / 2 + cur_len % 2;
        let co2_sum = get_index_sum(&co2_vec, index);
        println!("{} {}",cur_len, co2_sum);
        co2_vec.retain(|entry|{
            let e_char = entry.chars().nth(index).unwrap();

            if co2_sum > maj {
e_char == '0'
            }
            else if co2_sum < maj {
e_char == '1'
            }
            else {
             e_char == '0'
            }
        });
        index +=1;
    }
        println!("CO2: {:?}",co2_vec);
    let o2 = isize::from_str_radix(o2_vec.get(0).unwrap(),2).unwrap();
    let co2 = isize::from_str_radix(co2_vec.get(0).unwrap(),2).unwrap();

    println!("Output: {}", o2 * co2);
}
