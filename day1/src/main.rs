use std::io::Read;
use std::str::FromStr;

fn main(){
    let mut file_ref = std::fs::File::open("/home/emrik/Github/aoc2022/hello-rust/src/input.txt").unwrap();

    let mut data = String::new();
   
    file_ref.read_to_string(&mut data).unwrap();

    let splited_pack: Vec<&str> = data.split("\n\n").collect();

    let mut max: i32 = 0;

    for elf_pack in splited_pack{
        let x: i32 = get_calories(elf_pack);
        if x > max{
            max = x
        }
        
    }
    print!("{}",max)

}

fn get_calories(pack: &str) -> i32{
    let one_pack: Vec<&str> = pack.split("\n").collect();
    let mut calories: i32 = 0;

    for food in one_pack{
        //let num: i32 = food.parse().unwrap();

        let num = i32::from_str(food).unwrap();
        calories = calories + num
        
    }

    return calories
}
