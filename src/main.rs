#![allow(unused_imports)]
#![allow(dead_code)]
use rand::Rng;
use std::collections::HashMap;
use std::io;

use core::any::type_name;

fn main() {
    game();
}

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

fn debug() {
    let mut counts: HashMap<String, u32> = HashMap::new();
    counts.insert('a'.to_string(), 100);

    match counts.get(&'a'.to_string()) {
        Some(res) => {
            println!("{}", res);
        }
        None => (),
    }

    counts.insert('a'.to_string(), 200);

    match counts.get(&'a'.to_string()) {
        Some(res) => {
            println!("{}", res);
        }
        None => (),
    }
}

fn game() {
    // generate random secret and convert to chars (padding by 0 if needed)
    let secret_number: u32 = rand::thread_rng().gen_range(1, 10001);
    let secret_number = format!("{:04}", secret_number);
    let secret_cs: Vec<char> = secret_number.chars().collect();
    // for c in sec_cs {
    //     println!("{}", c.to_string());
    // }
    println!("{}", secret_number);

    // prepareing game components

    // let mut hit: u32 = 0;
    // let mut blow: u32 = 0;

    // game loop
    loop {
        // read imput and conver to chars
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // check if guessed input can be treated as number
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // convert to chars (padding by 0 if needed)
        let guess = format!("{:04}", guess);
        let guess_cs: Vec<char> = guess.chars().collect();

        // counting blow
        let mut blow: u32 = 0;
        let mut counts: HashMap<char, u32> = HashMap::new();
        for c in &guess_cs {
            let count = counts.entry(*c).or_insert(0); // kono * nani? mada rikai sitenai
            *count += 1;
        }

        for c in &secret_cs {
            match counts.get_mut(c) {
                Some(res) => {
                    if *res > 0 {
                        blow += 1;
                        *res += 1;
                    }
                }
                None => (),
            }
        }

        // counting hit
        let iter_hit = guess_cs.iter().zip(secret_cs.iter());

        let mut hit: u32 = 0;
        for (guess_c, sec_c) in iter_hit {
            if guess_c == sec_c {
                hit += 1;
            }
        }

        println!("hit: {}, blow: {}", hit, blow - hit);

        if hit == 4 {
            break;
        }
    }
}
