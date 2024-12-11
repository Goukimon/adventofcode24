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

fn sort_rules_numbers<'a>(v : &Vec<&'a str>, rules_lines : &Vec<&'a str>) -> Vec<&'a str> {
    let mut bigger : Vec<&str> = Vec::new();
    let mut smaller : Vec<&str> = Vec::new();
    let mut stripped_rules : Vec<&str> = Vec::new();
    let mut stripped_rules_small_side : Vec<&str> = Vec::new();
    let mut stripped_rules_big_side : Vec<&str> = Vec::new();
    let mut stripped_rules_unrelated : Vec<&str> = Vec::new();

    let mut small_side : Vec<&str> = Vec::new();
    let mut big_side : Vec<&str> = Vec::new();

    if v.len() == 1 {
        return v.to_vec();
    }
    if v.len() == 0 {
        return v.to_vec();
    }

    for rule in rules_lines {
        let split_rule = rule.split('|').collect::<Vec<&str>>();
        if split_rule.contains(&v[0]){
            if split_rule[0] == v[0] {
                bigger.push(split_rule[1]);
            }
            else {
                smaller.push(split_rule[0]);
            }
        }
        else {
            stripped_rules.push(rule);
        }
    }

    for rule in stripped_rules {
        let split_rule = rule.split('|').collect::<Vec<&str>>();
        if smaller.contains(&split_rule[0]) && smaller.contains(&split_rule[1]){
            stripped_rules_small_side.push(rule);
        } 
        else if bigger.contains(&split_rule[0]) && bigger.contains(&split_rule[1]){
            stripped_rules_big_side.push(rule);
        }
        else {
            stripped_rules_unrelated.push(rule);
        }
    }

    /*
    while stripped_rules_unrelated.len() > 0{

        println!("stripped_rules_unrelated.len() : {}", stripped_rules_unrelated.len());
        println!("STRIPPED RULES UNRELATED : {:?}",stripped_rules_unrelated);
        println!("smaller : {:?}",smaller);
        println!("bigger : {:?}",bigger);

        let stripped_rules_unrelated_copy = stripped_rules_unrelated.clone();

        for rule in stripped_rules_unrelated_copy {
            let split_rule = rule.split('|').collect::<Vec<&str>>();

            println!("ENTERING THE SPLASH ZONE");
            println!("Values checked : &split_rule[0] = {} and &split_rule[1] = {}", &split_rule[0],&split_rule[1]);
            println!("smaller : {:?}",smaller);
            println!("bigger : {:?}",bigger);

            if smaller.contains(&split_rule[0]) && smaller.contains(&split_rule[1]){
                stripped_rules_small_side.push(rule);
                println!("ENTERING RETAIN ZONE FOR BOTH VALUES IN SMALLER : {:?}",stripped_rules_unrelated);
                stripped_rules_unrelated.retain(|&x| x!=rule);
                println!("EXITING RETAIN ZONE FOR BOTH VALUES IN SMALLER : {:?}",stripped_rules_unrelated);
            } 
            else if bigger.contains(&split_rule[0]) && bigger.contains(&split_rule[1]){
                stripped_rules_big_side.push(rule);
                println!("ENTERING RETAIN ZONE FOR BOTH VALUES IN BIGGER : {:?}",stripped_rules_unrelated);
                stripped_rules_unrelated.retain(|&x| x!=rule);
                println!("EXITING RETAIN ZONE FOR BOTH VALUES IN BIGGER : {:?}",stripped_rules_unrelated);
            }
            else if bigger.contains(&split_rule[0]) && !bigger.contains(&split_rule[1]){
                    bigger.push(&split_rule[1]);
                    stripped_rules_big_side.push(rule);
                    println!("ENTERING RETAIN ZONE FOR LEFT VALUE IN BIGGER : {:?}",stripped_rules_unrelated);
                    stripped_rules_unrelated.retain(|&x| x!=rule);
                    println!("EXITING RETAIN ZONE FOR LEFT VALUES IN BIGGER : {:?}",stripped_rules_unrelated);
            }
            else if smaller.contains(&split_rule[1]) && !smaller.contains(&split_rule[0]){
                    smaller.push(&split_rule[0]);
                    println!("ENTERING RETAIN ZONE FOR RIGHT VALUE IN SMALLER : {:?}",stripped_rules_unrelated);
                    stripped_rules_unrelated.retain(|&x| x!=rule);
                    println!("EXITING RETAIN ZONE FOR RIGHT VALUE IN SMALLER : {:?}",stripped_rules_unrelated);
            }

            println!("EXITING THE SPLASH ZONE");
        }
    }*/

    //stripped_rules_small_side.extend(stripped_rules_unrelated.clone());
    //stripped_rules_big_side.extend(stripped_rules_unrelated);

    small_side = sort_rules_numbers(&smaller, &stripped_rules_small_side);
    big_side = sort_rules_numbers(&bigger, &stripped_rules_big_side);

    small_side.push(v[0]);
    small_side.extend(big_side);
    return small_side;
}

fn main() {

    let data = get_data();

    let mut rules_lines : Vec<&str> = Vec::new();
    let mut updates_lines : Vec<_> = Vec::new();
    let mut unique_values : Vec<&str> = Vec::new();
    let mut sorted_values : Vec<&str> = Vec::new();
    let mut all_correct_middle_pages = 0;
    let mut left_side : Vec<&str> = Vec::new();
    let mut right_side : Vec<&str> = Vec::new();
    let mut smallest_value : Vec<&str> = Vec::new();
    let mut biggest_value : Vec<&str> = Vec::new();

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

    for rule in &rules_lines {
        let split_rule = rule.split('|').collect::<Vec<&str>>();
        if !unique_values.contains(&split_rule[0]){
            unique_values.push(&split_rule[0]);
        }
        if !unique_values.contains(&split_rule[1]){
            unique_values.push(&split_rule[1]);
        }
        left_side.push(split_rule[0]);
        right_side.push(split_rule[1]);
    }

    for value in &unique_values {
        if !left_side.contains(value){
            biggest_value.push(value);
        }
        if !right_side.contains(value){
            smallest_value.push(value);
        }
    }

    println!("Number of unique values : {}", unique_values.len());
    println!("Unique values : {:?}", unique_values);
    println!("Smallest value : {:?}",smallest_value);
    println!("Biggest value : {:?}",biggest_value);

    sorted_values = sort_rules_numbers(&unique_values,&rules_lines).to_vec();

    println!("Length of sorted_values : {}",sorted_values.len());
    println!("All sorted values : {:?}",sorted_values);

    assert_eq!(sorted_values.len(),unique_values.len());

    for update in &updates_lines {
        let split_update = update.split(',').collect::<Vec<&str>>();
        let sorted_values_copy = sorted_values.clone();
        let mut sorted_values_copy_retain = sorted_values.clone();

        for value in &sorted_values_copy {
            if !split_update.contains(&value){
                //println!("ENTERING RETAIN ZONE UPDATE");
                //println!("sorted_values_copy_retain : {:?}", sorted_values_copy_retain);
                sorted_values_copy_retain.retain(|x| **x != **value);
                //println!("sorted_values_copy_retain : {:?}", sorted_values_copy_retain);
                //println!("EXITING RETAIN ZONE UPDATE");
            }
        }
        
        if split_update == sorted_values_copy_retain {
            all_correct_middle_pages = all_correct_middle_pages + split_update[split_update.len()/2].parse::<i32>().unwrap();
        }
    }

    println!("Sum of correct middle pages : {}", all_correct_middle_pages);
}

