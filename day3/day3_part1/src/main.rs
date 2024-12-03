extern crate regex;

use std::env;
use std::fs;
use regex::Regex;


const ERRORFILE: &str = "ERROR WITH THE FILE.";

fn input_file() -> String {
    env::args().nth(1).unwrap()
}

fn get_data() -> String {
    let file_path : String = input_file();

    let data : String = fs::read_to_string(file_path).unwrap_or(ERRORFILE.to_string());

    if data == ERRORFILE.to_string() {
        panic!("{}",ERRORFILE);
    }

    data
}

fn main() {

    let data = get_data();

    let re_mul = Regex::new(r"mul\(([0-9]){1,3},([0-9]){1,3}\)").unwrap();
    let re_numbers = Regex::new(r"[0-9]{1,3}").unwrap();

    let mut all_matches: Vec<&str> = Vec::new();
    //let mut numbers: Vec<i32> = Vec::new();
    let mut end_value: i32 = 0;

    for line in data.lines() {
        all_matches.append(&mut re_mul.find_iter(line).map(|m| m.as_str()).collect());
    }

    for element in all_matches{
        let numbers = re_numbers.find_iter(element).map(|m| m.as_str().parse::<i32>().unwrap()).collect::<Vec<i32>>();
        end_value = numbers[0] * numbers[1] + end_value
    }

    println!("End value : {}", end_value);

}