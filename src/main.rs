use std::{i128};

extern crate rand;
use rand::{Rng};

const NAMELIST: &'static [&'static str] = &["bitch", "cunt"];
const MODIFIERLIST: &'static [&'static str] = &["firefox", "chrome"];

fn main() {

    println!("How many potions would you like to generate? Enter a singular number.");

    let mut num_pot: i128 = 0;
    let mut num_pot_string: String = String::new();
    loop {

        let mut breaker: bool = false;
        std::io::stdin().read_line(&mut num_pot_string).expect("failed to readline");
        let parser = num_pot_string.parse::<i128>();

        match parser {
            Ok(res) => {
                num_pot = res;
            },
            Err(_err) => {
                num_pot = -1;
            }
        };

        if num_pot == -1 {
            println!("That input was invalid, please try again.");
        }
        else {
            breaker = true;
        }

        if breaker {
            break;
        }
    }

    let mut pot_arr: Vec<Potion> = vec![];
    let mut num_randomizer = rand::thread_rng();
    for i in 1..num_pot {
        pot_arr.push(Potion {
            potion_number: i,
            name: randomizer("name"),
            strength_modifier: randomizer("modifier"),
            description: randomizer("name"),
            number_of_potions: num_randomizer.gen_range(0..i128::MAX) });
    }

    for i in pot_arr.into_iter() {
        println!("{}.{}Potion of {}\n{}.\n You have {} of these potions.", i.potion_number, i.strength_modifier, i.name, i.description, i.number_of_potions);
    }
}

struct Potion {
    potion_number: i128,
    name: String,
    strength_modifier: String,
    description: String,
    number_of_potions: i128
}

fn randomizer(s: &str) -> String {
    let mut rnd = rand::thread_rng();
    if s == "name" {
        return NAMELIST.get(rnd.gen_range(1..NAMELIST.len())).expect("Const list, string should return.").to_string();
    }
    else if s == "modifier" {
        return MODIFIERLIST.get(rnd.gen_range(1..MODIFIERLIST.len())).expect("Const list, string should return.").to_string();
    }
    else {
        return String::from("Invalid argument");
    }
}

