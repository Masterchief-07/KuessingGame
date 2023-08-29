use rand::prelude::*;
use std::fs::File;
use std::io::Write;
use std::{cmp::Ordering, str::FromStr};

#[derive(Debug, Clone, Copy)]
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
            score_file_path: String::from("./score.txt"), 
            number_to_guess:random_number,
            number_try: 0 
        }
    }
    fn guess(&mut self, number: &u128) -> Ordering{
        self.number_try +=1;
        number.cmp(&self.number_to_guess)
    }

    fn save_score(&self) -> Option<bool> {
        let mut file = match File::options().create(true).write(true).append(true).open(self.score_file_path.clone()){
            Ok(val) => val,
            Err(e) => {
                println!("{}", e);
                return None
            },
       };
       match writeln!(&mut file, "{}_{}_{}_{:?}", self.player_name.trim(), self.number_to_guess, self.number_try, self.difficulties){
            Ok(_) => Some(true),
            Err(_) => None
       }
    }
}

fn main() {
    let mut player_name: String = String::new();
    let mut difficulties: Difficulties;
    'main : loop {
        println!("WELCOME TO KUESSING GAME");
        'player_name_loop: loop{
            println!("type your name: ");
            // let mut player_name:String = String::new();
            match std::io::stdin().read_line(&mut player_name) {
                Ok(_) => break 'player_name_loop,
                Err(_) => continue 'player_name_loop,
            }

        }
        'party : loop{
        'select_diff_loop: loop{
            println!("select the difficulties\nEASY:\t 1\nMEDIUM:\t 2\nHARD:\t 3\n");
            let select = get_from_keyboard(Some(vec![1,2,3]));
            difficulties = match select{
                Some(1) => Difficulties(1, 10),
                Some(2) => Difficulties(1, 100),
                Some(3) => Difficulties(1, 1000),
                _ => continue 'select_diff_loop
            };
            break 'select_diff_loop;
        }   
        
        let mut game:Game = Game::new(difficulties, player_name.clone());
        'gamming: loop {
            println!("tape number: ");
            let guess = match get_from_keyboard::<u128>(None){
                Some(val) => val,
                None => {
                    println!("please type a number!");
                    continue 'gamming;
                }
            };
            match game.guess(&guess){
                Ordering::Equal => {
                    println!("SUCCEED!!!");
                    break 'gamming;
                },
                Ordering::Greater => {
                    println!("TOO HIGH");
                    continue 'gamming;
                },
                Ordering::Less => {
                    println!("TOO LOW");
                    continue 'gamming;
                },
            }
        }
        game.save_score();
        drop(game);

        println!("DO YOU WANT TO PLAY ANOTHER MATCH?(Y,n):");
        match get_from_keyboard(Some(vec!['Y','y'])){
            Some(_) => continue 'party,
            _ => break 'party,
        }
    }

        println!("THANK FOR PLAYING {}", player_name);
        break 'main;
    }
}

fn get_from_keyboard<T>(expected:Option<Vec<T>>) -> Option<T>
    where T : FromStr + PartialEq + PartialOrd + Clone + Copy{
    let mut input:String = String::new();
    match std::io::stdin().read_line(&mut input){
        Err(_) => return None,
        Ok(_) => ()
    };
    let result = match input.trim().parse::<T>(){
        Ok(val) => val,
        Err(_) => return None
    };
    return match &expected{
        Some(val) =>  
                    match val.iter().find(|&&x| x == result){
                        Some(_) => Some(result),
                        None => None,
                    },
        _ => Some(result)
    }

}