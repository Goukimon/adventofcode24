use std::env;
use std::fs;


fn input_file() -> String {
    env::args().nth(1).unwrap()
}

fn get_data() -> String {
    let file_path : String = input_file();
    let data : String = fs::read_to_string(file_path).unwrap();
    data
}

fn get_all_unique_values<'a>(rules_lines : &'a Vec<&'a str>) -> Vec<&'a str>{
    let mut unique_values : Vec<&str> = Vec::new();
    for rule in rules_lines {
        let split_rule = rule.split('|').collect::<Vec<&str>>();
        if !unique_values.contains(&split_rule[0]){
            unique_values.push(&split_rule[0]);
        }
        if !unique_values.contains(&split_rule[1]){
            unique_values.push(&split_rule[1]);
        }
    }

    return unique_values;
}

fn get_all_unique_values_from_an_update<'a>(update : &'a str) -> Vec<&'a str>{
    let mut unique_values : Vec<&str> = Vec::new();
    
    let split_update = update.split(',').collect::<Vec<&str>>();
    for value in split_update{
        if !unique_values.contains(&value){
            unique_values.push(&value);
        }
    }

    return unique_values;
}

fn fill_reference_table<'a>(unique_values : &'a Vec<&'a str>, rules_lines : &'a Vec<&'a str>) -> Vec<(&'a str,Vec<&'a str>,Vec<&'a str>,Vec<&'a str>)>{
    let mut bigger : Vec<&str> = Vec::new();
    let mut smaller : Vec<&str> = Vec::new();
    let mut unrelated : Vec<&str> = Vec::new();
    let mut reference_table : Vec<(&str,Vec<&str>,Vec<&str>,Vec<&str>)> = Vec::new();

    for value in unique_values {
        bigger.clear();
        smaller.clear();
        unrelated.clear();
        for rule in rules_lines {
            let split_rule = rule.split('|').collect::<Vec<&str>>();
            if split_rule.contains(&value){
                if split_rule[0] == *value {
                    bigger.push(&split_rule[1]);
                }
                else {
                    smaller.push(&split_rule[0]);
                }
            }
            else {
                unrelated.push(&split_rule[1]);
                unrelated.push(&split_rule[0]);
            }
        }
        if smaller.len() + bigger.len() + 1 != unique_values.len(){
            for number in unrelated.clone(){
                if bigger.contains(&number) || smaller.contains(&number){
                    unrelated.retain(|x| **x!=*number);
                }
            }
        }

        reference_table.push((value,smaller.clone(),bigger.clone(),unrelated.clone()));
    }

    reference_table
}

fn test_reference_table(reference_table : &Vec<(&str,Vec<&str>,Vec<&str>,Vec<&str>)>, rules_lines : &Vec<&str>){
    for reference in reference_table{
        for rule in rules_lines {
            let split_rule = rule.split('|').collect::<Vec<&str>>();
            if split_rule.contains(&reference.0){
                if split_rule[0] == reference.0{
                    assert!(reference.2.contains(&split_rule[1]));
                }
                else{
                    assert!(reference.1.contains(&split_rule[0]));
                }
            }
        }
    }
}

fn sort_values<'a>(reference_table : &'a Vec<(&'a str,Vec<&'a str>,Vec<&'a str>,Vec<&'a str>)>) -> Vec<&'a str>{
    let mut sorted_values : Vec<&str> = Vec::new();

    for row in reference_table{
        if sorted_values.is_empty(){
            sorted_values.push(row.0);
        }        
        else{
            for value in sorted_values.clone(){
                if row.1.contains(&value){
                    sorted_values.insert(sorted_values.iter().position(|&x| x==value).expect("Oh no"),row.0);
                    break;
                }
            }
            if !sorted_values.contains(&row.0){
                sorted_values.push(row.0);
            }
        }
    }
    sorted_values.reverse();

    sorted_values
}

fn test_sorted_values(sorted_values : &Vec<&str>,rules_lines : &Vec<&str>){
    for rule in rules_lines{
        let split_rule = rule.split('|').collect::<Vec<&str>>();
        if sorted_values.contains(&split_rule[0]) && sorted_values.contains(&split_rule[1]){
            assert!(sorted_values.iter().position(|&x| x==split_rule[0]).expect("Oh no") < sorted_values.iter().position(|&x| x==split_rule[1]).expect("Oh no"),"CONSTRAINT NOT RESPECTED.");
        }   
    }
}

fn trim_reference_table<'a>(unique_values: &'a Vec<&'a str>, reference_table: &'a Vec<(&'a str,Vec<&'a str>,Vec<&'a str>,Vec<&'a str>)>) -> Vec<(&'a str,Vec<&'a str>,Vec<&'a str>,Vec<&'a str>)>{
    let mut trimmed_reference_table : Vec<(&str,Vec<&str>,Vec<&str>,Vec<&str>)> = Vec::new();

    for row in reference_table{
        for value in unique_values{
            if row.0 == *value{
                trimmed_reference_table.push(row.clone());
            }
        }
    }

    trimmed_reference_table
}

fn check_correct_update<'a>(sorted_values : &'a Vec<&'a str>,update : &'a str) -> bool{
    let split_update = update.split(',').collect::<Vec<&str>>();
    
    if &split_update == sorted_values {
        return true;
    } else {
        return false;
    }
}

fn get_correct_updates<'a>(rules_lines : &'a Vec<&'a str>,updates_lines : &'a Vec<&'a str>,reference_table : &'a Vec<(&'a str,Vec<&'a str>,Vec<&'a str>,Vec<&'a str>)>) -> Vec<&'a str>{
    let mut correct_updates : Vec<&str> = Vec::new();
    let mut unique_values : Vec<&str>;  
    let mut trimmed_reference_table : Vec<(&str,Vec<&str>,Vec<&str>,Vec<&str>)>;
    let mut sorted_values : Vec<&str>;

    for update in updates_lines{
        unique_values = get_all_unique_values_from_an_update(&update);
        trimmed_reference_table = trim_reference_table(&unique_values,&reference_table);
        sorted_values = sort_values(&trimmed_reference_table);

        test_sorted_values(&sorted_values,&rules_lines);

        if check_correct_update(&sorted_values,update){
            correct_updates.push(update);
        }        
    }

    correct_updates
}

fn get_final_answer(correct_updates: Vec<&str>) -> i32 {
    let mut all_correct_middle_pages = 0;

    for update in correct_updates{
        let split_update = update.split(',').collect::<Vec<&str>>();
        all_correct_middle_pages = all_correct_middle_pages + split_update[split_update.len()/2].parse::<i32>().unwrap();
    }

    all_correct_middle_pages
}

fn main() {

    let data = get_data();

    let mut rules_lines : Vec<&str> = Vec::new();
    let mut updates_lines : Vec<&str> = Vec::new();
    let unique_values : Vec<&str>;
    let reference_table : Vec<(&str,Vec<&str>,Vec<&str>,Vec<&str>)>;
    let correct_updates : Vec<&str>;

    for line in data.lines() {
        if line.contains('|') {
            rules_lines.push(line);
        }
        if line.contains(',') {
            updates_lines.push(line);
        }
    }

    unique_values = get_all_unique_values(&rules_lines);

    reference_table = fill_reference_table(&unique_values,&rules_lines);
    test_reference_table(&reference_table,&rules_lines);

    correct_updates = get_correct_updates(&rules_lines,&updates_lines,&reference_table);

    println!("Sum of correct middle pages : {}", get_final_answer(correct_updates));
}

