use std::fs;

fn get_data() -> String {
    let file_path = "/home/gouki/Documents/Repos/adventofcode24_inputs/day2/input";
    let data : String = fs::read_to_string(file_path).unwrap();
    data
}

fn check_ascending_descending(reports : &Vec<Vec<usize>>) -> (Vec<Vec<usize>>,Vec<Vec<usize>>){
    let mut sorted_reports : Vec<Vec<usize>> = Vec::new();
    let mut unsafe_reports : Vec<Vec<usize>> = Vec::new();
    
    for r in reports{
        if r.is_sorted_by(|a,b| a > b) {
            sorted_reports.push(r.clone());
        }
        else if r.is_sorted_by(|a,b| a < b) {
            sorted_reports.push(r.clone());
        }
        else {
            unsafe_reports.push(r.clone());
        }
    }

    (sorted_reports,unsafe_reports)    
}

fn check_difference(reports : &Vec<Vec<usize>>) -> (Vec<Vec<usize>>,Vec<Vec<usize>>){
    let mut safe_reports : Vec<Vec<usize>> = Vec::new();
    let mut unsafe_reports : Vec<Vec<usize>> = Vec::new();
    
    for r in reports {
        let end_of_report = r.len(); 

        for i in 0..end_of_report {
            if i+1 == end_of_report{
                safe_reports.push(r.to_vec());
                break;
            }
            if (r[i]).abs_diff(r[i+1]) > 3 {
                unsafe_reports.push(r.to_vec());
                break;
            }
            if (r[i]).abs_diff(r[i+1]) < 1 {
                unsafe_reports.push(r.to_vec());
                break;
            }
        }
    }

    (safe_reports,unsafe_reports)
}

fn main() {

    let data = get_data();
    let mut report : Vec<usize> = Vec::new();
    let mut all_reports : Vec<Vec<usize>> = Vec::new();
    let sorted_reports : Vec<Vec<usize>>;
    let mut safe_reports : Vec<Vec<usize>>;
    let mut unsafe_reports : Vec<Vec<usize>>;

    for line in data.lines() {
        let split_line = line.split(" ").collect::<Vec<&str>>();
        
        report.clear();

        for string_value in split_line{
            report.push(string_value.parse::<usize>().unwrap());
        }

        all_reports.push(report.clone());
    }

    (sorted_reports,unsafe_reports) = check_ascending_descending(&all_reports);

    safe_reports = check_difference(&sorted_reports).0;
    unsafe_reports.extend(check_difference(&sorted_reports).1);


    println!("Number of reports : {}",all_reports.len());
    println!("Number of sorted reports : {}",sorted_reports.len());
    println!("Number of safe reports (PART 1 COMPLIANT) : {}",safe_reports.len());
    println!("Number of unsafe reports : {}", unsafe_reports.len());

    for report in unsafe_reports{
        let mut faulty_reports : Vec<Vec<usize>> = Vec::new();
        let dampened_reports : Vec<Vec<usize>>;
        
        for i in 0..report.len(){
            let mut report_copy = report.clone();
            report_copy.remove(i);
            faulty_reports.push(report_copy);
        }

       
        dampened_reports = check_difference(&(check_ascending_descending(&faulty_reports).0)).0;

        if dampened_reports.len() != 0 {
            safe_reports.push(dampened_reports[0].clone());
        }
    }

    println!("Number of safe reports after dampening : {}", safe_reports.len());

}

