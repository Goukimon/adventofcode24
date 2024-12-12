use std::fs;

fn get_data() -> String {
    let file_path = "/home/gouki/Documents/Repos/adventofcode24_inputs/day1/input";
    let data : String = fs::read_to_string(file_path).unwrap();
    data
}

fn main() {

    let data = get_data();
    let mut left_side : Vec<i32> = Vec::new();
    let mut right_side : Vec<i32> = Vec::new();
    let mut distances : Vec<i32> = Vec::new();

    for line in data.lines() {
        let split_line = line.split("   ").collect::<Vec<&str>>();

        left_side.push(split_line[0].parse::<i32>().unwrap());
        right_side.push(split_line[1].parse::<i32>().unwrap());
    }

    left_side.sort();
    right_side.sort();

    println!("Len left side {}",left_side.len());
    println!("Len right side {}",right_side.len());

    for i in 0..left_side.len(){
        distances.push((left_side[i] - right_side[i]).abs());
    }

    println!("Sum of all distances : {}",distances.iter().sum::<i32>());
}


