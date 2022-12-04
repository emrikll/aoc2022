use std::io::Read;
//use std::str::FromStr;

fn part1(data: Vec<Vec<&str>>) -> i32{
    let mut points: i32 = 0;
    let mut shape_points: i32;
    let mut game_points: i32;
    for game in data{
        shape_points = get_points_shape(game[1]);
        game_points = get_game_points_for_player(game[0], game[1]);
        points = points + shape_points + game_points
    }
    return points
}

fn part2(data: Vec<Vec<&str>>) -> i32{
    let mut points: i32 = 0;
    let mut shape_points: i32;
    let mut game_points: i32;
    for game in data{
        shape_points = get_points_shape(&convert_player_shape(game[0],game[1]));
        game_points = get_game_points_for_player_2(game[1]);
        points = points + shape_points + game_points
    }
    return points
}

fn get_points_shape(shape: &str) -> i32{
    if shape == "A" || shape =="X"{
        return 1
    }else if shape == "B" || shape == "Y"{
        return 2
    }else if shape == "C" || shape == "Z"{
        return 3
    }else{
        return -1
    }
}


fn get_game_points_for_player(opponent: &str, player: &str) -> i32{
    //let result_matrix: Vec<Vec<i32>> = vec![vec![2,1,0],vec![0,2,1],vec![1,0,2]];
    let result_matrix: Vec<Vec<i32>> = vec![vec![2,1,0],vec![0,2,1],vec![1,0,2]];
    let result: i32 = result_matrix[(get_points_shape(opponent)-1) as usize][(get_points_shape(player)-1) as usize];
    if result == 0{
        return 0
    }else if result == 1{
        return 6
    }else if result == 2{
        return 3
    }else{
        return -1
    }
}

fn convert_player_shape(opponent: &str, player: &str) -> String{
    //let result_matrix: Vec<Vec<i32>> = vec![vec![2,1,0],vec![0,2,1],vec![1,0,2]];
    let result_matrix: Vec<Vec<&str>> = vec![vec!["C","A","B"],vec!["A","B","C"],vec!["B","C","A"]];
    let result: &str = result_matrix[(get_points_shape(opponent)-1) as usize][(get_points_shape(player)-1) as usize];
    return result.to_string()
}

fn get_game_points_for_player_2(player: &str) -> i32{
    //let result_matrix: Vec<Vec<i32>> = vec![vec![2,1,0],vec![0,2,1],vec![1,0,2]];
    let result_matrix: Vec<i32> = vec![0,2,1];
    let result: i32 = result_matrix[(get_points_shape(player)-1) as usize];
    if result == 0{
        return 0
    }else if result == 1{
        return 6
    }else if result == 2{
        return 3
    }else{
        return -1
    }
}

fn main(){
    let mut file_ref = std::fs::File::open("src/input.txt").unwrap();

    let mut data = String::new();
   
    file_ref.read_to_string(&mut data).unwrap();

    let splited_game: Vec<&str> = data.split("\n").collect();

    let mut finished_data: Vec<Vec<&str>> = Vec::new();

    for game in splited_game{
        finished_data.push(game.split(" ").collect())
    }

    //print!("part 1 result: {}", part1(finished_data));
    print!("part 2 result: {}", part2(finished_data));

}

