use std::io::Read;
//use std::str::FromStr;
use ascii_converter::*;

fn check_duplicate(data: &str) -> String{
    let (first,last) = data.split_at(data.len()/2);
    for char in first.chars(){
        if last.contains(char){
            return char.to_string()
        }
    }
    return "false".to_string()
}

fn check_in_all(x: Vec<&str>) -> String{
    for char in x[0].chars(){
        if x[1].contains(char) && x[2].contains(char){
            return char.to_string()
        }
    }
    return "false".to_string()
}

fn get_priority(s: String) -> i32{
    let num= string_to_decimals(s.as_str()).unwrap();
    let prio = num[0] as i32;
    if prio > 96{
        return prio - 96
    }else{
        return prio - 38
    }
}

fn part1(proirity_list: Vec<String>) -> i32{
    let mut sum:i32 = 0;
    for f in proirity_list{
        sum = sum + get_priority(f)
    }
    return sum
}

fn part2(proirity_list: Vec<Vec<&str>>) -> i32{
    let mut sum:i32 = 0;
    for s in proirity_list{
        sum = sum + get_priority(check_in_all(s))
    }
    return sum
}

fn main(){
    let mut file_ref = std::fs::File::open("src/input.txt").unwrap();

    let mut data = String::new();
   
    file_ref.read_to_string(&mut data).unwrap();

    let splited_game: Vec<&str> = data.split("\n").collect();

    let num = splited_game.len();

    let mut finished_data: Vec<String> = Vec::new();

    let mut finished_data_part2: Vec<Vec<&str>> = Vec::new();

    for game in &splited_game{
        finished_data.push(check_duplicate(game))
    }

    for x in (0..num).step_by(3) {
        let mut new_list: Vec<&str> = Vec::new();
        new_list.push(splited_game[x]);
        new_list.push(splited_game[x+1]);
        new_list.push(splited_game[x+2]);
        finished_data_part2.push(new_list);
    }
    
    print!("part 1: {} \n", part1(finished_data));
    print!("part 2: {} \n", part2(finished_data_part2));

}