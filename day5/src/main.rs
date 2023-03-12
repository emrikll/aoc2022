use std::io::Read;
//use std::str::FromStr;



fn main(){
    let mut file_ref = std::fs::File::open("src/input.txt").unwrap();

    let mut data = String::new();
   
    file_ref.read_to_string(&mut data).unwrap();

    let mut stack1: Vec<&str> = vec!{"W","R","F"};
    let mut stack2: Vec<&str> = vec!{"T","H","M","C","D","V","W","P"};
    let mut stack3: Vec<&str> = vec!{"P","M","Z","N","L"};
    let mut stack4: Vec<&str> = vec!{"J","C","H","R"};
    let mut stack5: Vec<&str> = vec!{"C","P","G","H","Q","T","B"};
    let mut stack6: Vec<&str> = vec!{"G","C","W","L","F","Z"};
    let mut stack7: Vec<&str> = vec!{"W","V","L","Q","Z","J","G","C"};
    let mut stack8: Vec<&str> = vec!{"P","N","R","F","W","T","V","C"};
    let mut stack9: Vec<&str> = vec!{"J","W","H","G","R","S","V"}; 

    let splited_game: Vec<&str> = data.split("\n").collect();
    //print!("{:?}",splited_game);
    //let mut splited_list: Vec<&str> = splited_game[1].split("\n").unwrap();
    
    print!("part 1: {} \n", part1(parsing(&splited_game)));
    print!("part 2: {} \n", part2(parsing(&splited_game)));

}

fn parsing(input: &Vec<&str>) -> Vec<i32>{
    let mut splited_list: Vec<Vec<&str>> = Vec::new();
    let mut num_list: Vec<Vec<&str>> = Vec::new();
    let mut num_tmp: Vec<i32> = Vec::new();

    for j in input{
        //let mut tmp: String = ;
        splited_list.push(j.split(" ").collect());
    }

    for mut word in splited_list{
        for x in 0..word.len(){
            if x%2 == 0{
                word.remove(x);
            }else{
                num_list[x] = word[x].parse::<i32>().unwrap();
            }
        }
    }

    for n in num_list{
        let x: i32 = n[0].parse::<i32>().unwrap();
        let y: i32 = n[1].parse::<i32>().unwrap();
        num_tmp.push(x);
        num_tmp.push(y);
    }
    //print!("{:?}", num_tmp);
    return num_tmp
}

fn part1(inp: Vec<i32>) -> i32{
    let (mut x1, mut x2, mut x3, mut x4): (i32, i32, i32, i32);
    let mut sum:i32 = 0;
    for x in (0..inp.len()).step_by(4){
        x1 = inp[x];
        x2 = inp[x+1];
        x3 = inp[x+2];
        x4 = inp[x+3];
        //println!("{},{},{},{}",x1,x2,x3,x4);
        //println!("new ones");

        if x1 <= x3 && x2 >= x4{
            sum += 1
        }else if x1 >= x3 && x2 <= x4{
            sum += 1
        }
    }
    return sum
}

fn part2(inp: Vec<i32>) -> i32{
    let (mut x1, mut x2, mut x3, mut x4): (i32, i32, i32, i32);
    let mut sum:i32 = 0;
    for x in (0..inp.len()).step_by(4){
        x1 = inp[x];
        x2 = inp[x+1];
        x3 = inp[x+2];
        x4 = inp[x+3];
        //println!("{},{},{},{}",x1,x2,x3,x4);
        //println!("new ones");

        if x2 < x3{
            continue
        }else if x1 > x4{
            continue
        }else{
            sum += 1
        }
    }
    return sum
}
