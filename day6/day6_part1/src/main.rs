use std::fs;

fn get_data() -> String {
    let file_path = "/home/gouki/Documents/Repos/adventofcode24_inputs/day6/input";
    let data : String = fs::read_to_string(file_path).unwrap();
    data
}

fn get_position_guard(playground : &Vec<Vec<char>>, direction : &char) -> (usize,usize,char){
    for i in 0..playground.len(){
        for j in 0..playground[0].len(){
            if playground[i][j] != 'X' && playground[i][j] != '.' && playground[i][j] != '#' {
                if playground[i][j] == *direction{
                    return (j,i,*direction);
                }
                else {
                    return (j,i,playground[i][j]);
                }
            }
        }
    }
    return (0,0,'Z');
}

fn check_border(mut playground : Vec<Vec<char>>, x: &usize, y: &usize, mut direction : char) -> (Vec<Vec<char>>,char){
    if direction == '^' && *y == 0{
        playground[*y][*x] = 'X';
        direction = 'Z';
    }
    if direction == '>' && *x == playground.len() - 1{
        playground[*y][*x] = 'X';
        direction = 'Z';
    }
    if direction == 'v' && *y == playground[*x].len() -1 {
        playground[*y][*x] = 'X';
        direction = 'Z';
    }
    if direction == '<' && *x == 0{
        playground[*y][*x] = 'X';
        direction = 'Z';
    }

    (playground,direction)
}

fn check_collision(playground : &Vec<Vec<char>>, x: &usize, y: &usize,direction : &char) -> bool{
    if *direction == '^' && playground[*y-1][*x] == '#'{
        return false;
    }
    if *direction == '>' && playground[*y][*x+1] == '#'{
        return false;
    }
    if *direction == 'v' && playground[*y+1][*x] == '#'{
        return false;
    }
    if *direction == '<' && playground[*y][*x-1] == '#'{
        return false;
    }
    return true;
}

fn change_direction(direction : &char) -> char{
    if *direction == '^' {
        return '>';
    }
    if *direction == '>'{
        return 'v';
    }
    if *direction == 'v'{
        return '<';
    }
    if *direction == '<'{
        return '^';
    }
    return 'Z';
}

fn pretty_print_playground(playground : &Vec<Vec<char>>){
    println!("Current playground : ");
    let mut full_grid : Vec<String> = Vec::new();

    for row in playground{
        let s = String::from_iter(row);
        full_grid.push(s);
    }
    for row in full_grid{
        println!("{}",row)
    }
}

fn take_a_step(mut playground : Vec<Vec<char>>, direction : &char, x: &usize, y: &usize) -> Vec<Vec<char>>{
    if *direction == '^'{
        playground[*y][*x] = 'X';
        playground[*y-1][*x] = *direction;
    } else if *direction == '>'{
        playground[*y][*x] = 'X';
        playground[*y][*x+1] = '>';
    } else if *direction == 'v'{
        playground[*y][*x] = 'X';
        playground[*y+1][*x] = 'v';
    } else if *direction == '<'{
        playground[*y][*x] = 'X';
        playground[*y][*x-1] = '<';
    } 

    playground
}

fn main() {

    let data = get_data();
    let mut starting_playground : Vec<Vec<char>> = Vec::new();
    //let starting_x;
    //let starting_y;
    let /*mut*/ starting_direction = '^';
    let mut playground_final : Vec<Vec<char>> = Vec::new();
    let mut number_of_positions : usize = 0;

    for line in data.lines(){
        let mut r : Vec<char> = Vec::new();
        for character in line.chars(){
            r.push(character);
        }
        starting_playground.push(r);
    }

    //(starting_x,starting_y,starting_direction) = get_position_guard(&starting_playground,&starting_direction);  
    
    let mut current_direction =starting_direction.clone();
    let mut current_x : usize;
    let mut current_y : usize;
    let mut current_playground = starting_playground.clone();
    
    while current_direction != 'Z'{

        let updated_playground : Vec<Vec<char>>;
        
        (current_x,current_y,current_direction) = get_position_guard(&current_playground, &current_direction);

        (current_playground,current_direction) = check_border(current_playground, &current_x, &current_y, current_direction);

        if current_direction == 'Z'{
            playground_final = current_playground;
            break;
        } else {
            if check_collision(&current_playground, &current_x, &current_y, &current_direction){
                updated_playground = take_a_step(current_playground, &current_direction, &current_x, &current_y);
            } else {
                current_direction = change_direction(&current_direction);
                current_playground[current_y][current_x] = current_direction;
                updated_playground = current_playground;
            }
    
            current_playground = updated_playground;
        }
    }

    pretty_print_playground(&playground_final);

    for row in playground_final{
        for character in row{
            if character == 'X'{
              number_of_positions = number_of_positions+1;  
            }
        }
    }

    println!("Number of unique positions : {}",number_of_positions);
}
