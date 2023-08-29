use rand::prelude::*;
use std::cmp::Ordering;

struct Difficulties(u128, u128);
struct Game{
    player_name:String,
    difficulties:Difficulties,
    score_file_path:String,
    number_to_guess:u128,
    number_try: u128,
}

impl Game{
    fn new(difficulties:Difficulties, player_name:String) -> Game{
        let mut rng =   rand::thread_rng();
        let random_number = rng.gen_range(difficulties.0..difficulties.1);
        Game { 
            player_name, difficulties, 
            score_file_path: String::from("./"), 
            number_to_guess:random_number,
            number_try: 0 
        }
    }
    fn guess(&mut self, number: u128) -> bool{
        self.number_try +=1;
        match number.cmp(&self.number_to_guess){
            Ordering::Equal => { true },
            _ => {false},
        }
    }
    fn saveScore() {
        //code
    }
}

fn main() {
    'main : loop {
        println!("WELCOME TO KUESSING GAME");
        'player_name_loop: loop{
            println!("type your name: ");
            let mut player_name:String = String::new();
            match std::io::stdin().read_line(&mut player_name) {
                Ok(_) => break 'player_name_loop,
                Err(_) => continue 'player_name_loop,
            }

        }
        'select_diff_loop: loop{
            println!("select the difficulties\n
                        EASY:\t 1\n
                        MEDIUM:\t 2\n
                        HARD:\t 3\n");
            let mut input_select:String = String::new();
            match std::io::stdin().read_line(&mut input_select){
                Err(e) => {
                    println!("{}", e);
                    continue 'select_diff_loop},
                Ok(_) => ()
            }
            let select: u8 = match input_select.parse::<u8>() {
                Ok(val) => val,
                Err(e) => {
                    println!("input_str:{}, err:{}",input_select, e);
                    continue 'select_diff_loop
                }
            };
            let difficulties:Difficulties = match select{
                1 => Difficulties(1, 10),
                2 => Difficulties(1, 100),
                3 => Difficulties(1, 1000),
                _ => continue 'select_diff_loop
            };
            break 'select_diff_loop;
        }   
        

        break 'main;
    }
}
