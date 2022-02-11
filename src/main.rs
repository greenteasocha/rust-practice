#![allow(unused_imports)]
#![allow(dead_code)]
use std::io;
use rand::Rng;
use std::collections::HashMap;

fn main() {
    debug();
}

fn debug() {
    let mut counts: HashMap<char, u32> = HashMap::new();
    counts(*)
}

fn game() {
    // // generate random secret and convert to chars (padding by 0 if needed)
    // let secret_number: u32 = rand::thread_rng().gen_range(1,10001);
    // let secret_number = format!("{:04}", secret_number);
    // let secret_cs: Vec<char> = secret_number.chars().collect();
    // // for c in sec_cs {
    // //     println!("{}", c.to_string());
    // // }
    // println!("{}", secret_number);
    
    // // prepareing game components
    
    // // let mut hit: u32 = 0;
    // // let mut blow: u32 = 0;

    // // game loop
    // loop {
    //     // read imput and conver to chars
    //     let mut guess = String::new();
    //     io::stdin()
    //         .read_line(&mut guess)
    //         .expect("Failed to read line");

    //     // check if guessed input can be treated as number
    //     let guess: u32 = match guess.trim().parse() {
    //         Ok(num) => num,
    //         Err(_) => continue,
    //     };

    //     // convert to chars (padding by 0 if needed)
    //     let guess = format!("{:04}", guess);
    //     let guess_cs: Vec<char> = guess.chars().collect();

    //     for c in &guess_cs {
    //         println!("{}", c.to_string());
    //     }
    //     println!("{}", guess);

    //     counting blow
    //     let mut blow: u32 = 0;
    //     let mut counts: HashMap<char, u32> = HashMap::new();
    //     for c in &guess_cs {
    //         let count = counts.entry(*c).or_insert(0); // kono * nani? mada rikai sitenai
    //         *count += 1;
    //     }

    //     for c in &secret_cs {
    //         println!("{}", c.to_string());
    //         match counts.get(c) {
    //             Some(res) => {
    //                 let &mut res_borrowing = res;
    //                 if *res_borrowing > 0 {
    //                     blow += 1;
    //                     *res_borrowing -= 1;
    //                 }
    //             },
    //             None => (),
    //         }
    //     }
        
    //     // counting hit        
    //     let iter_hit = guess_cs.iter().zip(secret_cs.iter());

    //     let mut hit: u32 = 0;
    //     for (guess_c, sec_c) in iter_hit {
    //         if guess_c == sec_c {
    //             hit += 1;
    //         }
    //     }
    
        
    //     println!("hit: {}, blow: {}", hit, blow - hit);
    //     break;
    // }
    
}
