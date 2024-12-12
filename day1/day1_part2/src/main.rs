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
    let mut similarity_score : Vec<i32> = Vec::new();

    for line in data.lines() {
        let split_line = line.split("   ").collect::<Vec<&str>>();

        left_side.push(split_line[0].parse::<i32>().unwrap());
        right_side.push(split_line[1].parse::<i32>().unwrap());
    }

    left_side.sort();
    right_side.sort();

    println!("Len left side {}",left_side.len());
    println!("Len right side {}",right_side.len());

    for value_left in &left_side{
        let mut occurences = 0;
        for value_right in &right_side{
            if value_left == value_right{
                occurences = occurences+1;
            }
        }
        similarity_score.push(value_left * occurences);
    }

    println!("Similarity score : {}",similarity_score.iter().sum::<i32>());
}
