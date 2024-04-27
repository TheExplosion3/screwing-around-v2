use std::{alloc::handle_alloc_error, i128, time::{Duration, SystemTime}};

extern crate rand;
use rand::{Rng};

extern crate chrono;
use chrono::{DateTime, Local, Utc};

const NAMELIST: &'static [&'static str] = &["guh?", "Cunt"];
const MODIFIERLIST: &'static [&'static str] = &[" Greater ", " Inferior ", " Masterful "];
const DESCRIPTIONLIST: &'static [&'static str] = &["Kills you instantly.", "Makes you serve.", "Transes your gender."];


fn main() {

    let timer = SystemTime::now();

    println!("How many potions would you like to generate? Enter a singular number.");
    let mut num_pot: i128 = 0;
    let mut num_pot_string: String = String::new();
    loop {

        let mut breaker: bool = false;
        std::io::stdin().read_line(&mut num_pot_string).expect("failed to readline");
        let parser = num_pot_string.trim().parse::<i128>();

        match parser {
            Ok(res) => {
                num_pot = res;
            },
            Err(_err) => {
                num_pot = -1;
                num_pot_string = String::new();
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
    println!();
    let mut pot_arr: Vec<Potion> = vec![];
    let mut num_randomizer = rand::thread_rng();
    for i in 0..num_pot {
        pot_arr.push(Potion {
            potion_number: i+1,
            name: randomizer("name"),
            strength_modifier: randomizer("modifier"),
            description: randomizer("description"),
            number_of_potions: num_randomizer.gen_range(0..i128::MAX) });
    }

    for i in pot_arr.into_iter() {
        println!("{}.{}Potion of {}\n\tDescription: {}\n\tYou have {} of these potions.", i.potion_number, i.strength_modifier, i.name, i.description, i.number_of_potions);
    }

    println!("\nStatistics:\n");
    let elapsed: Duration;
    match timer.elapsed() {
        Ok(d) => {
            elapsed = d;
        }
        Err(err) => {
            elapsed = Duration::new(0, 0);
            eprintln!("{}", err);
        }
    }

    println!("Program terminating, task complete at {} (UTC)/{} (Local), taking {} seconds to run.", Utc::now().to_rfc2822(), Local::now().to_rfc2822(), elapsed.as_secs());
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
        return NAMELIST.get(rnd.gen_range(0..NAMELIST.len())).expect("Const list, string should return.").to_string();
    }
    else if s == "modifier" {
        return MODIFIERLIST.get(rnd.gen_range(0..MODIFIERLIST.len())).expect("Const list, string should return.").to_string();
    }
    else if s == "description" {
        return DESCRIPTIONLIST.get(rnd.gen_range(0..DESCRIPTIONLIST.len())).expect("Const list, string should return.").to_string();
    }
    else {
        return String::from("Invalid argument");
    }
}

