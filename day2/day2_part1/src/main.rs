use std::fs;

fn get_data() -> String {
    let file_path = "/home/gouki/Documents/Repos/adventofcode24_inputs/day2/input";
    let data : String = fs::read_to_string(file_path).unwrap();
    data
}

fn main() {

    let data = get_data();
    let mut report : Vec<usize> = Vec::new();
    let mut all_reports : Vec<Vec<usize>> = Vec::new();
    let mut sorted_reports : Vec<Vec<usize>> = Vec::new();
    let mut safe_reports : Vec<Vec<usize>> = Vec::new();

    for line in data.lines() {
        let split_line = line.split(" ").collect::<Vec<&str>>();
        
        report.clear();

        for string_value in split_line{
            report.push(string_value.parse::<usize>().unwrap());
        }

        all_reports.push(report.clone());
    }

    for r in &all_reports{
        if r.is_sorted_by(|a,b| a > b) {
            sorted_reports.push(r.clone());
        }
        if r.is_sorted_by(|a,b| a < b) {
            sorted_reports.push(r.clone());
        }
    }

    for r in &sorted_reports {
        let end_of_report = r.len(); 

        for i in 0..end_of_report {
            if i+1 == end_of_report{
                safe_reports.push(r.to_vec());
                break;
            }
            if (r[i]).abs_diff(r[i+1]) > 3 {
                break;
            }
            if (r[i]).abs_diff(r[i+1]) < 1 {
                break;
            }
        }
    }

    println!("Number of reports : {}",all_reports.len());
    println!("Number of sorted reports : {}",sorted_reports.len());
    println!("Number of safe reports : {}",safe_reports.len());
}
