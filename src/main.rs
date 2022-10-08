use rand::thread_rng;
use rand::seq::SliceRandom;
use std::fs;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use crate::models::card::Card;
pub mod models;


fn load_cards(dir_name: &str, cards: &mut Vec<Card>) {
    let paths = fs::read_dir(dir_name).unwrap();

    // Get filenames in each directory
    for path in paths {
        // Ignore .DS_Store
        let fname = path.unwrap().file_name();
        if fname == ".DS_Store" {
            println!("Note: Found .DS_Store, skipping");
            continue;
        }

        let mut file = File::open(&fname).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let card = Card {
            title: String::from(fname.to_str().unwrap().trim_end_matches(".md")),
            body: contents,
        };
        cards.push(card);
    }
}


fn main() {
    let mut cards : Vec<Card> = Vec::new();

    load_cards("../samples", &mut cards);
    println!("Loaded {} cards", cards.len());
    cards.shuffle(&mut thread_rng());

    for card in cards.iter() {
        println!("---\n{}\n---", card.title);
        println!("Easy: 1, Normal: 2, Tough: 3");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess : u32 = guess.trim().parse().expect("Number needed");

        println!("Answer: {}", card.body);
        println!("You entered {}", guess);
    }
}
