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
        for number in unrelated.clone(){
            if bigger.contains(&number) || smaller.contains(&number){
                unrelated.retain(|x| **x!=*number);
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
    println!("Reference table is correct.");
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
        println!("Testing rule : {}",rule);
        assert!(sorted_values.iter().position(|&x| x==split_rule[0]).expect("Oh no") < sorted_values.iter().position(|&x| x==split_rule[1]).expect("Oh no"),"CONSTRAINT NOT RESPECTED.");
    }
    println!("Sorted values respect all constraints.");
}

fn get_all_correct_updates<'a>(sorted_values : &'a Vec<&'a str>,updates_lines : &Vec<&'a str>) -> Vec<&'a str>{
    let mut correct_updates : Vec<&str> = Vec::new();
    for update in updates_lines {
        let split_update = update.split(',').collect::<Vec<&str>>();
        let mut sorted_values_copy_retain = sorted_values.clone();

        for value in sorted_values {
            if !split_update.contains(&value){
                sorted_values_copy_retain.retain(|x| **x != **value);
            }
        }
        
        if split_update == sorted_values_copy_retain {
            println!("A HIT !");
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
    let sorted_values : Vec<&str>;

    for line in data.lines() {
        if line.contains('|') {
            rules_lines.push(line);
        }
        if line.contains(',') {
            updates_lines.push(line);
        }
    }

    println!("Number of rules : {}", rules_lines.len());
    println!("Number of updates : {}", updates_lines.len());

    unique_values = get_all_unique_values(&rules_lines);

    println!("Number of unique values : {}", unique_values.len());
    println!("Unique values : {:?}", unique_values);

    reference_table = fill_reference_table(&unique_values,&rules_lines);
    test_reference_table(&reference_table,&rules_lines);

    sorted_values = sort_values(&reference_table);
    println!("Sorted values : {:?}",sorted_values);
    test_sorted_values(&sorted_values,&rules_lines);

    println!("Sum of correct middle pages : {}", get_final_answer(get_all_correct_updates(&sorted_values,&updates_lines)));
}

