use rand::thread_rng;
use rand::seq::SliceRandom;
use std::fs;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use crate::models::card::Card;
use crate::models::dir::Directory;
pub mod models;


fn load_cards(dir_name: &str, cards: &mut Vec<Card>) {
    let paths = fs::read_dir(dir_name).unwrap();

    // Get filenames in each directory
    for path in paths {
        // Ignore .DS_Store
        let p = path.unwrap();
        let fname = p.file_name();
        if fname == ".DS_Store" {
            println!("Note: Found .DS_Store, skipping");
            continue;
        }

        let metadata = p.metadata().unwrap();
        if metadata.is_dir() {
            continue;
        }

        let mut file = File::open(p.path()).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let card = Card {
            title: String::from(fname.to_str().unwrap().trim_end_matches(".md")),
            body: contents,
        };
        cards.push(card);
    }
}


fn load_directories(dir_name: &str, dirs: &mut Vec<Directory>) {
    let paths = fs::read_dir(dir_name).unwrap();

    for path in paths {
        let p = path.unwrap();
        let metadata = p.metadata().unwrap();
        if metadata.is_dir() {
            let dir = Directory {
                file_name: String::from(p.file_name().to_str().unwrap()),
                path: String::from(p.path().to_str().unwrap()),
            };
            dirs.push(dir);
        }
    }
}


fn main() {
    let mut cards : Vec<Card> = Vec::new();
    let mut dirs : Vec<Directory> = Vec::new();

    load_directories("./samples", &mut dirs);

    if dirs.len() == 0 {
        println!("No directories in given path");
        return;
    }

    let mut count = 1;
    let mut input = String::new();
    for dir in dirs.iter() {
        println!("{}: {}", count, dir.file_name);
        count += 1;
    }
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input : usize = input.trim().parse().expect("Number needed");

    if input < 1 || input > dirs.len() {
        println!("Invalid entry");
        return;
    }

    let dir = dirs.get(input-1).unwrap();
    
    // Load cards from chosen directory
    load_cards(&dir.path, &mut cards);
    println!("Loaded {} cards", cards.len());
    cards.shuffle(&mut thread_rng());

    for card in cards.iter() {
        println!("---\n{}\n---", card.title);
        println!("Easy: 1, Normal: 2, Tough: 3");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess : u32 = guess.trim().parse().expect("Number needed");
        println!("Answer: {}", card.body);
    }
}
