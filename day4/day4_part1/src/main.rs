use std::env;
use std::fs;


const ERRORFILE: &str = "ERROR WITH THE FILE.";

fn input_file() -> String {
    env::args().nth(1).unwrap()
}

fn get_data() -> String {
    let file_path : String = input_file();

    let data : String = fs::read_to_string(file_path).unwrap();//ERRORFILE.to_string());

    if data == ERRORFILE.to_string() {
        panic!("{}",ERRORFILE);
    }

    data
}

fn check_top_left(i: usize,j: usize, all_lines: &Vec<Vec<char>>) -> i32{
    if all_lines[i-1][j-1] =='M'{
        if all_lines[i-2][j-2] == 'A'{
            if all_lines[i-3][j-3] =='S'{
                return 1;
            }
        }
    }
    return 0;
}

fn check_top(i: usize,j: usize, all_lines: &Vec<Vec<char>>) -> i32{
    if all_lines[i-1][j] =='M'{
        if all_lines[i-2][j] == 'A'{
            if all_lines[i-3][j] =='S'{
                return 1;
            }
        }
    }
    return 0;
}

fn check_top_right(i: usize,j: usize, all_lines: &Vec<Vec<char>>) -> i32{
    if all_lines[i-1][j+1] =='M'{
        if all_lines[i-2][j+2] == 'A'{
            if all_lines[i-3][j+3] =='S'{
                return 1;
            }
        }
    }
    return 0;
}

fn check_left(i: usize,j: usize, all_lines: &Vec<Vec<char>>) -> i32{
    if all_lines[i][j-1] =='M'{
        if all_lines[i][j-2] == 'A'{
            if all_lines[i][j-3] =='S'{
                return 1;
            }
        }
    }
    return 0;
}

fn check_right(i: usize,j: usize, all_lines: &Vec<Vec<char>>) -> i32{
    if all_lines[i][j+1] =='M'{
        if all_lines[i][j+2] == 'A'{
            if all_lines[i][j+3] =='S'{
                return 1;
            }
        }
    }
    return 0;
}

fn check_bottom_left(i: usize,j: usize, all_lines: &Vec<Vec<char>>) -> i32{
    if all_lines[i+1][j-1] =='M'{
        if all_lines[i+2][j-2] == 'A'{
            if all_lines[i+3][j-3] =='S'{
                return 1;
            }
        }
    }
    return 0;
}

fn check_bottom(i: usize,j: usize, all_lines: &Vec<Vec<char>>) -> i32{
    if all_lines[i+1][j] =='M'{
        if all_lines[i+2][j] == 'A'{
            if all_lines[i+3][j] =='S'{
                return 1;
            }
        }
    }
    return 0;
}

fn check_bottom_right(i: usize,j: usize, all_lines: &Vec<Vec<char>>) -> i32{
    if all_lines[i+1][j+1] =='M'{
        if all_lines[i+2][j+2] == 'A'{
            if all_lines[i+3][j+3] =='S'{
                return 1;
            }
        }
    }
    return 0;
}

fn check_xmas(i: usize,j: usize, number_of_rows: usize, number_of_columns: usize, all_lines: &Vec<Vec<char>>) -> i32 {
    
    let mut accumulator = 0;

    if i >= 3 && j >= 3  {
        accumulator = accumulator + check_top_left(i,j, all_lines);
    }

    if i >= 3 {
        accumulator = accumulator + check_top(i,j, all_lines);
    }

    if i >= 3 && j <= (number_of_columns-4){
        accumulator = accumulator + check_top_right(i,j, all_lines);
    } 

    if j >= 3 {
        accumulator = accumulator + check_left(i,j, all_lines);
    }

    if j <= (number_of_columns-4){
        accumulator = accumulator + check_right(i,j, all_lines);
    }
    
    if i <= (number_of_rows-4) && j >= 3{
        accumulator = accumulator + check_bottom_left(i,j, all_lines);
    }

    if i <= (number_of_rows-4){
        accumulator = accumulator + check_bottom(i,j, all_lines);
    }
    
    if i <= (number_of_rows-4) && j <= (number_of_columns-4){
        accumulator = accumulator + check_bottom_right(i,j, all_lines);
    }

    accumulator
}

fn main() {

    let data = get_data();

    let mut all_lines : Vec<_> = Vec::new();

    for line in data.lines() {
        all_lines.push(line.chars().collect::<Vec<_>>());
    }

    let number_of_rows = all_lines.len();
    let number_of_columns = all_lines[0].len();
    let mut number_of_xmas = 0;

    println!("Rows : {}",number_of_rows);
    println!("Columns : {}",number_of_columns);

    for i in 0..number_of_rows {
        for j in 0..number_of_columns {
            if all_lines[i][j] == 'X'{
                number_of_xmas = number_of_xmas + check_xmas(i,j,number_of_rows,number_of_columns,&all_lines);
            }
        } 
    }

    println!("Number of XMAS : {}",number_of_xmas);
}

